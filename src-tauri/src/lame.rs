//! MP3 encoding through libmp3lame, linked dynamically.
//!
//! LAME is LGPL. The library lives as `libmp3lame.dylib` in `Contents/Frameworks` (built by
//! `scripts/baue-lame.sh`, source tarball next to it), so it can be replaced, and these few
//! declarations of LAME's public C interface are the app's own code: no LGPL code is compiled
//! into the executable. Settings and call order are fixed, because limiter and re-measurement
//! in `master.rs` are calibrated on them (CBR 192 kbit/s, quality 2, LAME 3.100).

use std::ffi::{c_char, c_float, c_int, c_uchar, c_void, CStr, CString};
use std::ptr::{self, NonNull};

#[link(name = "mp3lame", kind = "dylib")]
extern "C" {
    fn lame_init() -> *mut c_void;
    fn lame_close(gfp: *mut c_void) -> c_int;
    fn lame_set_num_channels(gfp: *mut c_void, channels: c_int) -> c_int;
    fn lame_set_in_samplerate(gfp: *mut c_void, rate: c_int) -> c_int;
    fn lame_set_VBR(gfp: *mut c_void, mode: c_int) -> c_int;
    fn lame_set_brate(gfp: *mut c_void, kbps: c_int) -> c_int;
    fn lame_set_quality(gfp: *mut c_void, quality: c_int) -> c_int;
    fn lame_set_mode(gfp: *mut c_void, mode: c_int) -> c_int;
    fn lame_set_bWriteVbrTag(gfp: *mut c_void, on: c_int) -> c_int;
    fn lame_init_params(gfp: *mut c_void) -> c_int;
    fn id3tag_init(gfp: *mut c_void);
    fn id3tag_add_v2(gfp: *mut c_void);
    fn id3tag_set_title(gfp: *mut c_void, title: *const c_char);
    fn id3tag_set_comment(gfp: *mut c_void, comment: *const c_char);
    fn lame_encode_buffer_ieee_float(gfp: *mut c_void, left: *const c_float, right: *const c_float, samples: c_int, out: *mut c_uchar, out_len: c_int) -> c_int;
    fn lame_encode_buffer_interleaved_ieee_float(gfp: *mut c_void, pcm: *const c_float, samples: c_int, out: *mut c_uchar, out_len: c_int) -> c_int;
    fn lame_encode_flush(gfp: *mut c_void, out: *mut c_uchar, out_len: c_int) -> c_int;
    fn get_lame_version() -> *const c_char;
}

const VBR_OFF: c_int = 0;
const JOINT_STEREO: c_int = 1;
const MONO: c_int = 3;
const KBPS: c_int = 192;
const QUALITY_NEAR_BEST: c_int = 2;
/// LAME keeps at most this much of an ID3 text field here.
const TAG_MAX: usize = 250;

/// Version string of the loaded library, e.g. "3.100".
pub fn version() -> String {
    // SAFETY: LAME returns a pointer to a static, NUL-terminated string.
    unsafe { CStr::from_ptr(get_lame_version()) }.to_string_lossy().into_owned()
}

/// What went wrong, without LAME's numeric codes: the caller shows a translated message.
#[derive(Debug)]
pub enum Error {
    Unavailable,
    Setting,
    Encoding,
}

pub struct Encoder {
    gfp: NonNull<c_void>,
    channels: usize,
}

fn tag(text: &str) -> CString {
    let bytes: Vec<u8> = text.bytes().filter(|b| *b != 0).take(TAG_MAX).collect();
    CString::new(bytes).unwrap_or_default()
}

impl Encoder {
    /// CBR 192 kbit/s, quality 2, mono or joint stereo, ID3v2 with title and comment, no Xing frame.
    pub fn new(channels: usize, sample_rate: u32, title: &str, comment: &str) -> Result<Self, Error> {
        // SAFETY: plain C calls on the handle LAME just returned; it is closed in `Drop`.
        unsafe {
            let enc = Encoder { gfp: NonNull::new(lame_init()).ok_or(Error::Unavailable)?, channels };
            let g = enc.gfp.as_ptr();
            let ok = |code: c_int| if code < 0 { Err(Error::Setting) } else { Ok(()) };
            ok(lame_set_num_channels(g, channels as c_int))?;
            ok(lame_set_in_samplerate(g, c_int::try_from(sample_rate).map_err(|_| Error::Setting)?))?;
            ok(lame_set_VBR(g, VBR_OFF))?;
            ok(lame_set_brate(g, KBPS))?;
            ok(lame_set_quality(g, QUALITY_NEAR_BEST))?;
            ok(lame_set_mode(g, if channels == 1 { MONO } else { JOINT_STEREO }))?;
            ok(lame_set_bWriteVbrTag(g, 0))?;
            if !title.is_empty() || !comment.is_empty() {
                id3tag_init(g);
                id3tag_add_v2(g);
                if !title.is_empty() {
                    id3tag_set_title(g, tag(title).as_ptr());
                }
                if !comment.is_empty() {
                    id3tag_set_comment(g, tag(comment).as_ptr());
                }
            }
            ok(lame_init_params(g))?;
            Ok(enc)
        }
    }

