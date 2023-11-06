#[cfg(feature = "times_schedulable")]
pub mod schedulable;

pub mod types;

/// 时间相关

pub type TimestampNanos = u64; // 时间戳 纳秒

pub type DurationNanos = u64; // 时间跨度 纳秒

#[inline]
pub fn now() -> TimestampNanos {
    ic_cdk::api::time()
}
