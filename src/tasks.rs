use std::time::Duration;

use super::times::Timestamp;

// ================== 心跳任务 ==================

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct HeartbeatConfig {
    pub enabled: bool,    // 是否启用
    pub last: Timestamp,  // 上次心跳检测时间，纳秒，每次触发后需要更新 last
    pub sleep: Timestamp, // 检测间隔  纳秒
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            last: 0,
            sleep: 1000000 * 1000 * 3600, // 默认 1 小时
        }
    }
}

impl HeartbeatConfig {
    pub fn beat(&mut self) -> Option<Timestamp> {
        // 1. 如果没有启用
        if !self.enabled {
            return None;
        }

        let now = super::times::now();

        // 2. 判断时间
        if now < self.last + self.sleep {
            return None;
        }

        // 3. 执行任务
        self.last = now;
        Some(now)
    }
}

// ================== 定时任务 ==================

pub use ic_cdk_timers::TimerId;

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct ScheduleConfig {
    pub enabled: bool,       // 是否启用
    pub interval: Timestamp, // 触发间隔 纳秒
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: 1000000 * 1000 * 3600, // 默认 1 小时
        }
    }
}

impl ScheduleConfig {
    pub fn clear(&self, timer_id: Option<TimerId>) {
        if timer_id.is_some() {
            // 关闭
            ic_cdk_timers::clear_timer(timer_id.unwrap());
        }
    }

    pub fn check(&self, task: impl FnMut() + 'static) -> Option<TimerId> {
        // 1. 如果没有启用
        if !self.enabled {
            return None;
        }

        // 2. 启动任务
        let timer_id = ic_cdk_timers::set_timer_interval(Duration::from_nanos(self.interval), task);
        Some(timer_id)
    }
}
