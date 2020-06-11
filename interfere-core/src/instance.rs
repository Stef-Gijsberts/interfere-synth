use super::{AudioGenerator, DependentDeriver, IndependentUpdater};

use crate::values::{DGlobalRow, DVoicesMatrix, IGlobalRow, IVoicesMatrix};
use crate::Parameter;

pub struct Instance {
    independent_updater: IndependentUpdater,
    dependent_deriver: DependentDeriver,
    audio_generator: AudioGenerator,
    dglobal: DGlobalRow,
    dvoices: DVoicesMatrix,
    iglobal: IGlobalRow,
    ivoices: IVoicesMatrix,
}

impl Default for Instance {
    fn default() -> Instance {
        Instance {
            dglobal: DGlobalRow::zeros(),
            dvoices: DVoicesMatrix::zeros(),
            iglobal: IGlobalRow::zeros(),
            ivoices: IVoicesMatrix::zeros(),
            independent_updater: Default::default(),
            dependent_deriver: Default::default(),
            audio_generator: Default::default(),
        }
    }
}

impl Instance {
    pub fn process_midi_event(&mut self, midi_event: [u8; 3]) {
        self.independent_updater.process_midi_event(midi_event);
    }

    pub fn update_parameters(&mut self, updates: impl Iterator<Item = (Parameter, f64)>) {
        self.dependent_deriver.update_parameters(updates);
    }

    pub fn audio_requested(&mut self, buffer: &mut [(f64, f64)], samplerate_in_hz: f64) {
        self.independent_updater
            .advance_time_s((1.0 / samplerate_in_hz) * buffer.len() as f64);

        self.independent_updater
            .values_requested(&mut self.iglobal, &mut self.ivoices);
        self.dependent_deriver.values_requested(
            &mut self.dglobal,
            &mut self.dvoices,
            &self.iglobal,
            &self.ivoices,
        );

        self.audio_generator.audio_requested(
            buffer,
            samplerate_in_hz,
            &self.dglobal,
            &self.dvoices,
        );
    }
}
