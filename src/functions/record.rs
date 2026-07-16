use std::collections::HashSet;

use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    identity::CallerId,
    types::{PageData, QueryPage, QueryPageError},
};

/// 记录 id
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordId(u64);

impl From<u64> for RecordId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl RecordId {
    /// 取出内部数据
    pub fn into_inner(&self) -> u64 {
        self.0
    }
    /// 下一个 id
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// 查询
pub trait Searchable<Record> {
    /// 查询
    fn test(&self, record: &Record) -> bool;
}

/// 可以记录的操作
pub trait Recordable<Record, RecordTopic, Search: Searchable<Record>> {
    // 查询
    /// 查询所有
    fn record_find_all(&self) -> &[Record];

    // 修改
    /// 插入记录
    fn record_push(&mut self, caller: CallerId, topic: RecordTopic, content: String) -> RecordId;
    /// 更新记录
    fn record_update(&mut self, record_id: RecordId, result: String);
    /// 按 id 批量删除记录，返回实际删除的记录数量
    fn record_delete(&mut self, ids: &HashSet<RecordId>) -> u64;

    /// 分页查询
    fn record_find_by_page(
        &self,
        page: &QueryPage,
        max_page_size: u32,
        search: &Option<Search>,
    ) -> Result<PageData<&Record>, QueryPageError> {
        let list = self.record_find_all();
        if let Some(search) = search {
            return page.query_desc_by_list_and_filter(list, max_page_size, |item| search.test(item));
        }
        page.query_desc_by_list(list, max_page_size)
    }
}

// ================== 简单实现 ==================

/// 记录功能简单实现
pub mod basic {
    use std::collections::HashSet;

    use candid::CandidType;
    use serde::{Deserialize, Serialize};

    use crate::{
        functions::types::{RecordId, Recordable, Searchable},
        identity::CallerId,
        types::TimestampNanos,
    };

    /// 记录主题
    pub type RecordTopic = u8;

    /// 每条记录
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
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
        /// 完成时间与执行结果
        #[serde(alias = "done")]
        pub completion: Option<(TimestampNanos, String)>,
    }

    impl Record {
        #[inline]
        fn same(&self, id: &RecordId) -> bool {
            self.id == *id
        }
    }

    /// 记录检索
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
    pub struct RecordSearch {
        /// id 范围过滤，依次为包含下界和包含上界
        #[serde(alias = "id")]
        pub id_range: Option<(Option<RecordId>, Option<RecordId>)>,
        /// 创建时间纳秒范围过滤，依次为包含下界和包含上界
        #[serde(alias = "created")]
        pub created_at_nanos_range: Option<(Option<TimestampNanos>, Option<TimestampNanos>)>,
        /// 调用人过滤
        pub caller: Option<HashSet<CallerId>>,
        /// 主题过滤
        pub topic: Option<HashSet<RecordTopic>>,
        /// 内容过滤
        pub content: Option<String>,
    }

    impl Searchable<Record> for RecordSearch {
        #[allow(unused)]
        #[inline]
        fn test(&self, record: &Record) -> bool {
            if let Some((id_min, id_max)) = &self.id_range {
                if let Some(id_min) = &id_min
                    && record.id < *id_min
                {
                    return false;
                }
                if let Some(id_max) = &id_max
                    && *id_max < record.id
                {
                    return false;
                }
            }
            if let Some(created) = self.created_at_nanos_range {
                let (created_min, created_max) = created;
                if let Some(created_min) = created_min
                    && record.created < created_min
                {
                    return false;
                }
                if let Some(created_max) = created_max
                    && created_max < record.created
                {
                    return false;
                }
            }
            if let Some(caller) = &self.caller
                && !caller.contains(&record.caller)
            {
                return false;
            }
            if let Some(topic) = &self.topic
                && !topic.contains(&record.topic)
            {
                return false;
            }
            if let Some(content) = &self.content
                && !record.content.contains(content)
            {
                return false;
            }
            true
        }
    }

    /// 持久化的记录对象
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
    pub struct Records {
        /// 最多保留的记录条数
        #[serde(alias = "max")]
        pub retention_limit: u64,
        /// 因保留上限被累计淘汰的记录数量
        #[serde(alias = "removed")]
        pub retention_evicted_count: u64,
        /// 下一个未使用的 id
        pub next_id: RecordId,
        /// 当前保留的记录列表
        pub records: Vec<Record>,
    }

    impl Default for Records {
        fn default() -> Self {
            Self {
                retention_limit: 1024 * 64, // 假设一条占用 1KB 则最大 64MB 记录
                retention_evicted_count: Default::default(),
                next_id: Default::default(),
                records: Default::default(),
            }
        }
    }

