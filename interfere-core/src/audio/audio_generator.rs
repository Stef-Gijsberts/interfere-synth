use crate::values::{DGlobalRow, DVoicesMatrix};

use super::voice::{Filter, Oscillator};

pub struct AudioGenerator {
    oscillator: Oscillator,
    filter: Filter,
    voicebuffer: [[f64; 16]; 1000],
    idx_head_voicebuffer: usize,
}

impl Default for AudioGenerator {
    fn default() -> AudioGenerator {
        let voicebuffer = [[0.0; 16]; 1000];
        let idx_head_voicebuffer = voicebuffer.len();

        AudioGenerator {
            idx_head_voicebuffer,
            voicebuffer,
            oscillator: Default::default(),
            filter: Default::default(),
        }
    }
}

impl AudioGenerator {
    pub fn audio_requested(
        &mut self,
        buffer: &mut [(f64, f64)],
        samplerate_in_hz: f64,
        dglobal: &DGlobalRow,
        dvoices: &DVoicesMatrix,
    ) {
        for (l, r) in buffer {
            let audio_available = self.voicebuffer.len() > self.idx_head_voicebuffer;

            if !audio_available {
                self.oscillator.voices_audio_requested(
                    &mut self.voicebuffer,
                    samplerate_in_hz,
                    dvoices,
                );
                self.filter.voices_audio_requested(
                    &mut self.voicebuffer,
                    samplerate_in_hz,
                    dvoices,
                );

                self.idx_head_voicebuffer = 0;
            }

            *l = self.voicebuffer[self.idx_head_voicebuffer]
                .iter()
                .sum::<f64>()
                / 16.0; // TODO: mix per voice?
            *r = *l;

            self.idx_head_voicebuffer += 1;
        }
    }
}
