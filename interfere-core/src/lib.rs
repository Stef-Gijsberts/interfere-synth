mod audio;
mod independent;
mod dependent;
mod values;
mod instance;

use self::dependent::DependentDeriver;
use self::independent::IndependentUpdater;
use self::audio::SoundGenerator;

pub use self::instance::Instance;
pub use self::values::DVoice as Parameter;