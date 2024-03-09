use std::fmt::Display;

use crate::times::{now, TimestampNanos};

/// 维护状态

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

impl PauseReason {
    /// 构造维护原因
    pub fn new(message: String) -> Self {
        PauseReason {
            timestamp_nanos: now(),
            message,
        }
    }
}

// ================== 功能 ==================

/// 维护记录
pub trait Pausable {
    // 查询

    /// 查询维护状态
    fn pause_query(&self) -> &Option<PauseReason>;
    // 修改

    /// 修改维护状态
    fn pause_replace(&mut self, reason: Option<PauseReason>);

    // 默认方法

    /// 是否维护中
    fn pause_is_paused(&self) -> bool {
        self.pause_query().is_some()
    }
    /// 是否正常运行
    fn pause_is_running(&self) -> bool {
        !self.pause_is_paused()
    }
    /// 正常运行中才能继续
    fn pause_must_be_running(&self) -> Result<(), String> {
        if let Some(reason) = &self.pause_query() {
            return Err(format!("Canister is paused: {}", reason.message));
        }
        Ok(())
    }
    /// 维护中才能继续
    fn pause_must_be_paused(&self) -> Result<(), String> {
        if self.pause_is_running() {
            return Err("Canister is running. Not paused.".into());
        }
        Ok(())
    }
}
