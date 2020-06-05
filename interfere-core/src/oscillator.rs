use crate::values::{DVoicesMatrix, DVoice, DVoices};
use crate::util::frequency_from_pitch;

pub struct Oscillator {
    phases_in_rad: [f64; 16],
    current_pitches_in_tones: [f64; 16],
    current_volumes_in_0: [f64; 16],
}

impl Default for Oscillator {
    fn default() -> Oscillator {
        Oscillator {
            phases_in_rad: [0.0; 16],
            current_pitches_in_tones: [0.0; 16],
            current_volumes_in_0: [0.0; 16],
        }
    }
}

impl Oscillator {
    pub fn audio_requested(
        &mut self,
        voices_dependents: &DVoicesMatrix,
        buffer: &mut [[f64; 16]],
        samplerate_in_hz: f64,
    ) {
        use std::f64::consts::PI;

        for frame in buffer {
            let frequencies_in_hz = self
                .current_pitches_in_tones
                .iter()
                .copied()
                .map(frequency_from_pitch);

            self.phases_in_rad
                .iter_mut()
                .zip(frequencies_in_hz)
                .for_each(|(p_in_rad, f_in_hz)| {
                    *p_in_rad += 2.0 * PI as f64 * f_in_hz / samplerate_in_hz
                });

            let values = self.phases_in_rad
                .iter()
                .copied()
                .map(f64::sin)
                .zip(self.current_volumes_in_0.iter())
                .map(|(sample, volume)| sample * volume);

            frame.iter_mut().zip(values).for_each(|(d, s)| *d = s);

            for idx in 0..16 {
                if self.phases_in_rad[idx] >= (2.0 * PI) {
                    self.current_pitches_in_tones[idx] = voices_dependents[DVoices(idx, DVoice::OscPitch)];
                    self.current_volumes_in_0[idx] = voices_dependents[DVoices(idx, DVoice::OscVolume)];
                    self.phases_in_rad[idx] %= 2.0 * PI;
                }
            }
        }
    }
}
