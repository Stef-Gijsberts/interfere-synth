use super::{MidiProcessor, Envelope, LFO};

pub struct IndependentUpdater {
    midi_processor: MidiProcessor,
    envelope: Envelope,
    lfo: LFO,
}
