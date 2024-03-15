use crate::times::{now, Timestamp};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct LimitDuration {
    start: Timestamp,
    end: Timestamp,
}

impl LimitDuration {
    pub fn new(start: Timestamp, end: Timestamp) -> Self {
        LimitDuration { start, end }
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct NftLimit(Vec<LimitDuration>); // 该时间段内,是限制时间

pub type NftLimitState = (Vec<LimitDuration>,);

impl NftLimit {
    pub fn is_limit(&self) -> bool {
        let now = now();
        for LimitDuration { start, end } in self.0.iter() {
            if start <= &now && &now < end {
                return false;
            }
        }
        true
    }

    pub fn set_limits(&mut self, limits: Vec<LimitDuration>) {
        self.0 = limits;
    }

    pub fn get_limits(&self) -> Vec<LimitDuration> {
        self.0.clone()
    }
}
