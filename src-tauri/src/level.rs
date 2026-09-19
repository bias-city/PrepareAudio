//! Speech leveller and stereo mixdown for step 3 (profile "Hörfassung").
//!
//! An offline tool can look at the whole recording first. One pass measures every channel in
//! 10-ms hops (after an 80-Hz high-pass), finds where somebody speaks and, in files with one
//! speaker per channel, who is the active one. From that come gain curves: each speaker is
//! brought to the same level, the gain rides the short-term level of the speech (±12 dB, held in
//! pauses, smoothed forwards and backwards so it moves before the jump), and a channel that only
//! carries the bleed of another speaker is turned down gently. The render pass applies the curves,
//! places every segment left, middle or right, and runs a light bus compressor. Normalisation to
//! the target loudness, the limiter and the MP3 encoder follow in `master.rs` as before.
//!
//! The profile "Dokumentarisch" uses none of this: gains stay 1, only the mixdown of polyphonic
//! files by their positions remains (mono and plain stereo files pass through untouched).

use crate::decode::AudioReader;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

const HOP_S: f64 = 0.010;
const HIGHPASS_HZ: f64 = 80.0;
const RIDE_MAX_DB: f32 = 12.0;
const RIDE_WINDOW_S: f64 = 3.0;
const RIDE_MIN_SPEECH_S: f64 = 0.3;
const RIDE_SMOOTH_S: f64 = 1.0;
const STATIC_MAX_DB: f32 = 15.0;
const DOMINANCE_DB: f32 = 6.0;
const DUCK_DB: f32 = -12.0;
const DUCK_SMOOTH_S: f64 = 0.12;
const MIN_SPEAKER_S: f64 = 5.0;
const COMP_OVER_SPEECH_DB: f32 = 6.0;
const COMP_RATIO: f32 = 2.0;
const COMP_ATTACK_S: f64 = 0.010;
const COMP_RELEASE_S: f64 = 0.150;
/// Left and right are clearly apart but still comfortable on headphones: about 80 % of the
/// power on the own side. The hard separation stays in the polyphonic WAV.
const PAN_NEAR: f32 = 0.894;
const PAN_FAR: f32 = 0.447;
const PAN_MID: f32 = std::f32::consts::FRAC_1_SQRT_2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    /// Speech leveller with stereo mixdown (default).
    #[default]
    Leveler,
    /// Fixed gain only, no intervention in the dynamics.
    Documentary,
}

impl Profile {
    pub fn code(self) -> &'static str {
        match self {
            Profile::Leveler => "leveler",
            Profile::Documentary => "documentary",
        }
    }
}

fn pan_gains(pos: char) -> (f32, f32) {
    match pos {
        'L' => (PAN_NEAR, PAN_FAR),
        'R' => (PAN_FAR, PAN_NEAR),
        _ => (PAN_MID, PAN_MID),
    }
}

/// What a file is for the mixdown.
#[derive(Debug, Clone, PartialEq)]
enum Layout {
    /// One speaker per channel with positions per segment (a shared file of step 2).
    Speakers,
    /// Mono stays mono; stereo keeps its image, both channels share one gain curve.
    Plain,
}

/// Result of the analysis pass: everything the render pass needs.
#[derive(Debug, Clone)]
pub struct Prep {
    rate: u32,
    hop: usize,
    ch_in: usize,
    pub out_ch: usize,
    layout: Layout,
    level: bool,
    /// Linear gain per channel and hop (Plain: one curve, index 0).
    gains: Vec<Vec<f32>>,
    /// (left, right) per channel and hop; only for `Speakers`.
    pans: Vec<Vec<(f32, f32)>>,
    /// Linear level above which the bus compressor works (0 = off).
    comp_threshold: f32,
    /// For the UI and tests: static gain per channel in dB, share of time each channel is the active speaker.
    pub static_db: Vec<f32>,
    pub active_share: Vec<f32>,
}

#[derive(Clone)]
struct Highpass {
    b: [f64; 3],
    a: [f64; 2],
    z: [f64; 2],
}

