use crate::values::{IGlobalRow, IVoicesMatrix};


#[derive(Default)]
pub struct LFO {
    time_s: f64
}


impl LFO {
    pub fn advance_time_s(&mut self, dt_s: f64) {
        self.time_s += dt_s;
    }

    pub fn values_requested(&self, iglobal: &mut IGlobalRow, ivoices: &mut IVoicesMatrix) {
        // TODO
    }
}