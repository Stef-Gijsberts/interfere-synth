use crate::values::{DVoice, DVoices, DVoicesMatrix};

#[derive(Default)]
pub struct Oscillator {
    phases_in_rad: [f64; 16],
    current_pitches_in_tones: [f64; 16],
    current_volumes_in_0: [f64; 16],
}

impl Oscillator {
    pub fn voices_audio_requested(
        &mut self,
        buffer: &mut [[f64; 16]],
        samplerate_in_hz: f64,
        dvoices: &DVoicesMatrix,
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
                .for_each(|(p_in_rad, f_in_hz)| *p_in_rad += 2.0 * PI * f_in_hz / samplerate_in_hz);

            let values = self
                .phases_in_rad
                .iter()
                .copied()
                // .map(|phase| {
                //     if phase % 2.0 * PI > PI {
                //         phase - 2.0 * PI
                //     }
                //     else {
                //         phase
                //     }
                // })
                .map(f64::sin)
                // .map(|x| {
                //     if x > 0.0 {
                //         1.0
                //     }
                //     else {
                //         -1.0
                //     }
                // })
                .zip(self.current_volumes_in_0.iter())
                .map(|(sample, volume)| sample * volume);

            frame.iter_mut().zip(values).for_each(|(d, s)| *d = s);

            for idx in 0..16 {
                if self.phases_in_rad[idx] >= (2.0 * PI) {
                    self.current_pitches_in_tones[idx] = dvoices[DVoices(idx, DVoice::OscPitch)];
                    self.current_volumes_in_0[idx] = dvoices[DVoices(idx, DVoice::OscVolume)];
                    self.phases_in_rad[idx] %= 2.0 * PI;
                }
            }
        }
    }
}

// Modified from https://github.com/RustAudio/vst-rs/blob/master/examples/sine_synth.rs
pub fn frequency_from_pitch(pitch: f64) -> f64 {
    const A4_PITCH: f64 = 69.0;
    const A4_FREQ: f64 = 440.0;

    // Midi notes can be 0-127
    ((pitch - A4_PITCH) / 12.0).exp2() * A4_FREQ
}
