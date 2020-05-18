pub struct Instance {}

impl Default for Instance {
    fn default() -> Instance {
        Instance {}
    }
}

impl Instance {
    pub fn audio_requested(&mut self, buffer: &mut [(f64, f64)], samplerate_hz: f64) {
        unimplemented!() // TODO
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
        unimplemented!() // TODO
    }

    fn note_off(&mut self, note: u8) {
        unimplemented!() // TODO
    }
}
