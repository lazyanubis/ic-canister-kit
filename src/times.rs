pub type Timestamp = u64; // 时间戳 纳秒

pub type Duration = u64; // 时间跨度 纳秒

#[inline]
pub fn now() -> Timestamp {
    ic_cdk::api::time()
}
