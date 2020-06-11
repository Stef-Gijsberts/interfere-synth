use super::{IndependentUpdater, DependentDeriver, SoundGenerator};

#[derive(Default)]
pub struct Instance {
    independent_updater: IndependentUpdater,
    dependent_deriver: DependentDeriver,
    sound_generator: SoundGenerator,
}
