use std::cmp::Ordering;

// ============= 分页查询 =============

// 分页对象
#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct QueryPage {
    pub page: u64, // 当前页码 1 开始计数
    pub size: u32, // 每页大小
}

#[derive(Debug)]
pub enum QueryPageError {
    WrongPage,                         // page can not be 0
    WrongSize { size: u32, max: u32 }, // size can not be 0 and has max value
}
impl std::fmt::Display for QueryPageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryPageError::WrongPage => write!(f, "page can not be 0"),
            QueryPageError::WrongSize { size, max } => {
                if *size == 0 {
                    write!(f, "size can not be 0")
                } else {
                    write!(f, "max({max}) < size({size})")
                }
            }
        }
    }
}
impl std::error::Error for QueryPageError {}

// 分页查询结果
#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct PageData<T> {
    pub page: u64,    // 请求的页码
    pub size: u32,    // 请求的页面大小
    pub total: u64,   // 总个数
    pub data: Vec<T>, // 查到的分页数据
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
    #[inline]
    pub fn empty<T>(&self) -> PageData<T> {
        PageData {
            page: self.page,
            size: self.size,
            total: 0,
            data: Vec::new(),
        }
    }

    // 检查分页选项是否有效
    #[inline]
    pub fn check(&self, max: u32) -> Result<(), QueryPageError> {
        if self.page == 0 {
            return Err(QueryPageError::WrongPage);
        }
        if self.size == 0 || max < self.size {
            return Err(QueryPageError::WrongSize {
                size: self.size,
                max,
            });
        }
        Ok(())
    }

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
    fn inner_query_by_list<'a, T>(
        &self,
        list: &'a [T],
        max: u32,
    ) -> Result<Vec<&'a T>, QueryPageError> {
        self.check(max)?;

        if list.is_empty() {
            return Ok(Vec::new());
        }

        let mut data = Vec::with_capacity(self.size as usize);

        // 偏移序号
        let start = ((self.page - 1) * self.size as u64) as usize;
        let end = ((self.page) * self.size as u64) as usize;

        if end < list.len() {
            data = list[start..end].iter().collect();
        } else if start < list.len() {
            data = list[start..].iter().collect();
        }

        Ok(data)
    }

    // 对所有数据进行分页查询
    #[inline]
    pub fn query_by_list<'a, T>(
        &self,
        list: &'a [T],
        max: u32,
    ) -> Result<PageData<&'a T>, QueryPageError> {
        let total = list.len() as u64;

        let data = self.inner_query_by_list(list, max)?;

        Ok(self.from_data(total, data))
    }

    // 对所有数据进行倒序分页查询
    #[inline]
    pub fn query_desc_by_list<'a, T>(
        &self,
        list: &'a [T],
        max: u32,
    ) -> Result<PageData<&'a T>, QueryPageError> {
        // 取出倒序索引
        let index_list: Vec<usize> = (0..list.len()).rev().collect();

        let total = index_list.len() as u64;

        let data = self.inner_query_by_list(&index_list, max)?;

        let data = data.into_iter().map(|i| &list[*i]).collect::<Vec<_>>();

        Ok(self.from_data(total, data))
    }

    /// 倒序过滤分页查询
    #[inline]
    pub fn query_desc_by_list_and_filter<'a, T, F>(
        &self,
        list: &'a [T],
        max: u32,
        filter: F, // 过滤条件
    ) -> Result<PageData<&'a T>, QueryPageError>
    where
        F: Fn(&T) -> bool,
    {
        // 取出过滤后的倒序索引
        let index_list: Vec<usize> = (0..list.len())
            .filter(|i| filter(&list[*i]))
            .rev()
            .collect();

        let total = index_list.len() as u64;

        let data = self.inner_query_by_list(&index_list, max)?;

        let data = data.into_iter().map(|i| &list[*i]).collect::<Vec<_>>();

        Ok(self.from_data(total, data))
    }

    /// 按条件分页查询
    #[inline]
    pub fn custom_query_by_list<T, R, Filter, Compare, Transform>(
        &self,
        list: &[T],
        max: u32,
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

        let data = self.inner_query_by_list(&list, max)?;

        let data = data.into_iter().map(|t| transform(t)).collect::<Vec<_>>();

        Ok(self.from_data(total, data))
    }
}
