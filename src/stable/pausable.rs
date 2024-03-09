use std::fmt::Display;

use crate::{
    functions::types::{Pausable, Reasonable},
    types::TimestampNanos,
};

// ================== 简单实现 ==================

/// 维护原因对象
#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct PauseReason {
    /// 维护时间
    pub timestamp_nanos: TimestampNanos,

    /// 维护原因
    pub message: String,
}

impl Display for PauseReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl std::error::Error for PauseReason {}

impl Reasonable for PauseReason {
    fn message(&self) -> &str {
        &self.message
    }
}

impl PauseReason {
    /// 构造维护原因
    pub fn new(message: String) -> Self {
        PauseReason {
            timestamp_nanos: crate::times::now(),
            message,
        }
    }
}

/// 记录维护状态
#[derive(candid::CandidType, candid::Deserialize, Debug, Clone, Default)]
pub struct Pause(Option<PauseReason>);

impl Pausable<PauseReason> for Pause {
    // 查询
    fn pause_query(&self) -> &Option<PauseReason> {
        &self.0
    }
    // 修改
    // 设置维护状态
    fn pause_replace(&mut self, reason: Option<PauseReason>) {
        self.0 = reason;
    }
}
