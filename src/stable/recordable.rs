use std::collections::HashSet;
use std::fmt::Display;

use crate::canister::call::call_canister;
use crate::common::pages::{
    page_find_with_reserve, page_find_with_reserve_and_filter, Page, PageData,
};
use crate::identity::{caller, CallerId, CanisterId};
use crate::times::schedulable::async_execute;
use crate::times::TimestampNanos;

/// 日志记录

// 日志等级
#[derive(candid::CandidType, candid::Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
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
    pub id: u64,                 // 日志 id
    pub created: TimestampNanos, // 时间戳 纳秒
    pub level: RecordLevel,      // 日志级别
    pub caller: CallerId,        // 调用人
    pub topic: String,           // 日志主题
    pub content: String,         // 日志内容
    pub done: TimestampNanos,    // 完成时间
    pub result: String,          // 执行结果
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct RecordSearch {
    pub id: Option<(Option<u64>, Option<u64>)>, // id 过滤
    pub created: Option<(Option<TimestampNanos>, Option<TimestampNanos>)>, // 创建时间过滤
    pub level: Option<HashSet<RecordLevel>>,    // 日志级别过滤
    pub caller: Option<HashSet<CallerId>>,      // 调用人过滤
    pub topic: Option<HashSet<String>>,         // 日志主题过滤
    pub content: Option<String>,                // 日志内容过滤
}

impl RecordSearch {
    fn test(&self, record: &Record) -> bool {
        if let Some(id) = self.id {
            let (id_min, id_max) = id;
            if let Some(id_min) = id_min {
                if record.id < id_min {
                    return false;
                }
            }
            if let Some(id_max) = id_max {
                if id_max < record.id {
                    return false;
                }
            }
        }
        if let Some(created) = self.created {
            let (created_min, created_max) = created;
            if let Some(created_min) = created_min {
                if record.created < created_min {
                    return false;
                }
            }
            if let Some(created_max) = created_max {
                if created_max < record.created {
                    return false;
                }
            }
        }
        if let Some(level) = &self.level {
            if !level.contains(&record.level) {
                return false;
            }
        }
        if let Some(caller) = &self.caller {
            if !caller.contains(&record.caller) {
                return false;
            }
        }
        if let Some(topic) = &self.topic {
            if !topic.contains(&record.topic) {
                return false;
            }
        }
        if let Some(content) = &self.content {
            if !record.content.contains(content) {
                return false;
            }
        }
        true
    }
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct MigratedRecords {
    pub topics: HashSet<String>,
    pub next_id: u64,
    pub records: Vec<Record>,
    pub updated: Vec<(u64, TimestampNanos, String)>,
}

pub trait Recordable {
    // 查询
    fn record_collector_find(&self) -> Option<CanisterId>;
    fn record_topics(&self) -> &HashSet<String>;
    fn record_find_all(&self, search: &Option<RecordSearch>) -> Vec<&Record>;
    fn record_find_by_page(
        &self,
        search: &Option<RecordSearch>,
        page: &Page,
        max: u32,
    ) -> PageData<&Record>;
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
    fn record_collector_update(&mut self, collector: Option<CanisterId>, notice: bool);
    fn record_migrate(&mut self, max: u32) -> MigratedRecords;
}

// 持久化的日志记录对象
#[derive(candid::CandidType, candid::Deserialize, Debug, Default)]
pub struct Records {
    collector: Option<CanisterId>, // 日志收集者
    pub topics: HashSet<String>,   // 所有主题
    pub next_id: u64,              // 下一个未使用的 id
    pub records: Vec<Record>,
    pub updated: Vec<(u64, TimestampNanos, String)>, // 如果更新失败了, 需要记录给日志收集者处理
}

impl Records {
    pub fn new() -> Self {
        Records {
            collector: None,        // 日志收集者
            topics: HashSet::new(), // 所有主题
            next_id: 0,             // 下一个未使用的 id
            records: Vec::new(),
            updated: Vec::new(), // 如果更新失败了, 需要记录给日志收集者处理
        }
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
}

impl Recordable for Records {
    // 查询
    fn record_topics(&self) -> &HashSet<String> {
        &self.topics
    }
    // 查询所有 正序
    fn record_find_all(&self, search: &Option<RecordSearch>) -> Vec<&Record> {
        if let Some(search) = search {
            let records: Vec<&Record> = self
                .records
                .iter()
                .filter(|record| search.test(record))
                .collect();
            return records;
        }
        self.records.iter().collect()
    }
    fn record_find_by_page(
        &self,
        search: &Option<RecordSearch>,
        page: &Page,
        max: u32,
    ) -> PageData<&Record> {
        if let Some(search) = search {
            return page_find_with_reserve_and_filter(&self.records, page, max, |record| {
                search.test(record)
            });
        }
        page_find_with_reserve(&self.records, page, max)
    }

    fn record_collector_find(&self) -> Option<CanisterId> {
        self.collector
    }

    // 修改
    fn record_push(
        &mut self,
        level: RecordLevel,
        caller: CallerId,
        topic: String,
        content: String,
    ) -> u64 {
        let id = self.next_id;

        self.next_id += 1;

        self.records.push(Record {
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
    fn record_update(&mut self, record_id: u64, result: String) {
        let now = crate::times::now();
        let mut i = self.records.len();
        while 0 < i {
            let record = &mut self.records[i - 1];
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
    fn record_collector_update(&mut self, collector: Option<CanisterId>, notice: bool) {
        self.collector = collector;
        if notice {
            self.record_register();
        }
    }
    fn record_migrate(&mut self, max: u32) -> MigratedRecords {
        let topics = self.record_topics().clone();
        let next_id = self.next_id;
        let records = if self.records.len() < max as usize {
            std::mem::take(&mut self.records)
        } else {
            let (migrated, left) = self.records.split_at(max as usize);
            let migrated = migrated.to_owned();
            self.records = left.to_owned();
            migrated
        };
        let updated = std::mem::take(&mut self.updated);
        MigratedRecords {
            topics,
            next_id,
            records,
            updated,
        }
    }
}

pub fn format_option<T: Display>(value: &Option<T>) -> String {
    if let Some(value) = value {
        return value.to_string();
    }
    format!("None")
}

pub fn format_option_with_func<T, F: Fn(&T) -> String>(value: &Option<T>, f: F) -> String {
    if let Some(value) = value {
        return f(value);
    }
    format!("None")
}
