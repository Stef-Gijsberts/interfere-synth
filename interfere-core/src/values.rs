use std::fmt;

use num_derive::{FromPrimitive, ToPrimitive};

use nalgebra as na;

pub type Value = f64;

#[derive(FromPrimitive, ToPrimitive, Clone, Copy)]
pub enum DependentValueIndex {
    OscAVolume = 0,
    OscBVolume = 1,
    OscAPitch = 2,
    OscBPitch = 3,
}

impl fmt::Display for DependentValueIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DependentValueIndex::OscAVolume => write!(f, "Oscillator A Volume"),
            DependentValueIndex::OscBVolume => write!(f, "Oscillator B Volume"),
            DependentValueIndex::OscAPitch => write!(f, "Oscillator A Pitch"),
            DependentValueIndex::OscBPitch => write!(f, "Oscillator B Pitch"),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Clone, Copy)]
pub enum IndependentValueIndex {
    One = 0,
    Pitch = 1,
    OscA = 2,
    OscB = 3,
    Mod1,
    Mod2,
    Mod3,
    LFO1,
    LFO2,
    LFO3,
}

pub struct ConnectionValueIndex(pub IndependentValueIndex, pub DependentValueIndex);

pub type NumDependentValues = na::U4;
pub type NumIndependentValues = na::U9;

pub type ConnectionWeightMatrix = na::MatrixMN<Value, NumIndependentValues, NumDependentValues>;
pub type DependentValueRow = na::RowVectorN<Value, NumDependentValues>;
pub type IndependentValueRow = na::RowVectorN<Value, NumIndependentValues>;



impl<'a> na::indexing::MatrixIndex<'a, Value, na::U1, NumDependentValues, na::storage::Owned<Value, na::U1, NumDependentValues>> for DependentValueIndex {
    type Output = &'a Value;

    fn contained_by(&self, matrix: &DependentValueRow) -> bool {
        (*self as usize).contained_by(matrix)
    }

    /// Produces a mutable view of the data at this location, without
    /// performing any bounds checking.
    #[doc(hidden)]
    unsafe fn get_unchecked(self, matrix: &'a DependentValueRow) -> Self::Output {
        matrix.get_unchecked(self as usize)
    }
}

impl<'a> na::indexing::MatrixIndexMut<'a, Value, na::U1, NumDependentValues, na::storage::Owned<Value, na::U1, NumDependentValues>> for DependentValueIndex {
    type OutputMut = &'a mut Value;

    /// Produces a mutable view of the data at this location, without
    /// performing any bounds checking.
    #[doc(hidden)]
    unsafe fn get_unchecked_mut(self, matrix: &'a mut DependentValueRow) -> Self::OutputMut {
        matrix.get_unchecked_mut(self as usize)
    }
}


impl<'a> na::indexing::MatrixIndex<'a, Value, na::U1, NumIndependentValues, na::storage::Owned<Value, na::U1, NumIndependentValues>> for IndependentValueIndex {
    type Output = &'a Value;

    fn contained_by(&self, matrix: &IndependentValueRow) -> bool {
        (*self as usize).contained_by(matrix)
    }

    /// Produces a mutable view of the data at this location, without
    /// performing any bounds checking.
    #[doc(hidden)]
    unsafe fn get_unchecked(self, matrix: &'a IndependentValueRow) -> Self::Output {
        matrix.get_unchecked(self as usize)
    }
}

impl<'a> na::indexing::MatrixIndexMut<'a, Value, na::U1, NumIndependentValues, na::storage::Owned<Value, na::U1, NumIndependentValues>> for IndependentValueIndex {
    type OutputMut = &'a mut Value;

    /// Produces a mutable view of the data at this location, without
    /// performing any bounds checking.
    #[doc(hidden)]
    unsafe fn get_unchecked_mut(self, matrix: &'a mut IndependentValueRow) -> Self::OutputMut {
        matrix.get_unchecked_mut(self as usize)
    }
}


impl<'a> na::indexing::MatrixIndex<'a, Value, NumIndependentValues, NumDependentValues, na::storage::Owned<Value, NumIndependentValues, NumDependentValues>> for ConnectionValueIndex {
    type Output = &'a Value;

    fn contained_by(&self, matrix: &ConnectionWeightMatrix) -> bool {
        (self.0 as usize, self.1 as usize).contained_by(matrix)
    }

    /// Produces a mutable view of the data at this location, without
    /// performing any bounds checking.
    #[doc(hidden)]
    unsafe fn get_unchecked(self, matrix: &'a ConnectionWeightMatrix) -> Self::Output {
        matrix.get_unchecked((self.0 as usize, self.1 as usize))
    }
}

impl<'a> na::indexing::MatrixIndexMut<'a, Value, NumIndependentValues, NumDependentValues, na::storage::Owned<Value, NumIndependentValues, NumDependentValues>> for ConnectionValueIndex {
    type OutputMut = &'a mut Value;

    /// Produces a mutable view of the data at this location, without
    /// performing any bounds checking.
    #[doc(hidden)]
    unsafe fn get_unchecked_mut(self, matrix: &'a mut ConnectionWeightMatrix) -> Self::OutputMut {
        matrix.get_unchecked_mut((self.0 as usize, self.1 as usize))
    }
}
