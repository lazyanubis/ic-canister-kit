/// 初始化
pub mod initial;

/// 升级功能更
pub mod upgrade;

/// 维护功能
pub mod pausable;

/// 定时任务功能
pub mod schedule;

/// 权限功能
pub mod permission;

/// 记录功能
pub mod record;

/// 稳定内存功能
#[cfg(feature = "stable-structures")]
pub mod stable;

/// 类型
pub mod types;
