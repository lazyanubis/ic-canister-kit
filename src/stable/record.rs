use std::collections::HashSet;

use crate::{
    functions::{
        record::MigratedRecords,
        types::{RecordId, Recordable, Searchable},
    },
    identity::CallerId,
    types::TimestampNanos,
};

/// 记录主题
pub type RecordTopic = u8;

/// 每条日志记录
#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct Record {
    /// 记录 id
    pub id: RecordId,
    /// 创建时间戳 纳秒
    pub created: TimestampNanos,
    /// 调用人
    pub caller: CallerId,
    /// 记录主题
    pub topic: RecordTopic,
    /// 记录内容
    pub content: String,
    /// 完成时间 完成结果
    pub done: Option<(TimestampNanos, String)>,
}

impl Record {
    #[inline]
    fn same(&self, id: &RecordId) -> bool {
        self.id == *id
    }

    #[inline]
    fn update(&mut self, done: impl Into<String>) {
        self.done = Some((crate::times::now(), done.into()));
    }
}

/// 记录检索
#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct RecordSearch {
    /// id 过滤
    pub id: Option<(Option<RecordId>, Option<RecordId>)>,
    /// 创建时间过滤
    pub created: Option<(Option<TimestampNanos>, Option<TimestampNanos>)>,
    /// 调用人过滤
    pub caller: Option<HashSet<CallerId>>,
    /// 日志主题过滤
    pub topic: Option<HashSet<RecordTopic>>,
    /// 日志内容过滤
    pub content: Option<String>,
}

impl Searchable<Record> for RecordSearch {
    #[allow(unused)]
    #[inline]
    fn test(&self, record: &Record) -> bool {
        if let Some((id_min, id_max)) = &self.id {
            if let Some(id_min) = &id_min {
                if record.id < *id_min {
                    return false;
                }
            }
            if let Some(id_max) = &id_max {
                if *id_max < record.id {
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

/// 持久化的日志记录对象
#[derive(candid::CandidType, candid::Deserialize, Debug, Default)]
pub struct Records {
    /// 最大保存个数
    pub max: u64,
    /// 删除的个数
    pub removed: u64,
    /// 下一个未使用的 id
    pub next_id: RecordId,
    /// 当前保存的个数
    pub records: Vec<Record>,
}

impl Recordable<Record, RecordTopic, RecordSearch> for Records {
    // 查询

    // 查询所有 正序
    fn record_find_all(&self) -> &[Record] {
        &self.records
    }

    // 修改
    fn record_push(
        &mut self,
        caller: CallerId,
        topic: impl Into<RecordTopic>,
        content: impl Into<String>,
    ) -> RecordId {
        // 判断最大个数
        if self.max <= self.records.len() as u64 {
            let (_migrated, left) = self.records.split_at(1);
            self.records = left.to_owned();
            self.removed += 1;
        }

        let id = self.next_id;

        self.next_id = self.next_id.next();

        self.records.push(Record {
            id,
            created: crate::times::now(),
            caller,
            topic: topic.into(),
            content: content.into(),
            done: None,
        });

        id
    }

    /// 更新记录
    fn record_update(&mut self, record_id: RecordId, done: impl Into<String>) {
        let list = &mut self.records;
        let mut index = list.len();
        loop {
            index -= 1;
            if let Some(item) = list.get_mut(index) {
                if item.same(&record_id) {
                    item.update(done);
                    break;
                }
            } else {
                break;
            }
            if index == 0 {
                break;
            }
        }
    }

    // 迁移
    fn record_migrate(&mut self, max: u32) -> MigratedRecords<Record> {
        let removed = self.removed;
        let next_id = self.next_id.into_inner();
        let records = if self.records.len() < max as usize {
            std::mem::take(&mut self.records) // 全部取走
        } else {
            let (migrated, left) = self.records.split_at(max as usize);
            let migrated = migrated.to_owned();
            self.records = left.to_owned();
            migrated
        };
        MigratedRecords {
            removed,
            next_id,
            records,
        }
    }
}
