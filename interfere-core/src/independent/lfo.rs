use crate::values::{IGlobalRow, IGlobal, DGlobal, DGlobalRow};

#[derive(Default)]
pub struct LFO {
    time_s: f64,
    phase_in_rad: f64,
    last_time_in_s: f64,
}

impl LFO {
    pub fn set_time_s(&mut self, time_s: f64) {
        self.time_s = time_s;
    }

    pub fn values_requested(&mut self, iglobal: &mut IGlobalRow, dglobal: &DGlobalRow) {
        use std::f64::consts::PI;

        let frequency_in_hz = dglobal[DGlobal::LFOFrequency] * 10.0;

        let elapsed_time_in_s = self.time_s - self.last_time_in_s;
        self.last_time_in_s = self.time_s;

        self.phase_in_rad += elapsed_time_in_s * 2.0 * PI * frequency_in_hz;

        iglobal[IGlobal::LFO] = f64::sin(self.phase_in_rad);
    }
}
