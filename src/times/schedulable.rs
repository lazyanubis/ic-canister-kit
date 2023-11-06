use super::TimestampNanos;

#[inline]
pub fn async_execute(task: impl FnOnce() + 'static) -> ic_cdk_timers::TimerId {
    ic_cdk_timers::set_timer(std::time::Duration::ZERO, task)
}

// ================== 心跳任务 ==================

// #[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
// pub struct HeartbeatConfig {
//     pub enabled: bool,    // 是否启用
//     pub last: Timestamp,  // 上次心跳检测时间，纳秒，每次触发后需要更新 last
//     pub sleep: Timestamp, // 检测间隔  纳秒
// }

// impl Default for HeartbeatConfig {
//     fn default() -> Self {
//         Self {
//             enabled: true,
//             last: 0,
//             sleep: 1000000 * 1000 * 3600, // 默认 1 小时
//         }
//     }
// }

// impl HeartbeatConfig {
//     pub fn beat(&mut self) -> Option<Timestamp> {
//         // 1. 如果没有启用
//         if !self.enabled {
//             return None;
//         }

//         let now = super::now();

//         // 2. 判断时间
//         if now < self.last + self.sleep {
//             return None;
//         }

//         // 3. 执行任务
//         self.last = now;
//         Some(now)
//     }
// }

// ================== 定时任务 ==================

pub use ic_cdk_timers::TimerId;

pub fn schedule_start(
    schedule: Option<TimestampNanos>,
    task: impl FnMut() + 'static,
) -> Option<TimerId> {
    schedule.and_then(|interval| {
        Some(ic_cdk_timers::set_timer_interval(
            std::time::Duration::from_nanos(interval),
            task,
        ))
    })
}

pub fn schedule_stop(timer_id: Option<TimerId>) {
    if let Some(timer_id) = timer_id {
        ic_cdk_timers::clear_timer(timer_id)
    }
}

pub trait Schedulable {
    // 查询
    fn schedule_find(&self) -> Option<TimestampNanos>;
    // 修改
    fn schedule_replace(&mut self, schedule: Option<TimestampNanos>);
}

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
