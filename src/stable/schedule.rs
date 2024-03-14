use std::cell::RefCell;

use ic_cdk_timers::TimerId;

use crate::{functions::types::Schedulable, types::DurationNanos};

// ================== 简单实现 ==================

thread_local! {
    static SCHEDULE: RefCell<Option<TimerId>> = RefCell::default(); // 定时任务 id 记录
}

/// 停止定时任务
#[inline]
pub fn stop_schedule() {
    SCHEDULE.with_borrow_mut(|timer_id| {
        if let Some(timer_id) = std::mem::take(timer_id) {
            ic_cdk_timers::clear_timer(timer_id)
        }
    });
}

/// 启动定时任务
#[inline]
pub fn start_schedule(schedule: &Option<DurationNanos>, task: impl FnMut() + 'static) {
    stop_schedule();
    let new_timer_id = schedule.map(|interval| {
        ic_cdk_timers::set_timer_interval(
            std::time::Duration::from_nanos(interval.into_inner() as u64),
            task,
        )
    });
    SCHEDULE.with_borrow_mut(|timer_id| *timer_id = new_timer_id);
}

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
