use crate::components::{Component, Oscillator};
use crate::values::{IndependentValueIndex, DependentValueIndex, IndependentValueRow, DependentValueRow, ConnectionWeightMatrix, ConnectionValueIndex};

pub struct Instance {
    independents: IndependentValueRow,
    dependents: DependentValueRow,
    connections: ConnectionWeightMatrix,
    osc_a: Oscillator,
    osc_b: Oscillator,
}

impl Default for Instance {
    fn default() -> Instance {
        let mut independents = IndependentValueRow::zeros();
        let dependents = DependentValueRow::zeros();
        let mut connections = ConnectionWeightMatrix::zeros();

        // First independent is always one. This is a trick to get the
        // first row of the connections to be the base value.
        // 
        // (This trick is inspired by how the bias is often encoded within
        // the weight matrix in neural networks.)
        *independents.index_mut(IndependentValueIndex::One) = 1.0;

        *connections.index_mut(ConnectionValueIndex(IndependentValueIndex::Pitch, DependentValueIndex::OscAPitch)) = 1.0;
        *connections.index_mut(ConnectionValueIndex(IndependentValueIndex::Pitch, DependentValueIndex::OscBPitch)) = 1.0;

        Instance {
            independents,
            dependents,
            connections,
            osc_a: Oscillator {
                frequency_in_hz: 1_000.0,
                phase_in_samples: 0,
                volume_in_0: 0.0
            },
            osc_b: Oscillator {
                frequency_in_hz: 1_000.0,
                phase_in_samples: 0,
                volume_in_0: 0.0
            }
        }
    }
}

// Found at https://github.com/RustAudio/vst-rs/blob/master/examples/sine_synth.rs
fn midi_pitch_to_freq(pitch: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    // Midi notes can be 0-127
    ((f64::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

impl Instance {
    pub fn audio_requested(&mut self, buffer: &mut [(f64, f64)], samplerate_hz: f64) {
        self.dependents = self.independents * self.connections;

        self.osc_a.volume_in_0 = *self.dependents.index(DependentValueIndex::OscAVolume);
        self.osc_a.frequency_in_hz = *self.dependents.index(DependentValueIndex::OscAPitch);

        self.osc_b.volume_in_0 = *self.dependents.index(DependentValueIndex::OscBVolume);
        self.osc_b.frequency_in_hz = *self.dependents.index(DependentValueIndex::OscBPitch);

        // self.osc_a.audio_requested(buffer, samplerate_hz);
        self.osc_b.audio_requested(buffer, samplerate_hz);
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
        *self.independents.index_mut(IndependentValueIndex::Pitch) = midi_pitch_to_freq(note);
    }

    fn note_off(&mut self, note: u8) {
        // TODO
    }

    pub fn update_parameters(&mut self, updates: impl Iterator<Item=(DependentValueIndex, f64)>) {
        for (idx, new_value) in updates {
            *self.connections.index_mut(ConnectionValueIndex(IndependentValueIndex::One, idx)) = new_value;
        }
    }
}