impl Highpass {
    fn new(rate: f64, f0: f64) -> Self {
        let w = std::f64::consts::TAU * f0 / rate;
        let (sn, cs) = w.sin_cos();
        let alpha = sn / (2.0 * std::f64::consts::FRAC_1_SQRT_2);
        let a0 = 1.0 + alpha;
        Highpass { b: [(1.0 + cs) / 2.0 / a0, -(1.0 + cs) / a0, (1.0 + cs) / 2.0 / a0], a: [-2.0 * cs / a0, (1.0 - alpha) / a0], z: [0.0; 2] }
    }
    #[inline]
    fn run(&mut self, x: f32) -> f32 {
        let x = x as f64;
        let y = self.b[0] * x + self.z[0];
        self.z[0] = self.b[1] * x - self.a[0] * y + self.z[1];
        self.z[1] = self.b[2] * x - self.a[1] * y;
        y as f32
    }
}

fn db(p: f32) -> f32 {
    10.0 * (p + 1e-12).log10()
}

fn percentile(v: &[f32], q: f64) -> f32 {
    if v.is_empty() {
        return -120.0;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.total_cmp(b));
    s[((s.len() - 1) as f64 * q).round() as usize]
}

/// Zero-phase smoothing: a one-pole filter forwards, then backwards.
fn smooth(v: &mut [f32], tau_s: f64) {
    let a = (-HOP_S / tau_s).exp() as f32;
    for pass in 0..2 {
        let mut y = if pass == 0 { v.first().copied() } else { v.last().copied() }.unwrap_or(0.0);
        let n = v.len();
        for k in 0..n {
            let i = if pass == 0 { k } else { n - 1 - k };
            y = a * y + (1.0 - a) * v[i];
            v[i] = y;
        }
    }
}

/// A hangover so word ends and short gaps stay inside: 50 ms before, 200 ms after.
fn hangover(raw: &[bool]) -> Vec<bool> {
    let mut out = vec![false; raw.len()];
    let (before, after) = (5usize, 20usize);
    for (i, &on) in raw.iter().enumerate() {
        if on {
            for o in out[i.saturating_sub(before)..(i + after + 1).min(raw.len())].iter_mut() {
                *o = true;
            }
        }
    }
    out
}

/// Where somebody speaks: hops clearly above the channel's noise floor and not more than 35 dB
/// below its loud passages (a digital-silence floor must not make everything count as voice).
fn voice_activity(level_db: &[f32]) -> Option<Vec<bool>> {
    let (floor, top) = (percentile(level_db, 0.10), percentile(level_db, 0.95));
    if top - floor < 6.0 || top < -70.0 {
        return None; // no speech to go by (silence, tone, steady noise)
    }
    let thr = (floor + (0.35 * (top - floor)).max(8.0)).max(top - 35.0).max(-60.0);
    Some(level_db.iter().map(|&d| d > thr).collect())
}

/// Gain curve in dB that rides the short-term level of the active speech around `speech_db`.
fn ride(power: &[f32], active: &[bool], speech_db: f32) -> Vec<f32> {
    let n = power.len();
    let half = (RIDE_WINDOW_S / HOP_S / 2.0) as usize;
    let need = (RIDE_MIN_SPEECH_S / HOP_S) as usize;
    // prefix sums over active hops
    let (mut sum, mut cnt) = (vec![0f64; n + 1], vec![0usize; n + 1]);
    for i in 0..n {
        sum[i + 1] = sum[i] + if active[i] { power[i] as f64 } else { 0.0 };
        cnt[i + 1] = cnt[i] + active[i] as usize;
    }
    let mut curve = vec![f32::NAN; n];
    for i in 0..n {
        let (a, b) = (i.saturating_sub(half), (i + half + 1).min(n));
        let c = cnt[b] - cnt[a];
        if c >= need {
            let st = db(((sum[b] - sum[a]) / c as f64) as f32);
            curve[i] = (speech_db - st).clamp(-RIDE_MAX_DB, RIDE_MAX_DB);
        }
    }
    // hold the last value through pauses, fill the start from the first value
    let first = curve.iter().copied().find(|v| v.is_finite()).unwrap_or(0.0);
    let mut last = first;
    for v in curve.iter_mut() {
        if v.is_finite() {
            last = *v;
        } else {
            *v = last;
        }
    }
    smooth(&mut curve, RIDE_SMOOTH_S);
    curve
}