    impl Records {
        fn push_at(
            &mut self,
            caller: CallerId,
            topic: RecordTopic,
            content: String,
            created: TimestampNanos,
        ) -> RecordId {
            let id = self.next_id;
            self.next_id = self.next_id.next();

            if self.retention_limit == 0 {
                self.retention_evicted_count = self.retention_evicted_count.saturating_add(1);
                return id;
            }

            let retention_limit = usize::try_from(self.retention_limit).unwrap_or(usize::MAX);
            if retention_limit <= self.records.len() {
                let remove_count = self.records.len() - retention_limit + 1;
                self.records.drain(..remove_count);
                self.retention_evicted_count = self.retention_evicted_count.saturating_add(remove_count as u64);
            }

            self.records.push(Record {
                id,
                created,
                caller,
                topic,
                content,
                completion: None,
            });

            id
        }

        fn update_at(&mut self, record_id: RecordId, result: String, completed_at: TimestampNanos) {
            if let Some(item) = self.records.iter_mut().rev().find(|item| item.same(&record_id)) {
                item.completion = Some((completed_at, result));
            }
        }
    }

    impl Recordable<Record, RecordTopic, RecordSearch> for Records {
        // 查询

        // 查询所有 正序
        fn record_find_all(&self) -> &[Record] {
            &self.records
        }

        // 修改
        fn record_push(&mut self, caller: CallerId, topic: RecordTopic, content: String) -> RecordId {
            self.push_at(caller, topic, content, crate::times::now())
        }

        /// 更新记录
        fn record_update(&mut self, record_id: RecordId, result: String) {
            self.update_at(record_id, result, crate::times::now());
        }

        // 删除
        fn record_delete(&mut self, ids: &HashSet<RecordId>) -> u64 {
            if ids.is_empty() {
                return 0;
            }

            let before = self.records.len();
            self.records.retain(|record| !ids.contains(&record.id));
            u64::try_from(before - self.records.len()).unwrap_or(u64::MAX)
        }
    }

    /// 记录检索
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
    pub struct RecordSearchArg {
        /// id 范围过滤，依次为包含下界和包含上界
        #[serde(alias = "id")]
        pub id_range: Option<(Option<u64>, Option<u64>)>,
        /// 创建时间纳秒范围过滤，依次为包含下界和包含上界
        #[serde(alias = "created")]
        pub created_at_nanos_range: Option<(Option<u64>, Option<u64>)>,
        /// 调用人过滤
        pub caller: Option<HashSet<CallerId>>,
        /// 主题过滤
        pub topic: Option<HashSet<String>>,
        /// 内容过滤
        pub content: Option<String>,
    }

