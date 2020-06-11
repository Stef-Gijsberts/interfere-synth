use crate::values::{IGlobalRow, IVoice, IVoicesMatrix, IGlobal, DGlobal, DGlobalRow, IVoices};

use super::Voice;

#[derive(Default)]
pub struct Envelope {
    time_s: f64,
}

impl Envelope {
    pub fn set_time_s(&mut self, time_s: f64) {
        self.time_s = time_s;
    }

    pub fn values_requested(
        &self,
        iglobal: &mut IGlobalRow,
        ivoices: &mut IVoicesMatrix,
        voices: &mut [Option<Voice>; 16],
        dglobal: &DGlobalRow,
    ) {
        
        let attack_s = dglobal[DGlobal::EnvAttack];
        let decay_s = dglobal[DGlobal::EnvDecay];
        let sustain_0 = dglobal[DGlobal::EnvSustain];
        let release_s = dglobal[DGlobal::EnvRelease];

        for (idx, maybe_voice) in (0..).zip(voices.iter_mut()) {

            let val = match maybe_voice {
                Some(voice) => {
                    let dt_s = self.time_s - voice.time_started_s;
                    let dt_released_s = voice.time_released_s.map(|tr_s| tr_s - voice.time_started_s);

                    if let Some(val) = adsr(attack_s, decay_s, sustain_0, release_s, dt_s, dt_released_s) {
                        val
                    }
                    else {
                        *maybe_voice = None;
                        0.0
                    }

                }
                None => 0.0
            };

            ivoices[IVoices(idx, IVoice::Envelope)] = val;
        }
    }
}

fn adsr(attack_s: f64, decay_s: f64, sustain_0: f64, release_s: f64, t_s: f64, t_released_s: Option<f64>) -> Option<f64> {
    if t_s < attack_s {
        return Some(t_s / attack_s);
    }

    if t_s <= attack_s + decay_s {
        return Some(1.0 - (1.0 - sustain_0) * ((t_s - attack_s) / decay_s));
    }

    match t_released_s {
        None => Some(sustain_0),
        Some(tr_s) => {
            let released_for_s = t_s - tr_s;

            if released_for_s > release_s {
                return None;
            }

            return Some(f64::max(0.0, sustain_0 * (1.0 - (released_for_s / release_s))));
        },
    }
}