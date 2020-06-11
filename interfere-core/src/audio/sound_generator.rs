use super::{Filter, Mixer, Oscillator};


#[derive(Default)]
pub struct SoundGenerator {
    Oscillator: Oscillator,
    filter: Filter,
    mixer: Mixer,
}
