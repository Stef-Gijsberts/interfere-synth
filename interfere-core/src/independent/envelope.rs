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
        voices: [Option<Voice>; 16],
        dglobal: &DGlobalRow,
    ) {
        
        let attack_s = dglobal[DGlobal::Env1Attack];
        let decay_s = dglobal[DGlobal::Env1Decay];
        let sustain_0 = dglobal[DGlobal::Env1Sustain];
        let release_s = dglobal[DGlobal::Env1Release];

        for (idx, maybe_voice) in (0..).zip(voices.iter()) {
            let val = match maybe_voice {
                Some(voice) => {
                    let dt_s = self.time_s - voice.time_started_s;
                    let dt_released_s = voice.time_released_s.map(|tr_s| tr_s - voice.time_started_s);

                    adsr(attack_s, decay_s, sustain_0, release_s, dt_s, dt_released_s)

                }
                None => 0.0
            };

            ivoices[IVoices(idx, IVoice::Envelope1)] = val;
        }
    }
}

fn adsr(attack_s: f64, decay_s: f64, sustain_0: f64, release_s: f64, t_s: f64, t_released_s: Option<f64>) -> f64 {
    if t_s < attack_s {
        return t_s / attack_s;
    }

    if t_s < attack_s + decay_s {
        return 1.0 - ((t_s - attack_s) / decay_s);
    }

    match t_released_s {
        None => sustain_0,
        Some(tr_s) => {
            let released_for_s = t_s - tr_s;
            return sustain_0 * (1.0 - released_for_s / release_s);
        },
    }
}