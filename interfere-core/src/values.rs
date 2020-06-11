use std::ops;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use nalgebra as na;

use na::Dim;


pub type Value = f64;

pub type NumVoices = na::U16;

pub type NumIGlobal = na::U5;
pub type NumIVoice = na::U3;

pub type NumDGlobal = na::U9;
pub type NumDVoice = na::U3;

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

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum DGlobal {
    LFO1Frequency,
    LFO2Frequency,
    Env1Attack,
    Env1Decay,
    Env1Sustain,
    Env1Release,
    Mod1,
    Mod2,
    Mod3,
}

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum DVoice {
    OscPitch = 0,
    OscVolume,
    FilterFrequency,
}

#[derive(Clone, Copy)]
pub struct IVoices(pub usize, pub IVoice);

#[derive(Clone, Copy)]
pub struct DVoices(pub usize, pub DVoice);

#[derive(Clone, Copy)]
pub struct WGlobalGlobal(pub IGlobal, pub DGlobal);

#[derive(Clone, Copy)]
pub struct WGlobalVoice(pub IGlobal, pub DVoice);

#[derive(Clone, Copy)]
pub struct WVoiceVoice(pub IVoice, pub DVoice);

type Matrix<R, C> = na::MatrixMN<Value, R, C>;
type Row<C> = Matrix<na::U1, C>;

pub type IGlobalRow = Row<NumIGlobal>;
pub type IVoicesMatrix = Matrix<NumVoices, NumIVoice>;

pub type WGlobalGlobalMatrix = Matrix<NumIGlobal, NumDGlobal>;
pub type WGlobalVoiceMatrix = Matrix<NumIGlobal, NumDVoice>;
pub type WVoiceVoiceMatrix = Matrix<NumIVoice, NumDVoice>;

pub type DGlobalRow = Row<NumDGlobal>;
pub type DVoicesMatrix = Matrix<NumVoices, NumDVoice>;

#[derive(Clone, Copy, Debug)]
pub enum Parameter {
    Global(DGlobal),
    Voice(DVoice),
}

impl Parameter {
    pub fn from_i32(i: i32) -> Option<Parameter> {
        assert!(i >= 0);

        let num_dglobal = NumDGlobal::try_to_usize().unwrap() as i32;
        let num_dvoice = NumDVoice::try_to_usize().unwrap() as i32;

        let start_dglobal = 0;
        let end_dglobal = start_dglobal + num_dglobal;
        let start_dvoice = end_dglobal;
        let end_dvoice = start_dvoice + num_dvoice;

        if (start_dglobal..end_dglobal).contains(&i) {
            Some(Parameter::Global(DGlobal::from_i32(i).unwrap()))
        }
        else if (start_dvoice..end_dvoice).contains(&i) {
            Some(Parameter::Voice(DVoice::from_i32(i - start_dvoice).unwrap()))
        }
        else {
            None
        }
    }
}


macro_rules! impl_row_index {
    ($row_type: ty, $index_type: ty, $num_columns: ty) => {
        impl ops::Index<$index_type> for $row_type {
            type Output = Value;

            fn index(&self, index: $index_type) -> &Value {
                self.index(index as usize)
            }
        }

        impl ops::IndexMut<$index_type> for $row_type {
            fn index_mut(&mut self, index: $index_type) -> &mut Value {
                self.index_mut(index as usize)
            }
        }

        impl<'a>
            na::indexing::MatrixIndex<
                'a,
                Value,
                na::U1,
                $num_columns,
                na::storage::Owned<Value, na::U1, $num_columns>,
            > for $index_type
        {
            type Output = Value;

            fn contained_by(&self, matrix: &$row_type) -> bool {
                (*self as usize) < matrix.ncols()
            }

            unsafe fn get_unchecked(self, matrix: &'a $row_type) -> Self::Output {
                matrix[self as usize]
            }
        }

        impl<'a>
            na::indexing::MatrixIndexMut<
                'a,
                Value,
                na::U1,
                $num_columns,
                na::storage::Owned<Value, na::U1, $num_columns>,
            > for $index_type
        {
            type OutputMut = &'a mut Self::Output;

            unsafe fn get_unchecked_mut(self, matrix: &'a mut $row_type) -> Self::OutputMut {
                &mut matrix[self as usize]
            }
        }
    };
}

macro_rules! impl_matrix_index {
    ($matrix_type: ty, $index_type: ty, $num_rows: ty, $num_columns: ty) => {
        impl ops::Index<$index_type> for $matrix_type {
            type Output = Value;

            fn index(&self, index: $index_type) -> &Value {
                self.index((index.0 as usize, index.1 as usize))
            }
        }

        impl ops::IndexMut<$index_type> for $matrix_type {
            fn index_mut(&mut self, index: $index_type) -> &mut Value {
                self.index_mut((index.0 as usize, index.1 as usize))
            }
        }

        impl<'a>
            na::indexing::MatrixIndex<
                'a,
                Value,
                $num_rows,
                $num_columns,
                na::storage::Owned<Value, $num_rows, $num_columns>,
            > for $index_type
        {
            type Output = Value;

            fn contained_by(&self, matrix: &$matrix_type) -> bool {
                (self.0 as usize) < matrix.ncols() && (self.1 as usize) < matrix.nrows()
            }

            unsafe fn get_unchecked(self, matrix: &'a $matrix_type) -> Self::Output {
                matrix[(self.0 as usize, self.1 as usize)]
            }
        }

        impl<'a>
            na::indexing::MatrixIndexMut<
                'a,
                Value,
                $num_rows,
                $num_columns,
                na::storage::Owned<Value, $num_rows, $num_columns>,
            > for $index_type
        {
            type OutputMut = &'a mut Self::Output;

            unsafe fn get_unchecked_mut(self, matrix: &'a mut $matrix_type) -> Self::OutputMut {
                &mut matrix[(self.0 as usize, self.1 as usize)]
            }
        }
    };
}

impl_row_index!(IGlobalRow, IGlobal, NumIGlobal);
impl_row_index!(DGlobalRow, DGlobal, NumDGlobal);

impl_matrix_index!(WGlobalGlobalMatrix, WGlobalGlobal, NumIGlobal, NumDGlobal);
impl_matrix_index!(WGlobalVoiceMatrix, WGlobalVoice, NumIGlobal, NumDVoice);
impl_matrix_index!(WVoiceVoiceMatrix, WVoiceVoice, NumIVoice, NumDVoice);

impl_matrix_index!(IVoicesMatrix, IVoices, NumVoices, NumIVoice);
impl_matrix_index!(DVoicesMatrix, DVoices, NumVoices, NumDVoice);
