use std::fmt::Display;

/// 时间相关

/// 时间戳 纳秒
#[derive(
    candid::CandidType, candid::Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct TimestampNanos(u64);

/// 时间跨度 纳秒
#[derive(
    candid::CandidType, candid::Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct DurationNanos(u64);

impl From<u64> for TimestampNanos {
    fn from(value: u64) -> Self {
        TimestampNanos(value)
    }
}

impl From<u64> for DurationNanos {
    fn from(value: u64) -> Self {
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
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl DurationNanos {
    /// 内部数据
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

/// 当前时间戳
#[inline]
pub fn now() -> TimestampNanos {
    ic_cdk::api::time().into()
}
