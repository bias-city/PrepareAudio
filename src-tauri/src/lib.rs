pub mod decode;
pub mod i18n;
pub mod lame;
pub mod level;
pub mod master;
pub mod merge;
pub mod player;
pub mod scan;
pub mod sync;
pub mod wav;

#[cfg(test)]
mod testutil;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;
use i18n::{t, Msg};

#[derive(Default)]
struct AppState {
    scan: Mutex<Option<Arc<scan::Scan>>>,
    sync_plan: Mutex<Option<Arc<sync::SyncPlan>>>,
    master_plan: Mutex<Option<Arc<master::MasterPlan>>>,
    player: OnceLock<player::Player>,
    cancel: Arc<AtomicBool>,
    busy: Arc<AtomicBool>,
}

#[derive(Clone, Serialize)]
struct Beat {
    op: &'static str,
    elapsed_ms: u64,
}

/// Sends `work-heartbeat` every 0.5 s while alive, so the interface can show
/// that the backend really is still computing.
struct Heartbeat(Arc<AtomicBool>);

impl Heartbeat {
    fn start(app: &AppHandle, op: &'static str) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let (flag, app) = (stop.clone(), app.clone());
        std::thread::spawn(move || {
            let t = Instant::now();
            while !flag.load(Ordering::SeqCst) {
                let _ = app.emit("work-heartbeat", Beat { op, elapsed_ms: t.elapsed().as_millis() as u64 });
                std::thread::sleep(Duration::from_millis(500));
            }
        });
        Heartbeat(stop)
    }
}

impl Drop for Heartbeat {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

fn player<'a>(app: &AppHandle, state: &'a AppState) -> &'a player::Player {
    state.player.get_or_init(|| {
        let app = app.clone();
        player::Player::start(move |p| {
            let _ = app.emit("player-position", p);
        })
    })
}

#[tauri::command]
async fn scan_paths(app: AppHandle, state: State<'_, AppState>, paths: Vec<String>) -> Result<scan::Scan, String> {
    let _beat = Heartbeat::start(&app, "scan");
    let inputs: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();
    let emitter = app.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        let mut last = std::time::Instant::now();
        scan::scan_with(&inputs, &scan::Options::default(), &decode::cache_dir(), &std::sync::atomic::AtomicBool::new(false), &mut |done, total| {
            if last.elapsed() >= Duration::from_millis(150) {
                last = std::time::Instant::now();
                let _ = emitter.emit("scan-progress", serde_json::json!({ "done": done, "total": total }));
            }
        })
    })
        .await
        .map_err(|e| e.to_string())??;
    *state.scan.lock().map_err(|e| e.to_string())? = Some(Arc::new(result.clone()));
    Ok(result)
}

#[tauri::command]
async fn pick_folders(app: AppHandle, title: String) -> Result<Vec<String>, String> {
    let picked = tauri::async_runtime::spawn_blocking(move || app.dialog().file().set_title(title).blocking_pick_folders())
        .await
        .map_err(|e| e.to_string())?;
    Ok(picked
        .unwrap_or_default()
        .into_iter()
        .filter_map(|p| p.into_path().ok())
        .map(|p| p.to_string_lossy().into_owned())
        .collect())
}

#[tauri::command]
async fn merge_recordings(
    app: AppHandle,
    state: State<'_, AppState>,
    ids: Vec<usize>,
    out_dir: String,
) -> Result<merge::Summary, String> {
    let scan = state
        .scan
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or(t(Msg::ScanFirst))?;
    if ids.is_empty() {
        return Err(t(Msg::NoRecordingsSelected).into());
    }
    if out_dir.trim().is_empty() {
        return Err(t(Msg::NoOutDir).into());
    }
    if state.busy.swap(true, Ordering::SeqCst) {
        return Err(t(Msg::Busy).into());
    }
    let _beat = Heartbeat::start(&app, "merge");
    state.cancel.store(false, Ordering::SeqCst);
    let cancel = state.cancel.clone();
    let busy = state.busy.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        let recs: Vec<&scan::Recording> = scan.recordings.iter().filter(|r| ids.contains(&r.id)).collect();
        let mut last: Option<Instant> = None;
        merge::run(&recs, Path::new(&out_dir), &cancel, |p| {
            if p.milestone || last.map_or(true, |t| t.elapsed() >= Duration::from_millis(80)) {
                last = Some(Instant::now());
                let _ = app.emit("merge-progress", p);
            }
        })
    })
    .await;
    busy.store(false, Ordering::SeqCst);
    result.map_err(|e| e.to_string())?
}

