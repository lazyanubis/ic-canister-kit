use crate::types::DurationNanos;

// ================== 异步执行代码 ==================
// 不知道和 ic_cdk::spawn(future) 区别在哪里

/// 异步执行代码
#[inline]
pub fn async_execute<Task>(task: Task) -> ic_cdk_timers::TimerId
where
    Task: FnOnce() + 'static,
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
pub fn schedule_start(
    schedule: &Option<DurationNanos>,
    task: impl FnMut() + 'static,
) -> Option<TimerId> {
    schedule.map(|interval| {
        ic_cdk_timers::set_timer_interval(
            std::time::Duration::from_nanos(interval.into_inner() as u64),
            task,
        )
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
            SCHEDULE.with_borrow_mut(|timer_id| {
                crate::functions::schedule::schedule_stop(std::mem::take(timer_id))
            });
        }

        /// 启动定时任务
        #[inline]
        pub fn start_schedule(schedule: &Option<DurationNanos>, task: impl FnMut() + 'static) {
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
