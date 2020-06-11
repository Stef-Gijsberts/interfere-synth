use crate::values::*;

pub struct DependentDeriver {
    w_global_global: WGlobalGlobalMatrix,
    w_global_voice: WGlobalVoiceMatrix,
    w_voice_voice: WVoiceVoiceMatrix,
}

impl Default for DependentDeriver {
    fn default() -> DependentDeriver {
        let w_global_global = WGlobalGlobalMatrix::zeros();
        let w_global_voice = WGlobalVoiceMatrix::zeros();
        let mut w_voice_voice = WVoiceVoiceMatrix::zeros();

        w_voice_voice[WVoiceVoice(IVoice::Pitch, DVoice::OscPitch)] = 1.0;
        w_voice_voice[WVoiceVoice(IVoice::Envelope1, DVoice::OscVolume)] = 1.0;
        w_voice_voice[WVoiceVoice(IVoice::Envelope1, DVoice::FilterFrequency)] = 1.0;

        DependentDeriver {
            w_global_global,
            w_global_voice,
            w_voice_voice,
        }
    }
}

impl DependentDeriver {
    pub fn update_parameters(&mut self, updates: impl Iterator<Item = (Parameter, f64)>) {
        for (param, new_value) in updates {
            match param {
                Parameter::Global(idx) => self.w_global_global[WGlobalGlobal(IGlobal::One, idx)] = new_value,
                Parameter::Voice(idx) => self.w_global_voice[WGlobalVoice(IGlobal::One, idx)] = new_value,
            }
        }
    }

    pub fn values_requested(
        &self,
        dglobal: &mut DGlobalRow,
        dvoices: &mut DVoicesMatrix,
        iglobal: &IGlobalRow,
        ivoices: &IVoicesMatrix,
    ) {
        *dglobal = iglobal * self.w_global_global;

        let voices_common_part = iglobal * self.w_global_voice;

        *dvoices = ivoices * self.w_voice_voice;

        dvoices
            .row_iter_mut()
            .for_each(|mut row| row += voices_common_part);
    }
}
