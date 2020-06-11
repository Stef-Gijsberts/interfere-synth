mod envelope;
mod lfo;
mod midi_processor;
mod independent_updater;

use self::midi_processor::MidiProcessor;
use self::envelope::Envelope;
use self::lfo::LFO;

pub use self::independent_updater::IndependentUpdater;
