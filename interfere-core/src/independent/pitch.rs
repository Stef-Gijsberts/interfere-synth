use crate::values::{IGlobalRow, IVoice, IVoicesMatrix};

use super::Voice;

#[derive(Default)]
pub struct Pitch;

impl Pitch {
    pub fn values_requested(
        &self,
        ivoices: &mut IVoicesMatrix,
        voices: [Option<Voice>; 16],
    ) {
        // TODO: remove all this and actually implement it
        for (mut row, maybe_voice) in ivoices.row_iter_mut().zip(voices.iter()) {
            if let Some(voice) = maybe_voice {
                row[IVoice::Pitch as usize] = voice.note_pitch as f64;
            }
        }
    }
}
