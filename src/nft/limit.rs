use candid::CandidType;
use serde::Deserialize;

use crate::stable::Stable;
use crate::times::{now, Timestamp};

#[derive(CandidType, Deserialize, Debug, Default, Clone)]
pub struct LimitDuration {
    start: Timestamp,
    end: Timestamp,
}

impl LimitDuration {
    pub fn new(start: Timestamp, end: Timestamp) -> Self {
        LimitDuration { start, end }
    }
}

#[derive(CandidType, Deserialize, Debug, Default)]
pub struct NftLimit {
    limits: Vec<LimitDuration>, // 该时间段内,是限制时间
}

pub type NftLimitState = (Vec<LimitDuration>,);

impl Stable<NftLimitState, NftLimitState> for NftLimit {
    fn store(&mut self) -> NftLimitState {
        let limits = std::mem::take(&mut self.limits);
        (limits,)
    }

    fn restore(&mut self, state: NftLimitState) {
        let _ = std::mem::replace(&mut self.limits, state.0);
    }
}

impl NftLimit {
    pub fn is_limit(&self) -> bool {
        let now = now();
        for LimitDuration { start, end } in self.limits.iter() {
            if start <= &now && &now < end {
                return false;
            }
        }
        true
    }

    pub fn set_limits(&mut self, limits: Vec<LimitDuration>) {
        self.limits = limits;
    }

    pub fn get_limits(&self) -> Vec<LimitDuration> {
        self.limits.clone()
    }
}
