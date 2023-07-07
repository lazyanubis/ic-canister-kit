use crate::identity::CallerId;

use super::times::Timestamp;

// ================= 日志相关 =================

#[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq, Clone)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq, Clone)]
pub struct Log {
    pub time: Timestamp,  // 时间戳 纳秒
    pub level: LogLevel,  // 日志级别
    pub caller: CallerId, // 调用人
    pub content: String,  // 日志内容
}

// 日志记录
pub type Logs = Vec<Log>;

pub fn log_insert_with_mut_logs(
    logs: &mut Logs,
    level: LogLevel,
    caller: CallerId,
    content: String,
) {
    logs.push(Log {
        time: super::times::now(),
        level,
        caller,
        content,
    });
}
