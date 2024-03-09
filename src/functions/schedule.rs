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

/// 启动任务
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
/// 停止任务
pub fn schedule_stop(timer_id: Option<TimerId>) {
    if let Some(timer_id) = timer_id {
        ic_cdk_timers::clear_timer(timer_id)
    }
}
