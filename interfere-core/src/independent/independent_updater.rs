use super::{MidiProcessor, Envelope, LFO};

use crate::values::{IGlobalRow, IVoicesMatrix, IGlobal};


#[derive(Clone, Copy)]
pub struct Voice {
    pub note_pitch: u8,
    pub time_started_s: f64,
}

#[derive(Default)]
pub struct IndependentUpdater {
    midi_processor: MidiProcessor,
    envelope: Envelope,
    lfo: LFO,
    voices: [Option<Voice>; 16],
    time_s: f64,
}

impl IndependentUpdater {
    pub fn process_midi_event(&mut self, midi_event: [u8; 3]) {
        self.midi_processor.process(&mut self.voices, midi_event);
    }

    pub fn advance_time_s(&mut self, dt_s: f64) {
        self.time_s += dt_s;

        self.midi_processor.set_time_s(self.time_s);
        self.envelope.set_time_s(self.time_s);
        self.lfo.set_time_s(self.time_s);
    }

    pub fn values_requested(&self, iglobal: &mut IGlobalRow, ivoices: &mut IVoicesMatrix) {
        iglobal[IGlobal::One] = 1.0;

        self.envelope.values_requested(iglobal, ivoices, self.voices);
        self.lfo.values_requested(iglobal);
    }
}
