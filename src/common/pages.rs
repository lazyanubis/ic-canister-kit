use std::cmp::Ordering;

use candid::CandidType;
use serde::{Deserialize, Serialize};

// ============= 分页查询 =============

/// 分页对象
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct QueryPage {
    /// 当前页码 1 开始计数
    pub page: u64,
    /// 每页大小
    pub size: u32,
}

/// 分页查询错误
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum QueryPageError {
    /// 错误的页码，不能为 0
    WrongPage, // page can not be 0

    /// 错误的页面大小
    WrongSize {
        /// 页面大小
        size: u32,
        /// 最大页面大小
        #[serde(alias = "max")]
        max_page_size: u32,
    }, // size can not be 0 and has max value
}
impl std::fmt::Display for QueryPageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryPageError::WrongPage => write!(f, "page can not be 0"),
            QueryPageError::WrongSize { size, max_page_size } => {
                if *size == 0 {
                    write!(f, "size can not be 0")
                } else {
                    write!(f, "max_page_size({max_page_size}) < size({size})")
                }
            }
        }
    }
}
impl std::error::Error for QueryPageError {}

/// 分页查询结果
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PageData<T> {
    /// 请求的页码
    pub page: u64,
    /// 请求的页面大小
    pub size: u32,
    /// 总个数
    pub total: u64,
    /// 查到的分页数据
    pub data: Vec<T>,
}

impl<T: Clone> From<PageData<&T>> for PageData<T> {
    fn from(value: PageData<&T>) -> Self {
        PageData {
            page: value.page,
            size: value.size,
            total: value.total,
            data: value.data.into_iter().cloned().collect(),
        }
    }
}

// 空结果
impl QueryPage {
    /// 空结果
    #[inline]
    pub fn empty<T>(&self) -> PageData<T> {
        PageData {
            page: self.page,
            size: self.size,
            total: 0,
            data: Vec::new(),
        }
    }

    /// 检查分页选项是否有效
    #[inline]
    pub fn check(&self, max_page_size: u32) -> Result<(), QueryPageError> {
        if self.page == 0 {
            return Err(QueryPageError::WrongPage);
        }
        if self.size == 0 || max_page_size < self.size {
            return Err(QueryPageError::WrongSize {
                size: self.size,
                max_page_size,
            });
        }
        Ok(())
    }

    /// 分页数据对象
    #[inline]
    pub fn from_data<T>(&self, total: u64, data: Vec<T>) -> PageData<T> {
        PageData {
            page: self.page,
            size: self.size,
            total,
            data,
        }
    }

    #[inline]
    fn page_start(&self) -> Option<usize> {
        let start = (self.page - 1).checked_mul(u64::from(self.size))?;
        usize::try_from(start).ok()
    }

    #[inline]
    fn page_window(&self, total_items: usize) -> Option<(usize, usize)> {
        let start = self.page_start()?;
        if total_items <= start {
            return None;
        }

        let end = start.saturating_add(self.size as usize).min(total_items);
        Some((start, end))
    }

    #[inline]
    fn inner_query_by_list<'a, T>(&self, list: &'a [T], max_page_size: u32) -> Result<Vec<&'a T>, QueryPageError> {
        self.check(max_page_size)?;

        if list.is_empty() {
            return Ok(Vec::new());
        }

        let Some((start, end)) = self.page_window(list.len()) else {
            return Ok(Vec::new());
        };

        Ok(list[start..end].iter().collect())
    }

    /// 对所有数据进行分页查询
    #[inline]
    pub fn query_by_list<'a, T>(&self, list: &'a [T], max_page_size: u32) -> Result<PageData<&'a T>, QueryPageError> {
        let total_items = list.len() as u64;

        let data = self.inner_query_by_list(list, max_page_size)?;

        Ok(self.from_data(total_items, data))
    }

    /// 对所有数据进行倒序分页查询
    #[inline]
    pub fn query_desc_by_list<'a, T>(
        &self,
        list: &'a [T],
        max_page_size: u32,
    ) -> Result<PageData<&'a T>, QueryPageError> {
        self.check(max_page_size)?;
        let total_items = list.len() as u64;
        let data = self
            .page_window(list.len())
            .map(|(start, end)| list.iter().rev().skip(start).take(end - start).collect())
            .unwrap_or_default();

        Ok(self.from_data(total_items, data))
    }

    /// 倒序过滤分页查询
    #[inline]
    pub fn query_desc_by_list_and_filter<'a, T, F>(
        &self,
        list: &'a [T],
        max_page_size: u32,
        filter: F, // 过滤条件
    ) -> Result<PageData<&'a T>, QueryPageError>
    where
        F: Fn(&T) -> bool,
    {
        self.check(max_page_size)?;
        let start = self.page_start();
        let mut total_items = 0_usize;
        let mut data = Vec::with_capacity((self.size as usize).min(list.len()));
        for item in list.iter().rev() {
            if filter(item) {
                if start.is_some_and(|start| start <= total_items) && data.len() < self.size as usize {
                    data.push(item);
                }
                total_items += 1;
            }
        }

        Ok(self.from_data(total_items as u64, data))
    }

    /// 按条件分页查询
    #[inline]
    pub fn custom_query_by_list<T, R, Filter, Compare, Transform>(
        &self,
        list: &[T],
        max_page_size: u32,
        filter: Filter,       // 过滤条件
        compare: Compare,     // 排序方法
        transform: Transform, // 变形方法
    ) -> Result<PageData<R>, QueryPageError>
    where
        Filter: Fn(&T) -> bool,
        Compare: Fn(&T, &T) -> Ordering,
        Transform: Fn(&T) -> R,
    {
        // 1. 过滤有效的结果
        let mut list: Vec<&T> = list.iter().filter(|&item| filter(item)).collect();

        // 2. 进行排序
        list.sort_by(|&a, &b| compare(a, b));

        let total = list.len() as u64;

        let data = self.inner_query_by_list(&list, max_page_size)?;

        let data = data.into_iter().map(|t| transform(t)).collect::<Vec<_>>();

        Ok(self.from_data(total, data))
    }
}

