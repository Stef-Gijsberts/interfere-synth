use crate::values::IGlobalRow;

#[derive(Default)]
pub struct LFO {
    time_s: f64,
}

impl LFO {
    pub fn set_time_s(&mut self, time_s: f64) {
        self.time_s = time_s;
    }

    pub fn values_requested(&self, iglobal: &mut IGlobalRow) {
        // TODO
    }
}
