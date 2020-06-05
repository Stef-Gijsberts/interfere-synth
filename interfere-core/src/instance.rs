use crate::values::*;
use crate::Oscillator;

pub struct Instance {
    global_independents: IGlobalRow,
    voices_independents: IVoicesMatrix,
    pub weights_global_global: WGlobalGlobalMatrix,
    pub weights_global_voice: WGlobalVoiceMatrix,
    pub weights_voice_voice: WVoiceVoiceMatrix,
    global_dependents: DGlobalRow,
    voices_dependents: DVoicesMatrix,
    oscillator: Oscillator,
    voices: [Option<Voice>; 16],
    voice_buffers: [[f64; 16]; 1000],
    idx_voice_buffers_head: usize,
}

#[derive(Clone, Copy)]
struct Voice {
    pub note_pitch: u8,
    pub t_samples: usize,
}

impl Default for Instance {
    fn default() -> Instance {
        let mut global_independents = IGlobalRow::zeros();
        let voices_independents = IVoicesMatrix::zeros();

        let weights_global_global = WGlobalGlobalMatrix::zeros();
        let weights_global_voice = WGlobalVoiceMatrix::zeros();
        let mut weights_voice_voice = WVoiceVoiceMatrix::zeros();

        global_independents
            .row_mut(IGlobal::One as usize)
            .fill(1.0);

        weights_voice_voice[WVoiceVoice(IVoice::Pitch, DVoice::OscPitch)] = 1.0;
        weights_voice_voice[WVoiceVoice(IVoice::Envelope1, DVoice::OscVolume)] = 1.0;
        // TODO: set more sane defaults

        let voice_buffers = [[0.0; 16]; 1000];

        Instance {
            global_independents,
            voices_independents,
            weights_global_global,
            weights_global_voice,
            weights_voice_voice,
            global_dependents: DGlobalRow::zeros(),
            voices_dependents: DVoicesMatrix::zeros(),
            oscillator: Default::default(),
            voices: [None; 16],
            voice_buffers,
            idx_voice_buffers_head: voice_buffers.len(),
        }
    }
}

impl Instance {
    pub fn new(
        wgg: WGlobalGlobalMatrix,
        wgv: WGlobalVoiceMatrix,
        wvv: WVoiceVoiceMatrix,
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
            let audio_available = self.voice_buffers.len() - self.idx_voice_buffers_head > 0;

            if !audio_available {
                self.oscillator.audio_requested(&self.voices_dependents, &mut self.voice_buffers[self.idx_voice_buffers_head..], samplerate_in_hz);
                self.idx_voice_buffers_head = 0;
            }

            *l = self.voice_buffers[self.idx_voice_buffers_head].iter().sum::<f64>() / 16.0;
            *r = *l;

            self.idx_voice_buffers_head += 1;
        });
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

            self.voices_independents[IVoices(idx, IVoice::Pitch)] = note as f64;
            self.voices_independents[IVoices(idx, IVoice::Envelope1)] = 1.0;

            return;
        }
    }

    fn note_off(&mut self, note: u8) {
        for idx in 0..self.voices.len() {
            if let Some(voice) = &self.voices[idx] {
                self.voices_independents[IVoices(idx, IVoice::Envelope1)] = 0.0;

                if voice.note_pitch == note {
                    self.voices[idx] = None;
                }
            }
        }
    }

    pub fn update_parameters(&mut self, updates: impl Iterator<Item = (DVoice, f64)>) {
        for (idx, new_value) in updates {
            self.weights_global_voice[WGlobalVoice(IGlobal::One, idx)] = new_value;
        }
    }
}