#[cfg(test)]
mod tests {
    use ciborium::value::Value;
    use serde::Serialize;

    use super::{PageData, QueryPage, QueryPageError};

    #[test]
    fn rejects_zero_page_and_invalid_size() {
        assert!(matches!(
            QueryPage { page: 0, size: 1 }.check(10),
            Err(QueryPageError::WrongPage)
        ));
        assert!(matches!(
            QueryPage { page: 1, size: 0 }.check(10),
            Err(QueryPageError::WrongSize { .. })
        ));
        assert!(matches!(
            QueryPage { page: 1, size: 11 }.check(10),
            Err(QueryPageError::WrongSize { .. })
        ));
    }

    #[test]
    fn returns_empty_for_overflowing_or_out_of_range_page() {
        let data = [1, 2, 3];
        let overflow = QueryPage {
            page: u64::MAX,
            size: 10,
        };
        assert!(overflow.query_by_list(&data, 10).unwrap().data.is_empty());

        let out_of_range = QueryPage { page: 3, size: 2 };
        assert!(out_of_range.query_by_list(&data, 10).unwrap().data.is_empty());
    }

    #[test]
    fn paginates_forward_reverse_and_filtered_data() {
        use std::cell::Cell;

        let data = [1, 2, 3, 4, 5];
        let page = QueryPage { page: 2, size: 2 };

        assert_eq!(page.query_by_list(&data, 10).unwrap().data, vec![&3, &4]);
        assert_eq!(page.query_desc_by_list(&data, 10).unwrap().data, vec![&3, &2]);

        let calls = Cell::new(0);
        let filtered = page.query_desc_by_list_and_filter(&data, 10, |value| {
            calls.set(calls.get() + 1);
            value % 2 == 1
        });
        let filtered = filtered.unwrap();
        assert_eq!(filtered.total, 3);
        assert_eq!(filtered.data, vec![&1]);
        assert_eq!(calls.get(), data.len());
    }

    #[test]
    fn preserves_page_total_and_deserializes_legacy_error_name() {
        #[derive(Serialize)]
        struct LegacyPageData {
            page: u64,
            size: u32,
            total: u64,
            data: Vec<u8>,
        }

        #[derive(Serialize)]
        enum LegacyQueryPageError {
            WrongSize { size: u32, max: u32 },
        }

        let mut legacy_page_cbor = Vec::new();
        ciborium::ser::into_writer(
            &LegacyPageData {
                page: 1,
                size: 10,
                total: 2,
                data: vec![1, 2],
            },
            &mut legacy_page_cbor,
        )
        .unwrap();
        let page: PageData<u8> = ciborium::de::from_reader(legacy_page_cbor.as_slice()).unwrap();
        assert_eq!(page.total, 2);

        let mut current_page_cbor = Vec::new();
        ciborium::ser::into_writer(&page, &mut current_page_cbor).unwrap();
        let current: Value = ciborium::de::from_reader(current_page_cbor.as_slice()).unwrap();
        let Value::Map(entries) = current else {
            panic!("expected a CBOR map")
        };
        let keys: Vec<&str> = entries
            .iter()
            .filter_map(|(key, _)| match key {
                Value::Text(key) => Some(key.as_str()),
                _ => None,
            })
            .collect();
        assert!(keys.contains(&"total"));
        assert!(!keys.contains(&"total_items"));

        let mut legacy_error_cbor = Vec::new();
        ciborium::ser::into_writer(
            &LegacyQueryPageError::WrongSize { size: 11, max: 10 },
            &mut legacy_error_cbor,
        )
        .unwrap();
        let error: QueryPageError = ciborium::de::from_reader(legacy_error_cbor.as_slice()).unwrap();
        assert!(matches!(
            error,
            QueryPageError::WrongSize {
                size: 11,
                max_page_size: 10
            }
        ));

        let mut current_error_cbor = Vec::new();
        ciborium::ser::into_writer(
            &QueryPageError::WrongSize {
                size: 11,
                max_page_size: 10,
            },
            &mut current_error_cbor,
        )
        .unwrap();
        let current_error: Value = ciborium::de::from_reader(current_error_cbor.as_slice()).unwrap();
        let Value::Map(variant) = current_error else {
            panic!("expected a CBOR enum map")
        };
        let Value::Map(fields) = &variant[0].1 else {
            panic!("expected WrongSize fields")
        };
        let keys: Vec<&str> = fields
            .iter()
            .filter_map(|(key, _)| match key {
                Value::Text(key) => Some(key.as_str()),
                _ => None,
            })
            .collect();
        assert!(keys.contains(&"max_page_size"));
        assert!(!keys.contains(&"max"));
    }
}
