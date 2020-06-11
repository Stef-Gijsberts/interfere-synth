use super::{Envelope, MidiProcessor, LFO, Pitch};

use crate::values::{IGlobal, IGlobalRow, IVoicesMatrix, DGlobalRow};

#[derive(Clone, Copy)]
pub struct Voice {
    pub note_pitch: u8,
    pub time_started_s: f64,
    pub time_released_s: Option<f64>,
}

#[derive(Default)]
pub struct IndependentUpdater {
    midi_processor: MidiProcessor,
    pitch: Pitch,
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

    pub fn values_requested(&mut self, iglobal: &mut IGlobalRow, ivoices: &mut IVoicesMatrix, dglobal: &DGlobalRow) {
        iglobal[IGlobal::One] = 1.0;

        self.envelope.values_requested(iglobal, ivoices, &mut self.voices, dglobal);
        self.pitch.values_requested(ivoices, self.voices);
        self.lfo.values_requested(iglobal, dglobal);
    }
}
