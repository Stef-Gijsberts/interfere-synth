use crate::values::*;

use nalgebra::Dim;
use num_traits::FromPrimitive;


#[derive(Clone, Copy, Debug)]
pub enum Parameter {
    GlobalGlobal(WGlobalGlobal),
    GlobalVoice(WGlobalVoice),
    VoiceVoice(WVoiceVoice),
}

impl Parameter {
    pub fn from_i32(i: i32) -> Option<Parameter> {
        assert!(i >= 0);

        let num_ig = NumIGlobal::try_to_usize().unwrap() as i32;
        let num_iv = NumIVoice::try_to_usize().unwrap() as i32;
        let num_dg = NumDGlobal::try_to_usize().unwrap() as i32;
        let num_dv = NumDVoice::try_to_usize().unwrap() as i32;

        let num_wgg = num_ig * num_dg;
        let num_wgv = num_ig * num_dv;
        let num_wvv = num_iv * num_dv;

        let start_wgg = 0;
        let end_wgg = start_wgg + num_wgg;

        let start_wgv = end_wgg;
        let end_wgv = start_wgv + num_wgv;

        let start_wvv = end_wgv;
        let end_wvv = start_wvv + num_wvv;

        if (start_wgg..end_wgg).contains(&i) {
            let row = i / num_ig;
            let col = i % num_ig;

            if let (Some(a), Some(b)) = (IGlobal::from_i32(row), DGlobal::from_i32(col)) {
                Some(Parameter::GlobalGlobal(WGlobalGlobal(a, b)))
            }
            else {
                None
            }
        }

        else if (start_wgv..end_wgv).contains(&i) {
            let row = i / num_ig;
            let col = i % num_ig;

            if let (Some(a), Some(b)) = (IGlobal::from_i32(row), DVoice::from_i32(col)) {
                Some(Parameter::GlobalVoice(WGlobalVoice(a, b)))
            }
            else {
                None
            }
        }

        else if (start_wvv..end_wvv).contains(&i) {
            let row = i / num_iv;
            let col = i % num_iv;

            if let (Some(a), Some(b)) = (IVoice::from_i32(row), DVoice::from_i32(col)) {
                Some(Parameter::VoiceVoice(WVoiceVoice(a, b)))
            }
            else {
                None
            }
        }

        else {
            None
        }
    }
}
