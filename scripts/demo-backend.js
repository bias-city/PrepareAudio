/* Nachgebautes Backend für die Oberfläche im Browser (Bildschirmfotos, Sichtprüfung).
   Wird VOR ui/app.js geladen und stellt window.__TAURI__ bereit. Die Daten in
   scripts/demo-data/ stammen aus der echten Analyse des erfundenen Materials von
   scripts/demo-material.py (Test dump_ui). Aufruf: scripts/screenshots.mjs setzt
   window.PA_DEMO = { scan, plan, peaks, master } und lädt danach diese Datei. */
(() => {
  const D = window.PA_DEMO;
  const listeners = new Map();
  const emit = (name, payload) => (listeners.get(name) || []).forEach((f) => f({ payload }));
  const clipsUpdate = (clips) => ({ clips, items: D.plan.items, edited: false });
  const handlers = {
    scan_paths: () => D.scan,
    analyze_tracks: () => D.plan,
    analyze_master: () => D.master,
    pick_folders: () => ['/Users/demo/Gespraech/aufnahmen'],
    pick_output_dir: ({ sub }) => `/Users/demo/Gespraech/${sub}`,
    sync_set_clips: ({ clips }) => clipsUpdate(clips),
    sync_reset_clips: () => clipsUpdate(D.plan.clips),
    sync_peaks: ({ track, t0, t1, buckets }) => {
      const all = D.peaks[track] || [];
      const dur = D.plan.tracks[track].duration;
      const n = Math.max(1, Math.min(20000, buckets));
      return Array.from({ length: n }, (_, i) => {
        const a = Math.max(0, Math.floor(((t0 + (i * (t1 - t0)) / n) / dur) * all.length));
        const b = Math.min(all.length, Math.ceil(((t0 + ((i + 1) * (t1 - t0)) / n) / dur) * all.length));
        let m = 0;
        for (let k = a; k < b; k++) if (all[k] > m) m = all[k];
        return m;
      });
    },
  };
  window.__TAURI__ = {
    core: { invoke: async (name, args) => (handlers[name] ? handlers[name](args || {}) : null) },
    event: { listen: async (name, f) => { listeners.set(name, [...(listeners.get(name) || []), f]); return () => {}; } },
  };
  window.PA_DEMO_EMIT = emit;
})();