impl Prep {
    /// Plain pass-through description (profile "Dokumentarisch" for mono and stereo files).
    fn passthrough(rate: u32, ch_in: usize) -> Prep {
        Prep {
            rate,
            hop: ((rate as f64 * HOP_S) as usize).max(1),
            ch_in,
            out_ch: ch_in.clamp(1, 2),
            layout: Layout::Plain,
            level: false,
            gains: Vec::new(),
            pans: Vec::new(),
            comp_threshold: 0.0,
            static_db: Vec::new(),
            active_share: Vec::new(),
        }
    }

    /// Analysis pass. `pan_segments`: `(channel from 1, from s, to s, 'L'|'M'|'R')` from the iXML of a
    /// shared file; empty for any other file.
    pub(crate) fn analyze(
        reader: &mut dyn AudioReader,
        pan_segments: &[(usize, f64, f64, char)],
        profile: Profile,
        cancel: &AtomicBool,
        on_secs: &mut dyn FnMut(f64),
    ) -> Result<Prep, String> {
        let (ch_in, rate) = (reader.channels().max(1), reader.rate());
        let speakers = !pan_segments.is_empty() || ch_in > 2;
        let level = profile == Profile::Leveler;
        if !level && !speakers {
            return Ok(Prep::passthrough(rate, ch_in));
        }
        let hop = ((rate as f64 * HOP_S) as usize).max(1);
        let curves = if speakers { ch_in } else { 1 };

        // ---- pass: power per hop (after the high-pass), per curve
        let mut power: Vec<Vec<f32>> = vec![Vec::new(); curves];
        if level {
            let mut hp: Vec<Highpass> = (0..ch_in).map(|_| Highpass::new(rate as f64, HIGHPASS_HZ)).collect();
            let (mut acc, mut fill) = (vec![0f64; curves], 0usize);
            let (mut buf, mut frames) = (Vec::new(), 0u64);
            loop {
                buf.clear();
                let more = reader.read(&mut buf)?;
                if cancel.load(Ordering::Relaxed) {
                    return Err(crate::i18n::cancelled());
                }
                for frame in buf.chunks_exact(ch_in) {
                    for (c, &x) in frame.iter().enumerate() {
                        let y = hp[c].run(x) as f64;
                        acc[if speakers { c } else { 0 }] += y * y;
                    }
                    fill += 1;
                    if fill == hop {
                        let per = if speakers { hop } else { hop * ch_in };
                        for (c, a) in acc.iter_mut().enumerate() {
                            power[c].push((*a / per as f64) as f32);
                            *a = 0.0;
                        }
                        fill = 0;
                    }
                }
                frames += (buf.len() / ch_in) as u64;
                on_secs(frames as f64 / rate as f64);
                if !more {
                    break;
                }
            }
        }
        let hops = power.first().map_or(0, Vec::len);

        // ---- speech and active speaker
        let mut prep = Prep {
            rate,
            hop,
            ch_in,
            out_ch: if speakers { 2 } else { ch_in.clamp(1, 2) },
            layout: if speakers { Layout::Speakers } else { Layout::Plain },
            level,
            gains: vec![vec![1.0; hops]; curves],
            pans: Vec::new(),
            comp_threshold: 0.0,
            static_db: vec![0.0; curves],
            active_share: vec![0.0; curves],
        };
        if speakers {
            let default_pos = |c: usize| if ch_in < 2 { 'M' } else if c == 0 { 'L' } else if c + 1 == ch_in { 'R' } else { 'M' };
            prep.pans = (0..ch_in)
                .map(|c| {
                    let mut v = vec![pan_gains(default_pos(c)); hops.max(1)];
                    for &(ch, from, to, pos) in pan_segments.iter().filter(|s| s.0 == c + 1) {
                        let _ = ch;
                        let (a, b) = (((from / HOP_S) as usize).min(v.len()), ((to / HOP_S).ceil() as usize).min(v.len()));
                        v[a..b].iter_mut().for_each(|p| *p = pan_gains(pos));
                    }
                    v
                })
                .collect();
        }
        if !level || hops == 0 {
            return Ok(prep);
        }

        let level_db: Vec<Vec<f32>> = power.iter().map(|p| p.iter().map(|&x| db(x)).collect()).collect();
        let vad: Vec<Option<Vec<bool>>> = level_db.iter().map(|d| voice_activity(d)).collect();
        // First estimate of each speaker's level: all voiced hops.
        let speech_level = |c: usize, mask: &[bool]| -> Option<f32> {
            let (mut s, mut n) = (0f64, 0usize);
            for (i, &on) in mask.iter().enumerate() {
                if on {
                    s += power[c][i] as f64;
                    n += 1;
                }
            }
            (n as f64 * HOP_S >= MIN_SPEAKER_S).then(|| db((s / n as f64) as f32))
        };
        let rough: Vec<Option<f32>> = (0..curves).map(|c| vad[c].as_ref().and_then(|m| speech_level(c, m))).collect();
        // Active speaker: voiced, and relative to the own level within 6 dB of the strongest channel.
        // Judged on the hops that really carry voice (in the gaps between syllables the quieter
        // channel would "win" by its lower level); the hangover comes afterwards.
        let mut smoothed: Vec<Vec<f32>> = power.clone();
        smoothed.iter_mut().for_each(|p| smooth(p, 0.05));
        smoothed.iter_mut().for_each(|p| p.iter_mut().for_each(|x| *x = db(*x)));
        let active: Vec<Vec<bool>> = (0..curves)
            .map(|c| {
                let (Some(mask), Some(own)) = (vad[c].as_ref(), rough[c]) else { return vec![false; hops] };
                let raw: Vec<bool> = (0..hops)
                    .map(|i| {
                        if !mask[i] {
                            return false;
                        }
                        let best = (0..curves)
                            .filter_map(|k| match (&vad[k], rough[k]) {
                                (Some(m), Some(l)) if m[i] => Some(smoothed[k][i] - l),
                                _ => None,
                            })
                            .fold(f32::NEG_INFINITY, f32::max);
                        smoothed[c][i] - own >= best - DOMINANCE_DB
                    })
                    .collect();
                hangover(&raw)
            })
            .collect();
        if std::env::var_os("PA_LEVEL_DEBUG").is_some() {
            for c in 0..curves {
                let half = hops / 2;
                let share = |r: std::ops::Range<usize>| active[c][r.clone()].iter().filter(|&&a| a).count() as f32 / r.len() as f32;
                let voiced = vad[c].as_ref().map_or(0.0, |m| m.iter().filter(|&&a| a).count() as f32 / hops as f32);
                eprintln!("[level] ch{c}: rough {:?} voiced {voiced:.2} active first half {:.2} second half {:.2} p10 {:.1} p95 {:.1}", rough[c], share(0..half), share(half..hops), percentile(&level_db[c], 0.10), percentile(&level_db[c], 0.95));
            }
        }
        // The level that counts: where the speaker is the active one.
        let levels: Vec<Option<f32>> = (0..curves).map(|c| speech_level(c, &active[c])).collect();
        let reference = levels.iter().flatten().copied().fold(f32::NEG_INFINITY, f32::max);
        if !reference.is_finite() {
            return Ok(prep); // nothing that counts as speech: leave the file alone
        }
        for c in 0..curves {
            prep.active_share[c] = active[c].iter().filter(|&&a| a).count() as f32 / hops as f32;
            let Some(own) = levels[c] else { continue }; // ambience or bleed only: gain stays 1
            let fixed = (reference - own).clamp(0.0, STATIC_MAX_DB);
            prep.static_db[c] = fixed;
            let riding = ride(&power[c], &active[c], own);
            let mut duck = vec![0f32; hops];
            if curves > 1 {
                for i in 0..hops {
                    if !active[c][i] && (0..curves).any(|k| k != c && active[k][i]) {
                        duck[i] = DUCK_DB;
                    }
                }
                smooth(&mut duck, DUCK_SMOOTH_S);
            }
            for i in 0..hops {
                prep.gains[c][i] = 10f32.powf((fixed + riding[i] + duck[i]) / 20.0);
            }
        }
        prep.comp_threshold = 10f32.powf((reference + COMP_OVER_SPEECH_DB) / 20.0);
        Ok(prep)
    }

