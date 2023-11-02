use std::fmt::Display;

use crate::times::{now, Timestamp};

/// 维护状态

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct MaintainingReason {
    pub created: Timestamp,
    pub message: String,
}

impl MaintainingReason {
    pub fn new(message: String) -> Self {
        MaintainingReason {
            created: now(),
            message,
        }
    }
}

pub trait Maintainable {
    // 查询
    fn maintaining_query(&self) -> &Option<MaintainingReason>;
    fn maintaining_is_maintaining(&self) -> bool {
        self.maintaining_query().is_some()
    }
    fn maintaining_is_running(&self) -> bool {
        !self.maintaining_is_maintaining()
    }
    // 非维护中才能继续
    fn maintaining_must_be_running(&self) {
        if let Some(reason) = &self.maintaining_query() {
            panic!("System is maintaining: {}", reason.message);
        }
    }
    fn maintaining_must_be_maintaining(&self) {
        if self.maintaining_is_running() {
            panic!("System is running. Not maintaining.");
        }
    }
    // 修改
    fn maintaining_replace(&mut self, reason: Option<MaintainingReason>);
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone, Default)]
pub struct Maintaining(Option<MaintainingReason>);

impl Maintainable for Maintaining {
    // 查询
    fn maintaining_query(&self) -> &Option<MaintainingReason> {
        &self.0
    }
    // 修改
    // 设置维护状态
    fn maintaining_replace(&mut self, reason: Option<MaintainingReason>) {
        self.0 = reason;
    }
}

impl Display for MaintainingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
