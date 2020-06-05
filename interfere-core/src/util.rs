// Found at https://github.com/RustAudio/vst-rs/blob/master/examples/sine_synth.rs
pub fn frequency_from_pitch(pitch: f64) -> f64 {
    const A4_PITCH: f64 = 69.0;
    const A4_FREQ: f64 = 440.0;

    // Midi notes can be 0-127
    ((pitch - A4_PITCH) / 12.0).exp2() * A4_FREQ
}
