use crate::functions::types::{Pausable, PauseReason};

// ================== 简单实现 ==================

/// 记录维护状态
#[derive(candid::CandidType, candid::Deserialize, Debug, Clone, Default)]
pub struct Pause(Option<PauseReason>);

impl Pausable for Pause {
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
