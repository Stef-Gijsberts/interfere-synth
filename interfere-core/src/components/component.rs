pub trait Component {
    fn audio_requested(&mut self, buffer: &[(f64, f64)], samplerate_hz: f64);
}