    impl RecordSearchArg {
        /// 参数转变
        pub fn into<E, F: Fn(&str) -> Result<RecordTopic, E>>(self, f: F) -> Result<RecordSearch, E> {
            Ok(RecordSearch {
                id_range: self.id_range.map(|(a, b)| (a.map(|a| a.into()), b.map(|b| b.into()))),
                created_at_nanos_range: self
                    .created_at_nanos_range
                    .map(|(a, b)| (a.map(|a| (a as i128).into()), b.map(|b| (b as i128).into()))),
                caller: self.caller,
                topic: self
                    .topic
                    .map(|topic| topic.iter().map(|t| f(t)).collect::<Result<HashSet<_>, _>>())
                    .transpose()?,
                content: self.content,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashSet;

        use candid::Principal;
        use ciborium::value::Value;
        use serde::Serialize;

        use super::{RecordSearchArg, Records};
        use crate::{
            functions::{record::RecordId, types::Recordable},
            types::TimestampNanos,
        };

        #[derive(Serialize)]
        struct LegacyRecord {
            id: RecordId,
            created: TimestampNanos,
            caller: Principal,
            topic: u8,
            content: String,
            done: Option<(TimestampNanos, String)>,
        }

        #[derive(Serialize)]
        struct LegacyRecords {
            max: u64,
            removed: u64,
            next_id: RecordId,
            records: Vec<LegacyRecord>,
        }

        #[derive(Serialize)]
        struct LegacyRecordSearchArg {
            id: Option<(Option<u64>, Option<u64>)>,
            created: Option<(Option<u64>, Option<u64>)>,
            caller: Option<HashSet<Principal>>,
            topic: Option<HashSet<String>>,
            content: Option<String>,
        }

        fn map_keys(value: &Value) -> Vec<&str> {
            let Value::Map(entries) = value else {
                panic!("expected a CBOR map")
            };
            entries
                .iter()
                .filter_map(|(key, _)| match key {
                    Value::Text(key) => Some(key.as_str()),
                    _ => None,
                })
                .collect()
        }

        fn push(records: &mut Records, value: &str, time: i128) -> RecordId {
            records.push_at(Principal::anonymous(), 1, value.to_string(), TimestampNanos::from(time))
        }

        #[test]
        fn zero_capacity_discards_records_without_panicking() {
            let mut records = Records {
                retention_limit: 0,
                ..Default::default()
            };

            let id = push(&mut records, "discarded", 1);
            records.update_at(id, "ignored".to_string(), TimestampNanos::from(2));

            assert!(records.records.is_empty());
            assert_eq!(records.retention_evicted_count, 1);
            assert_eq!(records.next_id.into_inner(), 1);
        }

        #[test]
        fn enforces_capacity_after_capacity_is_reduced() {
            let mut records = Records {
                retention_limit: 4,
                ..Default::default()
            };
            push(&mut records, "zero", 0);
            push(&mut records, "one", 1);
            push(&mut records, "two", 2);
            records.retention_limit = 2;

            let newest = push(&mut records, "three", 3);

            assert_eq!(records.records.len(), 2);
            assert_eq!(records.records[0].content, "two");
            assert_eq!(records.records[1].id, newest);
            assert_eq!(records.retention_evicted_count, 2);
        }

        #[test]
        fn updates_existing_record_and_ignores_missing_record() {
            let mut records = Records::default();
            records.update_at(RecordId::from(99), "missing".to_string(), TimestampNanos::from(1));

            let id = push(&mut records, "created", 2);
            records.update_at(id, "done".to_string(), TimestampNanos::from(3));

            assert_eq!(
                records.records[0].completion.as_ref().map(|(_, value)| value.as_str()),
                Some("done")
            );
        }

        #[test]
        fn deletes_only_requested_ids_and_is_safe_to_retry() {
            let mut records = Records::default();
            let first = push(&mut records, "first", 1);
            let second = push(&mut records, "second", 2);
            let third = push(&mut records, "third", 3);
            let ids = HashSet::from([first, third, RecordId::from(99)]);

            assert_eq!(records.record_delete(&ids), 2);
            assert_eq!(records.records.len(), 1);
            assert_eq!(records.records[0].id, second);
            assert_eq!(records.next_id.into_inner(), 3);
            assert_eq!(records.retention_evicted_count, 0);

            assert_eq!(records.record_delete(&ids), 0);
            assert_eq!(records.records.len(), 1);
        }

        #[test]
        fn deserializes_legacy_aliases_and_serializes_current_names() {
            let legacy = LegacyRecords {
                max: 42,
                removed: 3,
                next_id: RecordId::from(7),
                records: vec![LegacyRecord {
                    id: RecordId::from(6),
                    created: TimestampNanos::from(1),
                    caller: Principal::anonymous(),
                    topic: 1,
                    content: "legacy".to_string(),
                    done: Some((TimestampNanos::from(2), "ok".to_string())),
                }],
            };

            let mut cbor = Vec::new();
            ciborium::ser::into_writer(&legacy, &mut cbor).unwrap();
            let decoded: Records = ciborium::de::from_reader(cbor.as_slice()).unwrap();
            assert_eq!(decoded.retention_limit, 42);
            assert_eq!(decoded.retention_evicted_count, 3);
            assert_eq!(decoded.records[0].completion.as_ref().unwrap().1, "ok");

            let mut current_cbor = Vec::new();
            ciborium::ser::into_writer(&decoded, &mut current_cbor).unwrap();
            let current: Value = ciborium::de::from_reader(current_cbor.as_slice()).unwrap();
            let current_keys = map_keys(&current);
            assert!(current_keys.contains(&"retention_limit"));
            assert!(current_keys.contains(&"retention_evicted_count"));
            assert!(!current_keys.contains(&"max"));
            assert!(!current_keys.contains(&"removed"));

            let Value::Map(entries) = current else { unreachable!() };
            let records = entries
                .iter()
                .find_map(|(key, value)| (key == &Value::Text("records".to_string())).then_some(value))
                .unwrap();
            let Value::Array(records) = records else {
                panic!("expected records to be a CBOR array")
            };
            let record_keys = map_keys(&records[0]);
            assert!(record_keys.contains(&"completion"));
            assert!(!record_keys.contains(&"done"));

            let legacy_search = LegacyRecordSearchArg {
                id: Some((Some(1), Some(2))),
                created: Some((Some(3), Some(4))),
                caller: None,
                topic: None,
                content: None,
            };
            let mut search_cbor = Vec::new();
            ciborium::ser::into_writer(&legacy_search, &mut search_cbor).unwrap();
            let search: RecordSearchArg = ciborium::de::from_reader(search_cbor.as_slice()).unwrap();
            assert_eq!(search.id_range, Some((Some(1), Some(2))));
            assert_eq!(search.created_at_nanos_range, Some((Some(3), Some(4))));
            let mut current_search_cbor = Vec::new();
            ciborium::ser::into_writer(&search, &mut current_search_cbor).unwrap();
            let current_search: Value = ciborium::de::from_reader(current_search_cbor.as_slice()).unwrap();
            let search_keys = map_keys(&current_search);
            assert!(search_keys.contains(&"id_range"));
            assert!(search_keys.contains(&"created_at_nanos_range"));
            assert!(!search_keys.contains(&"id"));
            assert!(!search_keys.contains(&"created"));
        }
    }
}