    /// A fresh render state for one pass over the file.
    pub fn mixer(&self) -> Mixer<'_> {
        Mixer {
            prep: self,
            hp: (0..self.ch_in).map(|_| Highpass::new(self.rate as f64, HIGHPASS_HZ)).collect(),
            frame: 0,
            env: 0.0,
            attack: (-1.0 / (COMP_ATTACK_S * self.rate as f64)).exp() as f32,
            release: (-1.0 / (COMP_RELEASE_S * self.rate as f64)).exp() as f32,
        }
    }
}

/// Applies the curves of a [`Prep`] frame by frame and mixes down to mono or stereo.
pub struct Mixer<'a> {
    prep: &'a Prep,
    hp: Vec<Highpass>,
    frame: u64,
    env: f32,
    attack: f32,
    release: f32,
}

impl Mixer<'_> {
    /// Gain of `curve` at the current frame, linear between the hop centres (no zipper noise).
    #[inline]
    fn gain(&self, curve: usize) -> f32 {
        let g = &self.prep.gains[curve];
        if g.is_empty() {
            return 1.0;
        }
        let pos = (self.frame as f64 / self.prep.hop as f64 - 0.5).max(0.0);
        let i = (pos as usize).min(g.len() - 1);
        let j = (i + 1).min(g.len() - 1);
        let f = (pos - i as f64) as f32;
        g[i] + (g[j] - g[i]) * f
    }

    /// One input frame in, one output frame (`out_ch` samples) out.
    #[inline]
    pub fn push(&mut self, frame: &[f32], out: &mut [f32; 2]) {
        let p = self.prep;
        match p.layout {
            Layout::Plain => {
                let g = if p.level { self.gain(0) } else { 1.0 };
                for c in 0..p.out_ch {
                    let x = frame[c];
                    out[c] = if p.level { self.hp[c].run(x) * g } else { x };
                }
            }
            Layout::Speakers => {
                let hop = ((self.frame / p.hop as u64) as usize).min(p.pans[0].len() - 1);
                let (mut l, mut r) = (0f32, 0f32);
                for c in 0..p.ch_in {
                    let x = if p.level { self.hp[c].run(frame[c]) * self.gain(c) } else { frame[c] };
                    let (gl, gr) = p.pans[c][hop];
                    l += x * gl;
                    r += x * gr;
                }
                *out = [l, r];
            }
        }
        if p.comp_threshold > 0.0 {
            let peak = if p.out_ch == 2 { out[0].abs().max(out[1].abs()) } else { out[0].abs() };
            let sq = peak * peak;
            let a = if sq > self.env { self.attack } else { self.release };
            self.env = a * self.env + (1.0 - a) * sq;
            let level = self.env.sqrt();
            if level > p.comp_threshold {
                // 2:1 above the threshold: half of the excess (in dB) is taken away
                let over_db = 20.0 * (level / p.comp_threshold).log10();
                let g = 10f32.powf(-over_db * (1.0 - 1.0 / COMP_RATIO) / 20.0);
                out[0] *= g;
                out[1] *= g;
            }
        }
        self.frame += 1;
    }
}
