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
pub enum IGlobal {
    One = 0,
    PitchBend,
    LFO1,
    LFO2,
    LFO3,
}

#[derive(Clone, Copy)]
pub enum IVoice {
    Pitch,
    Envelope1,
    Envelope2,
}

#[derive(Clone, Copy)]
pub enum DGlobal {
    LFO1Frequency,
    LFO2Frequency,
    Mod1,
    Mod2,
    Mod3,
}

#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum DVoice {
    OscPitch = 0,
    OscVolume,
}

#[derive(Clone, Copy)]
pub struct IVoices(pub usize, pub IVoice);

#[derive(Clone, Copy)]
pub struct DVoices(pub usize, pub DVoice);

#[derive(Clone, Copy)]
pub struct WGlobalGlobal(pub IGlobal, pub DGlobal);

#[derive(Clone, Copy)]
pub struct WGlobalVoice(pub IGlobal, pub IVoice);

#[derive(Clone, Copy)]
pub struct WVoiceVoice(pub IVoice, pub DVoice);

type Matrix<R, C> = na::MatrixMN<Value, R, C>;
type Row<C> = Matrix<na::U1, C>;

pub type IGlobalRow = Row<NumGlobalIndependents>;
pub type IVoicesMatrix = Matrix<NumVoices, NumVoiceIndependents>;

pub type WGlobalGlobalMatrix = Matrix<NumGlobalIndependents, NumGlobalDependents>;
pub type WGlobalVoiceMatrix = Matrix<NumGlobalIndependents, NumVoiceDependents>;
pub type WVoiceVoiceMatrix = Matrix<NumVoiceIndependents, NumVoiceDependents>;

pub type DGlobalMatrix = Row<NumGlobalDependents>;
pub type DVoicesMatrix = Matrix<NumVoices, NumVoiceDependents>;

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
