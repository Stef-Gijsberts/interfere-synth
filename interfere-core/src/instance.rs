use crate::values::{
    GlobalDependents, GlobalIndependent, GlobalIndependents, VoiceDependent, VoiceIndependent,
    VoicesDependents, VoicesIndependent, VoicesIndependents, WeightGlobalVoice, WeightVoiceVoice,
    WeightsGlobalGlobal, WeightsGlobalVoice, WeightsVoiceVoice,
};
use crate::Oscillator;

pub struct Instance {
    global_independents: GlobalIndependents,
    voices_independents: VoicesIndependents,
    pub weights_global_global: WeightsGlobalGlobal,
    pub weights_global_voice: WeightsGlobalVoice,
    pub weights_voice_voice: WeightsVoiceVoice,
    global_dependents: GlobalDependents,
    voices_dependents: VoicesDependents,
    oscillator: Oscillator,
    voices: [Option<Voice>; 16],
}

#[derive(Clone, Copy)]
struct Voice {
    pub note_pitch: u8,
    pub t_samples: usize,
}

impl Default for Instance {
    fn default() -> Instance {
        let mut global_independents = GlobalIndependents::zeros();
        let voices_independents = VoicesIndependents::zeros();

        let weights_global_global = WeightsGlobalGlobal::zeros();
        let weights_global_voice = WeightsGlobalVoice::zeros();
        let mut weights_voice_voice = WeightsVoiceVoice::zeros();

        global_independents
            .row_mut(GlobalIndependent::One as usize)
            .fill(1.0);

        weights_voice_voice[WeightVoiceVoice(VoiceIndependent::Pitch, VoiceDependent::OscPitch)] = 1.0;
        weights_voice_voice[WeightVoiceVoice(VoiceIndependent::Envelope1, VoiceDependent::OscVolume)] = 1.0;
        // TODO: set more sane defaults

        Instance {
            global_independents,
            voices_independents,
            weights_global_global,
            weights_global_voice,
            weights_voice_voice,
            global_dependents: GlobalDependents::zeros(),
            voices_dependents: VoicesDependents::zeros(),
            oscillator: Default::default(),
            voices: [None; 16],
        }
    }
}

impl Instance {
    pub fn new(
        wgg: WeightsGlobalGlobal,
        wgv: WeightsGlobalVoice,
        wvv: WeightsVoiceVoice,
    ) -> Instance {
        Instance {
            weights_global_global: wgg,
            weights_global_voice: wgv,
            weights_voice_voice: wvv,
            ..Default::default()
        }
    }

    pub fn audio_requested(&mut self, buffer: &mut [(f64, f64)], samplerate_in_hz: f64) {
        self.recalculate_dependents();

        buffer.iter_mut().for_each(|(l, r)| {
            *l = 0.0;
            *r = 0.0;
        });

        self.oscillator
            .audio_requested(&self.voices_dependents, buffer, samplerate_in_hz);
    }

    fn recalculate_dependents(&mut self) {
        self.global_dependents = self.global_independents * self.weights_global_global;

        let voices_common_part = self.global_independents * self.weights_global_voice;

        self.voices_dependents = self.voices_independents * self.weights_voice_voice;
        self.voices_dependents
            .row_iter_mut()
            .for_each(|mut row| row += voices_common_part);
    }

    pub fn process_midi_event(&mut self, data: [u8; 3]) {
        // source: http://www.midimountain.com/midi/midi_status.htm
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8) {
        for idx in 0..self.voices.len() {
            if self.voices[idx].is_some() {
                continue;
            }

            self.voices[idx] = Some(Voice {
                t_samples: 0,
                note_pitch: note
            });

            self.voices_independents[VoicesIndependent(idx, VoiceIndependent::Pitch)] = note as f64;
            self.voices_independents[VoicesIndependent(idx, VoiceIndependent::Envelope1)] = 1.0;

            return;
        }
    }

    fn note_off(&mut self, note: u8) {
        for idx in 0..self.voices.len() {
            if let Some(voice) = &self.voices[idx] {
                self.voices_independents[VoicesIndependent(idx, VoiceIndependent::Envelope1)] = 0.0;

                if voice.note_pitch == note {
                    self.voices[idx] = None;
                }
            }
        }
    }

    pub fn update_parameters(&mut self, updates: impl Iterator<Item = (VoiceDependent, f64)>) {
        for (idx, new_value) in updates {
            self.weights_global_voice[WeightGlobalVoice(GlobalIndependent::One, idx)] = new_value;
        }
    }
}