/// Second function: find synchronous stretches between tracks of different recorders.
#[tauri::command]
async fn analyze_tracks(app: AppHandle, state: State<'_, AppState>, paths: Vec<String>) -> Result<sync::SyncPlan, String> {
    if state.busy.swap(true, Ordering::SeqCst) {
        return Err(t(Msg::Busy).into());
    }
    let _beat = Heartbeat::start(&app, "sync-analyze");
    player(&app, &state).pause();
    state.cancel.store(false, Ordering::SeqCst);
    let cancel = state.cancel.clone();
    let busy = state.busy.clone();
    let inputs: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();
    let events = app.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        let app = events;
        let mut last: Option<Instant> = None;
        sync::analyze(&inputs, &cancel, &mut |p| {
            if p.done >= p.total || last.map_or(true, |t| t.elapsed() >= Duration::from_millis(100)) {
                last = Some(Instant::now());
                let _ = app.emit("sync-progress", p);
            }
        })
    })
    .await;
    busy.store(false, Ordering::SeqCst);
    let plan = result.map_err(|e| e.to_string())??;
    player(&app, &state).set_mix(player::Mix::from_plan(&plan));
    *state.sync_plan.lock().map_err(|e| e.to_string())? = Some(Arc::new(plan.clone()));
    Ok(plan)
}

#[derive(Serialize)]
struct ClipsUpdate {
    clips: Vec<sync::Clip>,
    items: Vec<sync::Item>,
    edited: bool,
}

fn edit_plan(app: &AppHandle, state: &AppState, change: impl FnOnce(&mut sync::SyncPlan) -> Result<(), String>) -> Result<ClipsUpdate, String> {
    let mut guard = state.sync_plan.lock().map_err(|e| e.to_string())?;
    let arc = guard.as_mut().ok_or(t(Msg::AnalyzeTracksFirst))?;
    let plan = Arc::make_mut(arc);
    change(plan)?;
    player(app, state).set_mix(player::Mix::from_plan(plan));
    Ok(ClipsUpdate { clips: plan.clips.clone(), items: plan.items.clone(), edited: plan.edited })
}

/// Takes the edited clips of the timeline, returns the resulting output files.
#[tauri::command]
fn sync_set_clips(app: AppHandle, state: State<'_, AppState>, clips: Vec<sync::Clip>) -> Result<ClipsUpdate, String> {
    edit_plan(&app, &state, |plan| sync::apply_clips(plan, clips))
}

#[tauri::command]
fn sync_reset_clips(app: AppHandle, state: State<'_, AppState>) -> Result<ClipsUpdate, String> {
    edit_plan(&app, &state, sync::reset_clips)
}

#[tauri::command]
fn sync_peaks(state: State<'_, AppState>, track: usize, t0: f64, t1: f64, buckets: usize) -> Result<Vec<u8>, String> {
    let guard = state.sync_plan.lock().map_err(|e| e.to_string())?;
    let plan = guard.as_ref().ok_or(t(Msg::AnalyzeTracksFirst))?;
    Ok(sync::peaks(plan, track, t0, t1, buckets))
}

#[tauri::command]
fn player_play(app: AppHandle, state: State<'_, AppState>, t: f64) {
    player(&app, &state).play(t);
}

#[tauri::command]
fn player_pause(app: AppHandle, state: State<'_, AppState>) {
    player(&app, &state).pause();
}

#[tauri::command]
fn player_seek(app: AppHandle, state: State<'_, AppState>, t: f64) {
    player(&app, &state).seek(t);
}

#[tauri::command]
fn player_solo(app: AppHandle, state: State<'_, AppState>, muted: Vec<String>, solo: Vec<String>) {
    player(&app, &state).set_solo(muted, solo);
}

/// "Wo speichern?": picks a folder, starting next to the sources, and returns
/// the step's subfolder inside it (or the folder itself if it already has that name).
#[tauri::command]
async fn pick_output_dir(app: AppHandle, title: String, start: String, sub: String) -> Result<Option<String>, String> {
    let start_dir = PathBuf::from(&start);
    let picked = tauri::async_runtime::spawn_blocking(move || {
        let mut dialog = app.dialog().file().set_title(title);
        if start_dir.is_dir() {
            dialog = dialog.set_directory(&start_dir);
        }
        dialog.blocking_pick_folder()
    })
    .await
    .map_err(|e| e.to_string())?;
    Ok(picked.and_then(|p| p.into_path().ok()).map(|dir| {
        let name = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let out = if sub.is_empty() || name == sub { dir } else { dir.join(&sub) };
        out.to_string_lossy().into_owned()
    }))
}

