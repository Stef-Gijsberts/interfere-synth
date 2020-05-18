use nalgebra as na;

use num_traits::FromPrimitive;

use crate::values::{DependentValueIndex, NumDependentValues, Value};

pub struct Parameters {
    vec: na::VectorN<Value, NumDependentValues>,
}

impl Default for Parameters {
    fn default() -> Parameters {
        let mut vec = na::VectorN::<Value, NumDependentValues>::repeat(0.0);

        *vec.index_mut(DependentValueIndex::OscAVolume as usize) = 0.7;

        Parameters { vec }
    }
}

impl Parameters {
    pub fn len() -> usize {
        1
    }

    pub fn get_parameter_label(&self, index: i32) -> String {
        // TODO
        "".to_owned()
    }

    pub fn get_parameter_text(&self, index: i32) -> String {
        // TODO
        "".to_owned()
    }

    pub fn get_parameter_name(&self, index: i32) -> String {
        // TODO
        "".to_owned()
    }

    pub fn get_parameter(&self, index: i32) -> f64 {
        DependentValueIndex::from_i64(index as i64)
            .map(|idx| *self.vec.index(idx as usize))
            .unwrap_or(0.0)
    }

    pub fn set_parameter(&mut self, index: i32, value: f64) {
        *self.vec.index_mut(index as usize) = value;
    }
}
