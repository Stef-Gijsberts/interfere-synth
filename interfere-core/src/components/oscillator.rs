use super::Component;

pub struct Oscillator {
    pub phase_in_samples: u64,
    pub volume_in_0: f64,
    pub pitch_in_tones: f64,
}

impl Component for Oscillator {
    fn audio_requested(&mut self, buffer: &mut [(f64, f64)], samplerate_in_hz: f64) {
        use std::f64::consts::PI;

        let frequency_in_hz = freq_from_pitch(self.pitch_in_tones);

        for (sample_l, sample_r) in buffer {
            let phase_in_rad = (2.0 * PI) * self.phase_in_samples as f64 * frequency_in_hz / samplerate_in_hz;

            let val = f64::sin(phase_in_rad) * self.volume_in_0;

            *sample_l = val;
            *sample_r = val;

            self.phase_in_samples += 1;
        }
    }
}


// Found at https://github.com/RustAudio/vst-rs/blob/master/examples/sine_synth.rs
fn freq_from_pitch(pitch: f64) -> f64 {
    const A4_PITCH: f64 = 69.0;
    const A4_FREQ: f64 = 440.0;

    // Midi notes can be 0-127
    ((pitch - A4_PITCH) / 12.0).exp2() * A4_FREQ
}