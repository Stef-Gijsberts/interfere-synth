use crate::values::{DVoicesMatrix, DVoice, DVoices};
use crate::util::frequency_from_pitch;


// Based on https://www.quora.com/Whats-the-C-coding-for-a-low-pass-filter and
// https://en.wikipedia.org/wiki/Low-pass_filter#Simple_infinite_impulse_response_filter


#[derive(Default)]
pub struct Filter {
    previous: [f64; 16]
}


impl Filter {
     pub fn audio_requested(
        &mut self,
        voices_dependents: &DVoicesMatrix,
        buffer: &mut [[f64; 16]],
        samplerate_in_hz: f64,
    ) {

        use std::f64::consts::PI;

        let cutoff_hz = voices_dependents[DVoices(0, DVoice::FilterFrequency)] * 10000.0;

        let rc = 1.0 / (cutoff_hz*2.0*PI); 
        let dt = 1.0 / samplerate_in_hz; 
        let alpha = dt/(rc+dt);

        for frame in buffer {
            for (current, previous) in frame.iter_mut().zip(self.previous.iter_mut()) {
                *current = *previous + (alpha * (*current - *previous));
                *previous = *current;
            }
        }
    }
}