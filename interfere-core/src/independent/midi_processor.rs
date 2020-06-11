use super::Voice;

#[derive(Default)]
pub struct MidiProcessor {
    time_s: f64,
}

impl MidiProcessor {
    pub fn set_time_s(&mut self, time_s: f64) {
        self.time_s = time_s;
    }

    pub fn process(&mut self, voices: &mut [Option<Voice>; 16], data: [u8; 3]) {
        // source: http://www.midimountain.com/midi/midi_status.htm
        let note = data[1];

        match data[0] {
            144 => self.note_on(voices, note),
            128 => self.note_off(voices, note),
            _ => (),
        }
    }

    fn note_on(&mut self, voices: &mut [Option<Voice>; 16], note: u8) {
        for maybe_voice in voices {
            if maybe_voice.is_none() {

                *maybe_voice = Some(Voice {
                    time_started_s: self.time_s,
                    note_pitch: note
                });

                return;
            }
        }
    }

    fn note_off(&mut self, voices: &mut [Option<Voice>; 16], note: u8) {
        for maybe_voice in voices {
            if let Some(voice) = maybe_voice {
                if voice.note_pitch == note {
                    *maybe_voice = None;
                }
            }
        }
    }
}