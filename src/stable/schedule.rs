use crate::{functions::types::Schedulable, types::TimestampNanos};

// ================== 简单实现 ==================

/// 周期定时任务
#[derive(candid::CandidType, candid::Deserialize, Debug, Clone, Default)]
pub struct Schedule(Option<TimestampNanos>);

impl Schedulable for Schedule {
    // 查询
    fn schedule_find(&self) -> Option<TimestampNanos> {
        self.0
    }
    // 修改
    fn schedule_replace(&mut self, schedule: Option<TimestampNanos>) {
        self.0 = schedule
    }
}
