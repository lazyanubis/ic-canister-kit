use std::fmt::Display;

use candid::CandidType;
use serde::{Deserialize, Serialize};

/// 时间相关

/// 时间戳 纳秒
#[derive(
    CandidType, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct TimestampNanos(i128);

/// 时间跨度 纳秒
#[derive(
    CandidType, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct DurationNanos(u128);

impl From<i128> for TimestampNanos {
    fn from(value: i128) -> Self {
        TimestampNanos(value)
    }
}

impl From<u128> for DurationNanos {
    fn from(value: u128) -> Self {
        DurationNanos(value)
    }
}

impl Display for TimestampNanos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for DurationNanos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TimestampNanos {
    /// 内部数据
    pub fn into_inner(self) -> i128 {
        self.0
    }
}

impl DurationNanos {
    /// 内部数据
    pub fn into_inner(self) -> u128 {
        self.0
    }
}

/// 当前时间戳
#[inline]
pub fn now() -> TimestampNanos {
    (ic_cdk::api::time() as i128).into()
}
