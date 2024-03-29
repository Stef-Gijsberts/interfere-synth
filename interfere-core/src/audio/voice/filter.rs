use crate::values::{DVoice, DVoices, DVoicesMatrix};

// Based on https://www.quora.com/Whats-the-C-coding-for-a-low-pass-filter and
// https://en.wikipedia.org/wiki/Low-pass_filter#Simple_infinite_impulse_response_filter

#[derive(Default)]
pub struct Filter {
    previous: [f64; 16],
}

impl Filter {
    pub fn voices_audio_requested(
        &mut self,
        buffer: &mut [[f64; 16]],
        samplerate_in_hz: f64,
        dvoices: &DVoicesMatrix,
    ) {
        use std::f64::consts::PI;

        let mut alphas: [f64; 16] = [0.0; 16];

        let cutoffs_hz = (0..16).map(|i| 30.0 + (dvoices[DVoices(i, DVoice::FilterFrequency)] * 12.0).exp() * 10.0);

        let rcs = cutoffs_hz.map(|cutoff_hz| 1.0 / (cutoff_hz * 2.0 * PI));
        let dt = 1.0 / samplerate_in_hz;

        for (alpha, rc) in alphas.iter_mut().zip(rcs) {
            *alpha = dt / (rc + dt);
        }

        for frame in buffer {
            for ((current, previous), alpha) in frame.iter_mut().zip(self.previous.iter_mut()).zip(alphas.iter()) {
                *current = *previous + (alpha * (*current - *previous));
                *previous = *current;
            }
        }
    }
}
