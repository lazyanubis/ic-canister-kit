use std::collections::HashSet;
use std::fmt::Display;

use crate::canister::call::call_canister;
use crate::common::pages::{
    page_find_with_reserve, page_find_with_reserve_and_filter, Page, PageData,
};
use crate::identity::{caller, CallerId, CanisterId};
use crate::times::schedulable::async_execute;
use crate::times::Timestamp;

/// 日志记录

// 日志等级
#[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq, Clone)]
pub enum RecordLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

// 每条日志记录
#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct Record {
    pub id: u64,            // 日志记录
    pub created: Timestamp, // 时间戳 纳秒
    pub level: RecordLevel, // 日志级别
    pub caller: CallerId,   // 调用人
    pub topic: String,      // 日志主题
    pub content: String,    // 日志内容
    pub done: Timestamp,    // 完成时间
    pub result: String,     // 执行结果
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct MigratedRecords {
    pub topics: HashSet<String>,
    pub data: Vec<Record>,
    pub updated: Vec<(u64, Timestamp, String)>,
}

pub trait Recordable {
    // 查询
    fn record_topics(&self) -> HashSet<String>;
    fn record_find_all(&self, topic: &Option<String>) -> Vec<Record>;
    fn record_find_by_page(
        &self,
        topic: &Option<String>,
        page: &Page,
        max: u32,
    ) -> PageData<Record>;
    fn record_collector_find(&self) -> Option<CanisterId>;
    // 通知
    fn record_register(&self);
    // 修改
    fn record_push(
        &mut self,
        level: RecordLevel,
        caller: CallerId,
        topic: String,
        content: String,
    ) -> u64;
    fn record_update(&mut self, record_id: u64, result: String);

    fn record_trace(&mut self, topic: String, content: String) -> u64 {
        self.record_push(RecordLevel::Trace, caller(), topic, content)
    }
    fn record_debug(&mut self, topic: String, content: String) -> u64 {
        self.record_push(RecordLevel::Debug, caller(), topic, content)
    }
    fn record_info(&mut self, topic: String, content: String) -> u64 {
        self.record_push(RecordLevel::Info, caller(), topic, content)
    }
    fn record_warn(&mut self, topic: String, content: String) -> u64 {
        self.record_push(RecordLevel::Warn, caller(), topic, content)
    }
    fn record_error(&mut self, topic: String, content: String) -> u64 {
        self.record_push(RecordLevel::Error, caller(), topic, content)
    }
    // 迁移
    fn record_collector_replace(&mut self, collector: Option<CanisterId>);
    fn record_migrate_data(&mut self, max: u32) -> Vec<Record>;
    fn record_migrate_updated(&mut self) -> Vec<(u64, Timestamp, String)>;
    fn record_migrate(&mut self, max: u32) -> MigratedRecords {
        MigratedRecords {
            topics: self.record_topics(),
            data: self.record_migrate_data(max),
            updated: self.record_migrate_updated(),
        }
    }
}

// 持久化的日志记录对象
#[derive(candid::CandidType, candid::Deserialize, Debug, Default)]
pub struct Records {
    collector: Option<CanisterId>, // 日志收集者
    pub topics: HashSet<String>,   // 所有主题
    next_id: u64,                  // 下一个未使用的 id
    pub data: Vec<Record>,
    pub updated: Vec<(u64, Timestamp, String)>, // 如果更新失败了, 需要记录给日志收集者处理
}

impl Records {
    pub fn new() -> Self {
        Records {
            collector: None,        // 日志收集者
            topics: HashSet::new(), // 所有主题
            next_id: 0,             // 下一个未使用的 id
            data: Vec::new(),
            updated: Vec::new(), // 如果更新失败了, 需要记录给日志收集者处理
        }
    }
}

impl Recordable for Records {
    // 查询

    fn record_topics(&self) -> HashSet<String> {
        self.topics.clone()
    }

    // 查询所有 正序
    fn record_find_all(&self, topic: &Option<String>) -> Vec<Record> {
        if let Some(topic) = topic {
            let data: Vec<Record> = self
                .data
                .iter()
                .filter(|d| &d.topic == topic)
                .map(|r| r.clone())
                .collect();
            return data;
        }
        self.data.clone()
    }

    // 分页倒序查询
    fn record_find_by_page(
        &self,
        topic: &Option<String>,
        page: &Page,
        max: u32,
    ) -> PageData<Record> {
        if let Some(topic) = topic {
            return page_find_with_reserve_and_filter(&self.data, page, max, |d| &d.topic == topic);
        }
        page_find_with_reserve(&self.data, page, max)
    }

    fn record_collector_find(&self) -> Option<CanisterId> {
        self.collector
    }

    // 通知
    // 如果需要通知宿主
    fn record_register(&self) {
        if let Some(collector) = self.collector {
            let _ = async_execute(move || {
                ic_cdk::spawn(async move {
                    call_canister::<(), ()>(collector, "business_record_register", ())
                        .await
                        .unwrap();
                })
            });
        }
    }

    // 修改
    // 插入
    fn record_push(
        &mut self,
        level: RecordLevel,
        caller: CallerId,
        topic: String,
        content: String,
    ) -> u64 {
        let id = self.next_id;

        self.next_id += 1;

        self.data.push(Record {
            id,
            created: crate::times::now(),
            level,
            caller,
            topic: topic.clone(),
            content,
            done: 0,
            result: String::default(),
        });

        self.topics.insert(topic);

        id
    }

    // 更新
    fn record_update(&mut self, record_id: u64, result: String) {
        let now = crate::times::now();
        let mut i = self.data.len();
        while 0 < i {
            let record = &mut self.data[i - 1];
            if record.id == record_id {
                record.done = now;
                record.result = result;
                return;
            }
            i -= 1;
        }
        self.updated.push((record_id, now, result));
    }

    // 迁移
    fn record_collector_replace(&mut self, collector: Option<CanisterId>) {
        self.collector = collector;
        self.record_register();
    }

    fn record_migrate_data(&mut self, max: u32) -> Vec<Record> {
        if self.data.len() < max as usize {
            std::mem::take(&mut self.data)
        } else {
            let (migrated, left) = self.data.split_at(max as usize);
            let migrated = migrated.to_owned();
            self.data = left.to_owned();
            migrated
        }
    }

    fn record_migrate_updated(&mut self) -> Vec<(u64, Timestamp, String)> {
        std::mem::take(&mut self.updated)
    }
}

pub fn record_option<T: Display>(value: &Option<T>) -> String {
    if let Some(id) = value {
        return id.to_string();
    }
    format!("None")
}