#[tauri::command]
async fn write_sync(
    app: AppHandle,
    state: State<'_, AppState>,
    ids: Vec<usize>,
    out_dir: String,
) -> Result<merge::Summary, String> {
    let plan = state
        .sync_plan
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or(t(Msg::AnalyzeTracksFirst))?;
    if ids.is_empty() {
        return Err(t(Msg::NoSectionsSelected).into());
    }
    if out_dir.trim().is_empty() {
        return Err(t(Msg::NoOutDir).into());
    }
    if state.busy.swap(true, Ordering::SeqCst) {
        return Err(t(Msg::Busy).into());
    }
    let _beat = Heartbeat::start(&app, "sync-write");
    state.cancel.store(false, Ordering::SeqCst);
    let cancel = state.cancel.clone();
    let busy = state.busy.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        let mut last: Option<Instant> = None;
        sync::write(&plan, &ids, Path::new(&out_dir), &cancel, |p| {
            if p.milestone || last.map_or(true, |t| t.elapsed() >= Duration::from_millis(80)) {
                last = Some(Instant::now());
                let _ = app.emit("sync-write-progress", p);
            }
        })
    })
    .await;
    busy.store(false, Ordering::SeqCst);
    result.map_err(|e| e.to_string())?
}

/// Third function: measure loudness, master to -16 LUFS as MP3.
#[tauri::command]
async fn analyze_master(app: AppHandle, state: State<'_, AppState>, paths: Vec<String>) -> Result<master::MasterPlan, String> {
    if state.busy.swap(true, Ordering::SeqCst) {
        return Err(t(Msg::Busy).into());
    }
    let _beat = Heartbeat::start(&app, "master-analyze");
    state.cancel.store(false, Ordering::SeqCst);
    let cancel = state.cancel.clone();
    let busy = state.busy.clone();
    let inputs: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();
    let result = tauri::async_runtime::spawn_blocking(move || {
        let mut last: Option<Instant> = None;
        master::analyze(&inputs, &cancel, &mut |p| {
            if p.done >= p.total || last.map_or(true, |t| t.elapsed() >= Duration::from_millis(100)) {
                last = Some(Instant::now());
                let _ = app.emit("master-progress", p);
            }
        })
    })
    .await;
    busy.store(false, Ordering::SeqCst);
    let plan = result.map_err(|e| e.to_string())??;
    *state.master_plan.lock().map_err(|e| e.to_string())? = Some(Arc::new(plan.clone()));
    Ok(plan)
}

#[tauri::command]
async fn write_master(
    app: AppHandle,
    state: State<'_, AppState>,
    ids: Vec<usize>,
    out_dir: String,
    profile: Option<level::Profile>,
) -> Result<master::MasterSummary, String> {
    let plan = state
        .master_plan
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or(t(Msg::AnalyzeFilesFirst))?;
    if ids.is_empty() {
        return Err(t(Msg::NoFilesSelected).into());
    }
    if out_dir.trim().is_empty() {
        return Err(t(Msg::NoOutDir).into());
    }
    if state.busy.swap(true, Ordering::SeqCst) {
        return Err(t(Msg::Busy).into());
    }
    let _beat = Heartbeat::start(&app, "master-write");
    state.cancel.store(false, Ordering::SeqCst);
    let cancel = state.cancel.clone();
    let busy = state.busy.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        master::write(&plan, &ids, Path::new(&out_dir), profile.unwrap_or_default(), &cancel, |p| {
            let _ = app.emit("master-write-progress", p);
        })
    })
    .await;
    busy.store(false, Ordering::SeqCst);
    result.map_err(|e| e.to_string())?
}

/// Language of the backend's messages ("de", "en", "fr", "it"); the interface
/// calls this at startup and on every switch.
#[tauri::command]
fn set_language(lang: String) {
    i18n::set(&lang);
}

#[tauri::command]
fn cancel_merge(state: State<'_, AppState>) {
    state.cancel.store(true, Ordering::SeqCst);
}

/// Opens a folder, or with `select` reveals a file, in Finder / Explorer. Through the opener
/// plugin (NSWorkspace on macOS): starting `/usr/bin/open` as a child does not work reliably
/// inside the App Store sandbox.
#[tauri::command]
fn reveal(path: String, select: bool) -> Result<(), String> {
    if select {
        tauri_plugin_opener::reveal_item_in_dir(Path::new(&path)).map_err(|e| e.to_string())
    } else {
        tauri_plugin_opener::open_path(&path, None::<&str>).map_err(|e| e.to_string())
    }
}

/// Opens a web link from the info panel in the default browser (https only).
#[tauri::command]
fn open_link(url: String) -> Result<(), String> {
    if !url.starts_with("https://") || url.chars().any(|c| c.is_whitespace()) {
        return Err(t(Msg::InvalidLink).into());
    }
    tauri_plugin_opener::open_url(&url, None::<&str>).map_err(|e| e.to_string())
}

