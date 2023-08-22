use crate::times::Timestamp;

use super::Stable;

/// 维护状态

#[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq, Clone)]
pub struct MaintainingReason {
    created: Timestamp,
    message: String,
}

#[derive(Debug, Default)]
pub struct Maintainable {
    maintaining: Option<MaintainingReason>,
}

pub type MaintainableState = (Option<MaintainingReason>,);

impl Stable<MaintainableState, MaintainableState> for Maintainable {
    fn store(&mut self) -> MaintainableState {
        let maintaining = std::mem::take(&mut self.maintaining);
        (maintaining,)
    }

    fn restore(&mut self, state: MaintainableState) {
        let _ = std::mem::replace(&mut self.maintaining, state.0);
    }
}

impl Maintainable {
    // 非维护中才能继续
    pub fn must_be_running(&self) {
        if let Some(reason) = &self.maintaining {
            panic!("System is maintaining: {}", reason.message.clone());
        }
    }
    // 当前状态是否维护中
    pub fn is_maintaining(&self) -> bool {
        self.maintaining.is_some()
    }
    // 设置尾注状态
    pub fn set_maintaining(&mut self, maintaining: Option<MaintainingReason>) {
        self.maintaining = maintaining;
    }
}
