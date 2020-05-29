use std::ops;

use nalgebra as na;

use num_derive::FromPrimitive;

pub type Value = f64;

pub type NumVoices = na::U16;

pub type NumGlobalIndependents = na::U5;
pub type NumVoiceIndependents = na::U7;

pub type NumGlobalDependents = na::U6;
pub type NumVoiceDependents = na::U6;

#[derive(Clone, Copy)]
pub enum GlobalIndependent {
    One = 0,
    PitchBend,
    LFO1,
    LFO2,
    LFO3,
}

#[derive(Clone, Copy)]
pub enum VoiceIndependent {
    Pitch,
    Envelope1,
    Envelope2,
}

#[derive(Clone, Copy)]
pub enum GlobalDependent {
    LFO1Frequency,
    LFO2Frequency,
    Mod1,
    Mod2,
    Mod3,
}

#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum VoiceDependent {
    OscPitch = 0,
    OscVolume,
}

#[derive(Clone, Copy)]
pub struct VoicesIndependent(pub usize, pub VoiceIndependent);

#[derive(Clone, Copy)]
pub struct VoicesDependent(pub usize, pub VoiceDependent);

#[derive(Clone, Copy)]
pub struct WeightGlobalGlobal(pub GlobalIndependent, pub GlobalDependent);

#[derive(Clone, Copy)]
pub struct WeightGlobalVoice(pub GlobalIndependent, pub VoiceDependent);

#[derive(Clone, Copy)]
pub struct WeightVoiceVoice(pub VoiceIndependent, pub VoiceDependent);

type Matrix<R, C> = na::MatrixMN<Value, R, C>;
type Row<C> = Matrix<na::U1, C>;

pub type GlobalIndependents = Row<NumGlobalIndependents>;
pub type VoicesIndependents = Matrix<NumVoices, NumVoiceIndependents>;

pub type WeightsGlobalGlobal = Matrix<NumGlobalIndependents, NumGlobalDependents>;
pub type WeightsGlobalVoice = Matrix<NumGlobalIndependents, NumVoiceDependents>;
pub type WeightsVoiceVoice = Matrix<NumVoiceIndependents, NumVoiceDependents>;

pub type GlobalDependents = Row<NumGlobalDependents>;
pub type VoicesDependents = Matrix<NumVoices, NumVoiceDependents>;

macro_rules! impl_row_index {
    ($indexable_type:ty, $index_type:ty) => {
        impl ops::Index<$index_type> for $indexable_type {
            type Output = Value;

            fn index(&self, index: $index_type) -> &Value {
                self.index(index as usize)
            }
        }

        impl ops::IndexMut<$index_type> for $indexable_type {
            fn index_mut(&mut self, index: $index_type) -> &mut Value {
                self.index_mut(index as usize)
            }
        }
    };
}

macro_rules! impl_matrix_index {
    ($indexable_type:ty, $index_type:ty) => {
        impl ops::Index<$index_type> for $indexable_type {
            type Output = Value;

            fn index(&self, index: $index_type) -> &Value {
                self.index((index.0 as usize, index.1 as usize))
            }
        }

        impl ops::IndexMut<$index_type> for $indexable_type {
            fn index_mut(&mut self, index: $index_type) -> &mut Value {
                self.index_mut((index.0 as usize, index.1 as usize))
            }
        }
    };
}

impl_row_index!(GlobalIndependents, GlobalIndependent);
impl_matrix_index!(VoicesIndependents, VoicesIndependent);

impl_matrix_index!(WeightsGlobalGlobal, WeightGlobalGlobal);
impl_matrix_index!(WeightsGlobalVoice, WeightGlobalVoice);
impl_matrix_index!(WeightsVoiceVoice, WeightVoiceVoice);

impl_row_index!(GlobalDependents, GlobalDependent);
impl_matrix_index!(VoicesDependents, VoicesDependent);
