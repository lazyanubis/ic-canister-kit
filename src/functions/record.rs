use crate::{
    identity::CallerId,
    types::{PageData, QueryPage, QueryPageError},
};

/// 记录 id
#[derive(
    candid::CandidType,
    candid::Deserialize,
    Copy,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
)]
pub struct RecordId(u64);

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

/// 迁移内容
#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct MigratedRecords<Record> {
    /// 一共被删除的记录个数
    pub removed: u64,
    /// 当前记录个数
    pub next_id: u64,
    /// 本次迁移的记录
    pub records: Vec<Record>,
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
    fn record_push(
        &mut self,
        caller: CallerId,
        topic: impl Into<RecordTopic>,
        content: impl Into<String>,
    ) -> RecordId;
    /// 更新记录
    fn record_update(&mut self, record_id: RecordId, done: impl Into<String>);
    /// 迁移
    fn record_migrate(&mut self, max: u32) -> MigratedRecords<Record>;

    /// 分页查询
    fn record_find_by_page(
        &self,
        page: &QueryPage,
        max: u32,
        search: &Option<Search>,
    ) -> Result<PageData<&Record>, QueryPageError> {
        let list = self.record_find_all();
        if let Some(search) = search {
            return page.query_desc_by_list_and_filter(list, max, |item| search.test(item));
        }
        page.query_desc_by_list(list, max)
    }
    /// 插入记录并更新记录
    fn record_push_by<R, F: FnOnce() -> (R, String)>(
        &mut self,
        caller: CallerId,
        topic: impl Into<RecordTopic>,
        content: impl Into<String>,
        f: F,
    ) -> R {
        let record_id = self.record_push(caller, topic, content);
        let (r, d) = f();
        self.record_update(record_id, d);
        r
    }
}
