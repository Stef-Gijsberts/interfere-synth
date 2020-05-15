use super::Component;

pub struct Oscillator {
    pub phase_in_samples: u64,
    pub volume_in_0: f64,
    pub frequency_in_hz: f64,
}

impl Component for Oscillator {
    fn audio_requested(&mut self, buffer: &mut [(f64, f64)], samplerate_in_hz: f64) {
        use std::f64::consts::PI;

        for (sample_l, sample_r) in buffer {
            let phase_in_rad =
                 (2.0 * PI) * self.phase_in_samples as f64 * self.frequency_in_hz / samplerate_in_hz;

            let val = f64::sin(phase_in_rad) * self.volume_in_0;

            *sample_l = val;
            *sample_r = val;

            self.phase_in_samples += 1;
        }
    }
}