    /// Encodes interleaved (or mono) samples and replaces the contents of `out` with the MP3 bytes.
    pub fn encode(&mut self, pcm: &[f32], out: &mut Vec<u8>) -> Result<(), Error> {
        let frames = pcm.len() / self.channels;
        out.clear();
        out.reserve(frames * 5 / 4 + 7200); // LAME's documented worst case
        let cap = c_int::try_from(out.capacity()).unwrap_or(c_int::MAX);
        // SAFETY: `out` has `cap` writable bytes; LAME reports how many it initialised.
        let n = unsafe {
            if self.channels == 1 {
                lame_encode_buffer_ieee_float(self.gfp.as_ptr(), pcm.as_ptr(), ptr::null(), frames as c_int, out.as_mut_ptr(), cap)
            } else {
                lame_encode_buffer_interleaved_ieee_float(self.gfp.as_ptr(), pcm.as_ptr(), frames as c_int, out.as_mut_ptr(), cap)
            }
        };
        if n < 0 {
            return Err(Error::Encoding);
        }
        // SAFETY: see above.
        unsafe { out.set_len(n as usize) };
        Ok(())
    }

    /// Flushes the last frames (with padding, so the file ends cleanly) into `out`.
    pub fn flush(&mut self, out: &mut Vec<u8>) -> Result<(), Error> {
        out.clear();
        out.reserve(7200);
        let cap = c_int::try_from(out.capacity()).unwrap_or(c_int::MAX);
        // SAFETY: as in `encode`.
        let n = unsafe { lame_encode_flush(self.gfp.as_ptr(), out.as_mut_ptr(), cap) };
        if n < 0 {
            return Err(Error::Encoding);
        }
        // SAFETY: as in `encode`.
        unsafe { out.set_len(n as usize) };
        Ok(())
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        // SAFETY: the handle is valid and not used afterwards.
        unsafe { lame_close(self.gfp.as_ptr()) };
    }
}

// SAFETY: the handle is only ever used by the thread that owns the encoder.
unsafe impl Send for Encoder {}

/// For tests of the decoder: a CBR 96 MP3 with LAME tag (encoder delay and padding for gapless
/// decoding), which the app itself never writes.
#[cfg(test)]
pub fn encode_with_lame_tag(sample_rate: u32, channels: usize, pcm: &[f32]) -> Vec<u8> {
    extern "C" {
        fn lame_get_lametag_frame(gfp: *const c_void, buffer: *mut c_uchar, size: usize) -> usize;
    }
    // SAFETY: as in `Encoder::new`.
    let mut enc = unsafe {
        let enc = Encoder { gfp: NonNull::new(lame_init()).unwrap(), channels };
        let g = enc.gfp.as_ptr();
        lame_set_num_channels(g, channels as c_int);
        lame_set_in_samplerate(g, sample_rate as c_int);
        lame_set_brate(g, 96);
        lame_set_quality(g, 0);
        lame_set_bWriteVbrTag(g, 1);
        assert!(lame_init_params(g) >= 0);
        enc
    };
    let (mut out, mut part) = (Vec::new(), Vec::new());
    for block in pcm.chunks(channels * 16_384) {
        enc.encode(block, &mut part).unwrap();
        out.extend_from_slice(&part);
    }
    enc.flush(&mut part).unwrap();
    out.extend_from_slice(&part);
    // SAFETY: first call asks for the size, second fills a buffer of that size.
    unsafe {
        let size = lame_get_lametag_frame(enc.gfp.as_ptr(), ptr::null_mut(), 0);
        let mut tag = vec![0u8; size];
        lame_get_lametag_frame(enc.gfp.as_ptr(), tag.as_mut_ptr(), size);
        out[..size].copy_from_slice(&tag);
    }
    out
}
