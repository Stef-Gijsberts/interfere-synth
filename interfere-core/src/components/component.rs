pub trait Component {
    fn audio_requested(&mut self, buffer: &mut [(f64, f64)], samplerate_hz: f64);
}
