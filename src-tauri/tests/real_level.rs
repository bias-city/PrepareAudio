//! Masters one real recording with both profiles into a folder of your choice and prints what
//! changed (not run by default; never writes next to the sources):
//!   PA_LEVEL_IN=/path/file.wav PA_LEVEL_OUT=/tmp/out \
//!     cargo test --release --test real_level -- --ignored --nocapture
use prepare_audio_lib::{level::Profile, master};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

#[test]
#[ignore]
fn master_one_recording_with_both_profiles() {
    let input = PathBuf::from(std::env::var("PA_LEVEL_IN").expect("PA_LEVEL_IN"));
    let out = PathBuf::from(std::env::var("PA_LEVEL_OUT").expect("PA_LEVEL_OUT"));
    let cancel = AtomicBool::new(false);
    let plan = master::analyze(&[input.clone()], &cancel, &mut |_| {}).unwrap();
    let f = &plan.files[0];
    println!("{} · {} · {:.1} s", f.name, f.format, f.duration);
    if let Some(l) = &f.loudness {
        println!("  gemessen {:.1} LUFS · TP {:.1} dBTP · LRA {:.1} LU", l.lufs, l.true_peak, l.lra);
    }
    for (profile, sub) in [(Profile::Documentary, "dokumentarisch"), (Profile::Leveler, "hoerfassung")] {
        let dir = out.join(sub);
        let t = std::time::Instant::now();
        let sum = master::write(&plan, &[f.id], Path::new(&dir), profile, &cancel, |_| {}).unwrap();
        let o = &sum.outcomes[0];
        let path = o.path.as_ref().expect("path");
        let Some(r) = o.result.as_ref() else {
            println!("  {sub}: schon vorhanden, nicht neu geschrieben\n    {path}");
            continue;
        };
        let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        println!("  {sub}: {:.2} LUFS · TP {:.2} dBTP · LRA {:.1} LU · {} MB · {:.1?}", r.lufs, r.true_peak, r.lra, size / 1_000_000, t.elapsed());
        println!("    {path}");
    }
}
