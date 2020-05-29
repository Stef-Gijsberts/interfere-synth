use crate::values::{VoiceDependent, VoicesDependent, VoicesDependents};

pub struct Oscillator {
    phases_in_samples: [usize; 16],
    current_pitches_in_tones: [f64; 16],
    current_volumes_in_0: [f64; 16],
}

impl Default for Oscillator {
    fn default() -> Oscillator {
        Oscillator {
            phases_in_samples: [0; 16],
            current_pitches_in_tones: [0.0; 16],
            current_volumes_in_0: [0.0; 16],
        }
    }
}

impl Oscillator {
    pub fn audio_requested(
        &mut self,
        voices_dependents: &VoicesDependents,
        buffer: &mut [(f64, f64)],
        samplerate_in_hz: f64,
    ) {
        use std::f64::consts::PI;

        for (sample_l, sample_r) in buffer {
            let mut phases_in_rad: [f64; 16] = [0.0; 16];

            let frequencies_in_hz = self
                .current_pitches_in_tones
                .iter()
                .copied()
                .map(frequency_from_pitch);

            phases_in_rad
                .iter_mut()
                .zip(self.phases_in_samples.iter().copied())
                .zip(frequencies_in_hz)
                .for_each(|((p_in_rad, p_in_samples), f_in_hz)| {
                    *p_in_rad = 2.0 * PI * p_in_samples as f64 * f_in_hz / samplerate_in_hz
                });

            let values = phases_in_rad
                .iter()
                .copied()
                .map(f64::sin)
                .zip(self.current_volumes_in_0.iter())
                .map(|(sample, volume)| sample * volume);

            let value = values.sum();

            *sample_l = value;
            *sample_r = value;

            self.phases_in_samples.iter_mut().for_each(|p| *p += 1);

            for idx in 0..16 {
                if phases_in_rad[idx] >= (2.0 * PI) {
                    self.phases_in_samples[idx] = 0;
                    self.current_pitches_in_tones[idx] =
                        voices_dependents[VoicesDependent(idx, VoiceDependent::OscPitch)];
                }
            }
        }
    }
}

// Found at https://github.com/RustAudio/vst-rs/blob/master/examples/sine_synth.rs
fn frequency_from_pitch(pitch: f64) -> f64 {
    const A4_PITCH: f64 = 69.0;
    const A4_FREQ: f64 = 440.0;

    // Midi notes can be 0-127
    ((pitch - A4_PITCH) / 12.0).exp2() * A4_FREQ
}
