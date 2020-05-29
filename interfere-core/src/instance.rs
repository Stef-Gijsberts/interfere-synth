use crate::values::{WeightGlobalGlobal, WeightGlobalVoice, WeightVoiceVoice, GlobalIndependents, VoiceIndependents, GlobalDependents, VoiceDependents, WeightsGlobalGlobal, WeightsGlobalVoice, WeightsVoiceVoice};

pub struct Instance {
    global_independents: GlobalIndependents,
    voice_independents: VoiceIndependents,
    pub weights_global_global: WeightsGlobalGlobal,
    pub weights_global_voice: WeightsGlobalVoice,
    pub weights_voice_voice: WeightsVoiceVoice,
    global_dependents: GlobalDependents,
    voice_dependents: VoiceDependents,
}

impl Default for Instance {
    fn default() -> Instance {
        let mut weights_global_global = WeightsGlobalGlobal::zeros();
        let mut weights_global_voice = WeightsGlobalVoice::zeros();
        let mut weights_voice_voice = WeightsVoiceVoice::zeros();

        // TODO: set sane defaults for weights

        Instance {
            global_independents: GlobalIndependents::zeros(),
            voice_independents: VoiceIndependents::zeros(),
            weights_global_global,
            weights_global_voice,
            weights_voice_voice,
            global_dependents: GlobalDependents::zeros(),
            voice_dependents: VoiceDependents::zeros(),
            voices: Default::default(),
        }
    }
}

impl Instance {
    pub fn new(wgg: WeightsGlobalGlobal, wgv: WeightsGlobalVoice, wvv: WeightsVoiceVoice) -> Instance {
        Instance {
            weights_global_global: wgg,
            weights_global_voice: wgv,
            weights_voice_voice: wvv,
            ..Default::default()
        }
    }

    pub fn audio_requested(&mut self, buffer: &mut [(f64, f64)], samplerate_hz: f64) {
        self.global_dependents = self.global_independents * self.weights_global_global;

        buffer.iter_mut().for_each(|(l, r)| {*l = 0.0; *r = 0.0;});
    }
}