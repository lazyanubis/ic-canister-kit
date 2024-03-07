use crate::types::TimestampNanos;

// ================== 异步执行代码 ==================
// 不知道和 ic_cdk::spawn(future) 区别在哪里
#[inline]
pub fn async_execute<Task>(task: Task) -> ic_cdk_timers::TimerId
where
    Task: FnOnce() + 'static,
{
    ic_cdk_timers::set_timer(std::time::Duration::ZERO, task)
}

// ================== 功能 ==================

pub use ic_cdk_timers::TimerId;

pub trait Schedulable {
    // 查询
    fn schedule_find(&self) -> Option<TimestampNanos>;
    // 修改
    fn schedule_replace(&mut self, schedule: Option<TimestampNanos>);

    // 默认方法
    // 启动任务
    fn schedule_start(&self, task: impl FnMut() + 'static) -> Option<TimerId> {
        self.schedule_find().map(|interval| {
            ic_cdk_timers::set_timer_interval(
                std::time::Duration::from_nanos(interval.into_inner()),
                task,
            )
        })
    }
    // 停止任务
    fn schedule_stop(&self, timer_id: Option<TimerId>) {
        if let Some(timer_id) = timer_id {
            ic_cdk_timers::clear_timer(timer_id)
        }
    }
}

// ================== 简单实现 ==================

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
