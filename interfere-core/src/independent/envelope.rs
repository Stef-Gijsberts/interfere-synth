use crate::values::{IGlobalRow, IVoice, IVoicesMatrix};

use super::Voice;

#[derive(Default)]
pub struct Envelope {
    time_s: f64,
}

impl Envelope {
    pub fn set_time_s(&mut self, time_s: f64) {
        self.time_s = time_s;
    }

    pub fn values_requested(
        &self,
        iglobal: &mut IGlobalRow,
        ivoices: &mut IVoicesMatrix,
        voices: [Option<Voice>; 16],
    ) {
        // TODO: remove all this and actually implement it
        for (mut row, voice) in ivoices.row_iter_mut().zip(voices.iter()) {
            if voice.is_some() {
                row[IVoice::Envelope1 as usize] = 1.0;
            }
            else {
                row[IVoice::Envelope1 as usize] = 0.0;
            }
        }
    }
}
