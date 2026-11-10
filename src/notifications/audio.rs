use std::f32::consts::PI;
use windows::core::w;
use windows::Win32::Media::Audio::{PlaySoundW, SND_ALIAS, SND_ASYNC, SND_MEMORY};

/// Generates a gentle, pleasant dual-tone soft chime in memory as a WAV buffer.
/// Tone 1: 523 Hz (C5) transitioning into 784 Hz (G5) with smooth exponential decay.
/// Sound is warm, subtle, and calming (never startling).
fn generate_soft_chime_wav() -> Vec<u8> {
    let sample_rate = 44100u32;
    let duration_secs = 0.65f32;
    let total_samples = (sample_rate as f32 * duration_secs) as usize;

    let mut pcm_data = Vec::with_capacity(total_samples);

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

    let mut wav = Vec::new();
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

pub fn play_gentle_chime() {
    std::thread::spawn(|| {
        let wav_data = generate_soft_chime_wav();
        unsafe {
            let res = PlaySoundW(
                windows::core::PCWSTR(wav_data.as_ptr() as *const u16),
                None,
                SND_MEMORY | SND_ASYNC,
            );
            if !res.as_bool() {
                // Fallback to standard gentle Windows notification alias
                let _ = PlaySoundW(w!("SystemNotification"), None, SND_ALIAS | SND_ASYNC);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_chime_wav() {
        let wav = generate_soft_chime_wav();
        assert!(wav.len() > 44);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
    }
}
