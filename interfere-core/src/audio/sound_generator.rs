use super::{Filter, Mixer, Oscillator};

pub struct SoundGenerator {
    Oscillator: Oscillator,
    filter: Filter,
    mixer: Mixer,
}
