use std::f32::consts::PI;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::core::w;
use windows::Win32::Media::Audio::{PlaySoundW, SND_ALIAS, SND_MEMORY, SND_NODEFAULT, SND_SYNC};

static CHIME_PLAYING: AtomicBool = AtomicBool::new(false);

struct ChimeGuard;

impl Drop for ChimeGuard {
    fn drop(&mut self) {
        CHIME_PLAYING.store(false, Ordering::Release);
    }
}

/// Generates a gentle, pleasant dual-tone soft chime in memory as a WAV buffer.
/// Tone 1: 523 Hz (C5) transitioning into 784 Hz (G5) with smooth exponential decay.
/// Sound is warm, subtle, and calming (never startling).
fn generate_soft_chime_wav() -> Vec<u8> {
    let sample_rate = 44100u32;
    let duration_secs = 0.65f32;
    let total_samples = (sample_rate as f32 * duration_secs) as usize;

    let mut pcm_data = Vec::with_capacity(total_samples * 2);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;

        // Exponential decay envelope
        let envelope = (-4.5 * t).exp();

        // Dual harmonic soft bell: fundamental (C5 / G5 mix) + soft harmonics
        let freq1 = 523.25; // C5
        let freq2 = 783.99; // G5

        let wave = 0.6 * (2.0 * PI * freq1 * t).sin()
            + 0.4 * (2.0 * PI * freq2 * t).sin()
            + 0.15 * (4.0 * PI * freq1 * t).sin();

        let sample = (wave * envelope * 0.45 * 32767.0) as i16;
        pcm_data.extend_from_slice(&sample.to_le_bytes());
    }

    let mut wav = Vec::with_capacity(44 + pcm_data.len());
    let byte_rate = sample_rate * 2; // 16-bit mono
    let data_len = pcm_data.len() as u32;

    // RIFF Header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_len).to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    // "fmt " subchunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // subchunk1 size (16 for PCM)
    wav.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat (1 = PCM)
    wav.extend_from_slice(&1u16.to_le_bytes()); // NumChannels (1 = mono)
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes()); // BlockAlign (2 bytes)
    wav.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample (16 bits)

    // "data" subchunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    wav.extend_from_slice(&pcm_data);

    wav
}

/// Copies a WAV byte slice into a 2-byte-aligned `Vec<u16>` buffer so passing its pointer to
/// `PlaySoundW(..., SND_MEMORY | SND_SYNC)` is strictly aligned and outlives synchronous playback.
fn wav_bytes_to_aligned_u16(bytes: &[u8]) -> Vec<u16> {
    let mut out = Vec::with_capacity(bytes.len().div_ceil(2));
    for chunk in bytes.chunks(2) {
        let lo = chunk[0];
        let hi = *chunk.get(1).unwrap_or(&0);
        out.push(u16::from_le_bytes([lo, hi]));
    }
    out
}

/// Synchronously plays the gentle chime on the calling thread while keeping `wav_data` alive.
/// Returns `true` if `PlaySoundW` succeeded (or skipped safely because another chime is playing).
pub fn play_chime_sync() -> bool {
    if CHIME_PLAYING
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return false;
    }
    let _guard = ChimeGuard;

    let wav_bytes = generate_soft_chime_wav();
    let wav_aligned = wav_bytes_to_aligned_u16(&wav_bytes);
    unsafe {
        let res = PlaySoundW(
            windows::core::PCWSTR(wav_aligned.as_ptr()),
            None,
            SND_MEMORY | SND_SYNC | SND_NODEFAULT,
        );
        if res.as_bool() {
            return true;
        }
        // Fallback to standard gentle Windows notification alias synchronously
        PlaySoundW(
            w!("SystemNotification"),
            None,
            SND_ALIAS | SND_SYNC | SND_NODEFAULT,
        )
        .as_bool()
    }
}

pub fn play_gentle_chime() {
    std::thread::spawn(|| {
        let _ = play_chime_sync();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_chime_wav() {
        let wav = generate_soft_chime_wav();
        assert!(wav.len() > 44);
        assert_eq!(wav.len() % 2, 0);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[12..16], b"fmt ");
        assert_eq!(&wav[36..40], b"data");

        let riff_size = u32::from_le_bytes([wav[4], wav[5], wav[6], wav[7]]) as usize;
        let data_size = u32::from_le_bytes([wav[40], wav[41], wav[42], wav[43]]) as usize;
        assert_eq!(riff_size + 8, wav.len());
        assert_eq!(data_size + 44, wav.len());

        let aligned = wav_bytes_to_aligned_u16(&wav);
        assert_eq!(aligned.len() * 2, wav.len());
        let roundtrip: Vec<u8> = aligned.iter().flat_map(|w| w.to_le_bytes()).collect();
        assert_eq!(roundtrip, wav);
    }

    #[test]
    fn test_play_chime_sync_completes_and_releases_guard() {
        let _played = play_chime_sync();
        assert!(
            !CHIME_PLAYING.load(Ordering::Acquire),
            "Chime playback guard must be released after play_chime_sync returns"
        );
    }
}
