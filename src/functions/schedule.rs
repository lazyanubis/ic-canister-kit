use std::cell::Cell;

use crate::types::DurationNanos;

const MIN_SCHEDULE_INTERVAL_NANOS: u128 = 1_000_000_000;

thread_local! {
    static SCHEDULE_TASK_RUNNING: Cell<bool> = const { Cell::new(false) };
}

/// 定时任务执行锁。
///
/// 锁离开作用域时会自动释放，使后续定时任务可以继续执行。
#[must_use = "dropping the guard immediately releases the schedule task lock"]
#[non_exhaustive]
pub struct ScheduleTaskGuard;

impl Drop for ScheduleTaskGuard {
    fn drop(&mut self) {
        SCHEDULE_TASK_RUNNING.with(|running| running.set(false));
    }
}

/// 尝试取得定时任务执行锁，防止自动任务和手动触发并发执行。
pub fn try_schedule_task_guard() -> Result<ScheduleTaskGuard, String> {
    SCHEDULE_TASK_RUNNING.with(|running| {
        if running.get() {
            return Err("Schedule task is already running.".to_string());
        }
        running.set(true);
        Ok(ScheduleTaskGuard)
    })
}

/// 验证定时任务间隔是否能够被 IC 定时器安全执行。
///
/// 已启用的任务间隔不得少于一秒，也不得超过定时器的 `u64` 纳秒范围或导致当前 Canister 时间溢出。
pub fn validate_schedule(schedule: Option<DurationNanos>) -> Result<Option<DurationNanos>, String> {
    if let Some(interval) = schedule {
        let nanos = interval.into_inner();
        if nanos < MIN_SCHEDULE_INTERVAL_NANOS {
            return Err(format!(
                "Schedule interval must be at least {MIN_SCHEDULE_INTERVAL_NANOS} nanoseconds."
            ));
        }
        let nanos = u64::try_from(nanos)
            .map_err(|_| "Schedule interval exceeds the timer's u64 nanosecond range.".to_string())?;
        if ic_cdk::api::time().checked_add(nanos).is_none() {
            return Err("Schedule interval is too large for the current canister time.".to_string());
        }
    }
    Ok(schedule)
}

// ================== 异步执行代码 ==================
// 不知道和 ic_cdk::spawn(future) 区别在哪里

/// 异步执行代码
#[inline]
pub fn async_execute<Task>(task: Task) -> ic_cdk_timers::TimerId
where
    Task: Future<Output = ()> + 'static,
{
    ic_cdk_timers::set_timer(std::time::Duration::ZERO, task)
}

// ================== 功能 ==================

pub use ic_cdk_timers::TimerId;

/// 定时任务功能
pub trait Schedulable {
    /// 查询
    fn schedule_find(&self) -> Option<DurationNanos>;
    /// 修改
    fn schedule_replace(&mut self, schedule: Option<DurationNanos>);
}

/// 停止任务
#[inline]
pub fn schedule_stop(timer_id: Option<TimerId>) {
    if let Some(timer_id) = timer_id {
        ic_cdk_timers::clear_timer(timer_id)
    }
}

/// 启动任务
#[inline]
pub fn schedule_start<F>(schedule: &Option<DurationNanos>, task: impl FnMut() -> F + 'static) -> Option<TimerId>
where
    F: Future<Output = ()> + 'static,
{
    schedule.map(|interval| {
        ic_cdk_timers::set_timer_interval(std::time::Duration::from_nanos(interval.into_inner() as u64), task)
    })
}

// ================== 简单实现 ==================

/// 定时任务简单实现
pub mod basic {
    use candid::CandidType;
    use serde::{Deserialize, Serialize};

    use crate::{functions::types::Schedulable, types::DurationNanos};

    #[cfg(feature = "schedule")]
    mod schedule {
        use std::cell::RefCell;

        use ic_cdk_timers::TimerId;

        use crate::types::DurationNanos;

        thread_local! {
            static SCHEDULE: RefCell<Option<TimerId>> = RefCell::default(); // 定时任务 id 记录
        }

        /// 停止定时任务
        #[inline]
        pub fn stop_schedule() {
            SCHEDULE.with_borrow_mut(|timer_id| crate::functions::schedule::schedule_stop(std::mem::take(timer_id)));
        }

        /// 启动定时任务
        #[inline]
        pub fn start_schedule<F>(schedule: &Option<DurationNanos>, task: impl FnMut() -> F + 'static)
        where
            F: Future<Output = ()> + 'static,
        {
            stop_schedule();
            let new_timer_id = crate::functions::schedule::schedule_start(schedule, task);
            SCHEDULE.with_borrow_mut(|timer_id| *timer_id = new_timer_id);
        }
    }
    #[cfg(feature = "schedule")]
    pub use schedule::*;

    /// 周期定时任务
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
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
}

#[cfg(feature = "schedule")]
pub use basic::{start_schedule, stop_schedule};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_task_guard_prevents_reentry_until_dropped() {
        let first = try_schedule_task_guard().expect("first schedule task should acquire the guard");
        assert!(try_schedule_task_guard().is_err());
        drop(first);
        assert!(try_schedule_task_guard().is_ok());
    }

    #[test]
    fn schedule_interval_rejects_unsafe_values_before_reading_canister_time() {
        assert!(validate_schedule(Some(0_u128.into())).is_err());
        assert!(validate_schedule(Some((u64::MAX as u128 + 1).into())).is_err());
    }
}
