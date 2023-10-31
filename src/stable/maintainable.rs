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
    fn maintaining_must_be_running(&self);
    fn maintaining_must_be_maintaining(&self);
    fn maintaining_is_maintaining(&self) -> bool;
    fn maintaining_is_running(&self) -> bool {
        !self.maintaining_is_maintaining()
    }
    fn maintaining_query(&self) -> Option<MaintainingReason>;
    // 修改
    fn maintaining_update_maintaining(&mut self, reason: MaintainingReason);
    fn maintaining_update_running(&mut self);
    fn maintaining_replace(&mut self, reason: Option<MaintainingReason>) {
        if let Some(reason) = reason {
            self.maintaining_update_maintaining(reason)
        } else {
            self.maintaining_update_running()
        }
    }
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone, Default)]
pub struct Maintaining(Option<MaintainingReason>);

impl Maintainable for Maintaining {
    // 查询
    // 非维护中才能继续
    fn maintaining_must_be_running(&self) {
        if let Some(reason) = &self.0 {
            panic!("System is maintaining: {}", reason.message);
        }
    }
    fn maintaining_must_be_maintaining(&self) {
        if let None = &self.0 {
            panic!("System is running. Not maintaining.");
        }
    }
    // 当前状态是否维护中
    fn maintaining_is_maintaining(&self) -> bool {
        self.0.is_some()
    }
    fn maintaining_query(&self) -> Option<MaintainingReason> {
        self.0.clone()
    }
    // 修改
    // 设置维护状态
    fn maintaining_update_maintaining(&mut self, reason: MaintainingReason) {
        self.0 = Some(reason);
    }
    fn maintaining_update_running(&mut self) {
        self.0 = None
    }
}

impl Display for MaintainingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
