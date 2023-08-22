use candid::{CandidType, Deserialize};

use crate::common::pages::{page_find_with_reserve, Page, PageData};
use crate::identity::CallerId;
use crate::times::Timestamp;

use super::Stable;

/// 日志记录

// 日志等级
#[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq, Clone)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

// 每条日志记录
#[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq, Clone)]
pub struct Log {
    pub time: Timestamp,  // 时间戳 纳秒
    pub level: LogLevel,  // 日志级别
    pub caller: CallerId, // 调用人
    pub content: String,  // 日志内容
}

// 持久化的日志记录对象
#[derive(CandidType, Deserialize, Debug, Default)]
pub struct StableLogs {
    logs: Vec<Log>,
}

// 持久化的对象
pub type StableLogsState = (Vec<Log>,);

// 持久化方法
impl Stable<StableLogsState, StableLogsState> for StableLogs {
    fn store(&mut self) -> StableLogsState {
        let logs = std::mem::take(&mut self.logs);
        (logs,)
    }
    fn restore(&mut self, restore: StableLogsState) {
        let _ = std::mem::replace(&mut self.logs, restore.0);
    }
}

impl StableLogs {
    // 插入
    pub fn push(&mut self, level: LogLevel, caller: CallerId, content: String) {
        self.logs.push(Log {
            time: crate::times::now(),
            level,
            caller,
            content,
        });
    }

    // 查询所有
    pub fn query_all(&self) -> &[Log] {
        &self.logs
    }
    // 分页倒序查询
    pub fn query_by_page(&self, page: &Page, max: u32) -> PageData<Log> {
        page_find_with_reserve(&self.logs, page, max)
    }
}
