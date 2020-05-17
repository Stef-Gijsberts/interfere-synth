use nalgebra as na;

pub type Value = f64;

pub enum DependentValueIndex {
    OscAVolume = 0,
    OscBVolume = 1,
    OscAPitch = 2,
    OscBPitch = 3,
}

pub enum IndependentValueIndex {
    One = 0,
    OscA = 1,
    OscB = 2,
    Mod1,
    Mod2,
    Mod3,
    LFO1,
    LFO2,
    LFO3,
}

pub type NumDependentValues = na::U4;
pub type NumIndependentValues = na::U9;

pub type ConnectionWeightMatrix = na::MatrixMN<Value, NumDependentValues, NumIndependentValues>;
pub type DependentValueVector = na::VectorN<Value, NumDependentValues>;
pub type IndependentValueVector = na::VectorN<Value, NumIndependentValues>;
