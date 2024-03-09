use crate::{functions::types::Schedulable, types::DurationNanos};

// ================== 简单实现 ==================

/// 周期定时任务
#[derive(candid::CandidType, candid::Deserialize, Debug, Clone, Default)]
pub struct Schedule(Option<DurationNanos>);

impl Schedulable for Schedule {
    // 查询
    fn schedule_find(&self) -> Option<DurationNanos> {
        self.0
    }
    // 修改
    fn schedule_replace(&mut self, schedule: Option<DurationNanos>) {
        self.0 = schedule
    }
}
