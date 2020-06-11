mod envelope;
mod lfo;
mod independent_updater;
mod midi_processor;

use self::envelope::Envelope;
use self::lfo::LFO;
use self::midi_processor::MidiProcessor;
use self::independent_updater::Voice;

pub use self::independent_updater::IndependentUpdater;