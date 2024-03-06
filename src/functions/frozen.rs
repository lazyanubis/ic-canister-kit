use std::fmt::Display;

use crate::times::{now, TimestampNanos};

/// 维护状态

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct FrozenReason {
    pub frozen: TimestampNanos,
    pub message: String,
}

impl Display for FrozenReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl std::error::Error for FrozenReason {}

impl FrozenReason {
    pub fn new(message: String) -> Self {
        FrozenReason {
            frozen: now(),
            message,
        }
    }
}

// ================== 功能 ==================

pub trait Frozen {
    // 查询
    fn frozen_query(&self) -> &Option<FrozenReason>;
    // 修改
    fn frozen_replace(&mut self, reason: Option<FrozenReason>);

    fn frozen_is_frozen(&self) -> bool {
        self.frozen_query().is_some()
    }
    fn frozen_is_running(&self) -> bool {
        !self.frozen_is_frozen()
    }
    // 非维护中才能继续
    fn frozen_must_be_running(&self) {
        if let Some(reason) = &self.frozen_query() {
            panic!("Canister is frozen: {}", reason.message);
        }
    }
    fn frozen_must_be_frozen(&self) {
        if self.frozen_is_running() {
            panic!("Canister is running. Not frozen.");
        }
    }
}

// ================== 简单实现 ==================

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone, Default)]
pub struct FrozenData(Option<FrozenReason>);

impl Frozen for FrozenData {
    // 查询
    fn frozen_query(&self) -> &Option<FrozenReason> {
        &self.0
    }
    // 修改
    // 设置维护状态
    fn frozen_replace(&mut self, reason: Option<FrozenReason>) {
        self.0 = reason;
    }
}