/// Distribution channel of this build: "mas" (Mac App Store, Cargo feature `mas`) or "dmg".
/// The info panel words the licence per channel.
#[tauri::command]
fn channel() -> &'static str {
    if cfg!(feature = "mas") {
        "mas"
    } else {
        "dmg"
    }
}

/// The menu, cut down to what this app really offers: no File, no View — it opens nothing and
/// prints nothing. English like the predefined items (About/Hide/Quit, Undo/Cut/Copy come from
/// the system in English); the menu therefore does not follow the interface language.
///
/// The About entry is our own instead of `PredefinedMenuItem::about`: the macOS panel shows only
/// name, version and credits — licence, website and the notices would be dropped, and links in it
/// could not be clicked. The item sends an event to the interface, which opens its own info panel.
fn menue(handle: &tauri::AppHandle) -> tauri::Result<tauri::menu::Menu<tauri::Wry>> {
    use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
    let app = Submenu::with_items(
        handle,
        "PrepareAudio",
        true,
        &[
            &MenuItem::with_id(handle, "ueber", "About PrepareAudio", true, None::<&str>)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::hide(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::quit(handle, None)?,
        ],
    )?;
    // Edit: the app has one text field (the licence search), but copying paths and messages out
    // of the lists is what people really need here.
    let text = Submenu::with_items(
        handle,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::cut(handle, None)?,
            &PredefinedMenuItem::copy(handle, None)?,
            &PredefinedMenuItem::paste(handle, None)?,
            &PredefinedMenuItem::select_all(handle, None)?,
        ],
    )?;
    let fenster = Submenu::with_items(
        handle,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(handle, None)?,
            &PredefinedMenuItem::fullscreen(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::close_window(handle, None)?,
        ],
    )?;
    // Help: the handbook in its own window (⌘⇧/ like everywhere on macOS). Older macOS refuses
    // the accelerator on this key — then the entry simply has none.
    let eintrag = MenuItem::with_id(handle, "handbuch", "PrepareAudio Handbook", true, Some("CmdOrCtrl+Shift+/"))
        .or_else(|_| MenuItem::with_id(handle, "handbuch", "PrepareAudio Handbook", true, None::<&str>))?;
    let hilfe = Submenu::with_items(handle, "Help", true, &[&eintrag])?;
    let menu = Menu::with_items(handle, &[&app, &text, &fenster, &hilfe])?;
    #[cfg(target_os = "macos")]
    let _ = hilfe.set_as_help_menu_for_nsapp();
    Ok(menu)
}

/// The handbook window (`ui/hilfe.html`), or focus it if it is already open.
fn hilfe_fenster(handle: &tauri::AppHandle) {
    use tauri::Manager;
    if let Some(w) = handle.get_webview_window("hilfe") {
        let _ = w.unminimize();
        let _ = w.set_focus();
        return;
    }
    let _ = tauri::WebviewWindowBuilder::new(handle, "hilfe", tauri::WebviewUrl::App("hilfe.html".into()))
        .title("PrepareAudio Handbook")
        .inner_size(980.0, 760.0)
        .min_inner_size(560.0, 420.0)
        .build();
}

/// Opens the handbook from the interface (the pill in the info panel).
#[tauri::command]
async fn handbuch_oeffnen(app: AppHandle) {
    let h = app.clone();
    let _ = app.run_on_main_thread(move || hilfe_fenster(&h));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .menu(menue)
        .on_menu_event(|handle, event| {
            if event.id() == "ueber" {
                let _ = handle.emit("ueber", ());
            } else if event.id() == "handbuch" {
                hilfe_fenster(handle);
            }
        })
        // The handbook window does not keep the app alive: it goes with the main window.
        .on_window_event(|fenster, ereignis| {
            use tauri::Manager;
            if fenster.label() == "main" {
                if let tauri::WindowEvent::Destroyed = ereignis {
                    if let Some(w) = fenster.app_handle().get_webview_window("hilfe") {
                        let _ = w.close();
                    }
                }
            }
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            scan_paths,
            pick_folders,
            merge_recordings,
            analyze_tracks,
            write_sync,
            analyze_master,
            write_master,
            sync_set_clips,
            sync_reset_clips,
            sync_peaks,
            player_play,
            player_pause,
            player_seek,
            player_solo,
            pick_output_dir,
            cancel_merge,
            set_language,
            reveal,
            open_link,
            channel,
            handbuch_oeffnen
        ])
        .run(tauri::generate_context!())
        .expect("PrepareAudio konnte nicht gestartet werden");
}
