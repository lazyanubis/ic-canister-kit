/// 时间相关

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone, Copy)]
pub struct TimestampNanos(u64); // 时间戳 纳秒

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone, Copy)]
pub struct DurationNanos(u64); // 时间跨度 纳秒

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

impl TimestampNanos {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl DurationNanos {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

#[inline]
pub fn now() -> TimestampNanos {
    ic_cdk::api::time().into()
}
