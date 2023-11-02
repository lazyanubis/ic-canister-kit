use std::cmp::Ordering;

// ============= 分页查询 =============

// 分页对象
#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct Page {
    pub page: u32, // 当前页码 1 开始计数
    pub size: u32, // 每页大小
}

// 分页查询结果
#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct PageData<T> {
    pub page: u32,    // 请求的页码
    pub size: u32,    // 请求的页面大小
    pub total: u32,   // 总个数
    pub data: Vec<T>, // 查到的分页数据
}

impl<T: Clone> From<PageData<&T>> for PageData<T> {
    fn from(value: PageData<&T>) -> Self {
        PageData {
            page: value.page,
            size: value.size,
            total: value.total,
            data: value.data.into_iter().map(|t| t.clone()).collect(),
        }
    }
}

// 空结果
impl Page {
    pub fn none<T>(&self) -> PageData<T> {
        PageData {
            page: self.page,
            size: self.size,
            total: 0,
            data: Vec::new(),
        }
    }
}

// 检查分页选项是否有效
pub fn page_check(page: &Page, max: u32) {
    if page.page == 0 {
        panic!("page can not be 0")
    }
    if max < page.size {
        panic!("max page size is {} < {:?}", max, page.size)
    }
}

/// 直接分页查询
pub fn page_find<T: Clone>(list: &Vec<T>, page: &Page, max: u32) -> PageData<T> {
    page_check(&page, max);

    let mut data = Vec::new();

    // 偏移序号
    let start = ((page.page - 1) * page.size) as usize;
    let end = ((page.page) * page.size) as usize;

    if end < list.len() {
        data = (&list[start..end]).iter().map(|t| t.clone()).collect();
    } else if start < list.len() {
        data = (&list[start..]).iter().map(|t| t.clone()).collect();
    }

    PageData {
        page: page.page,
        size: page.size,
        total: list.len() as u32,
        data,
    }
}

/// 倒序分页查询
pub fn page_find_with_reserve<'a, T>(list: &'a Vec<T>, page: &Page, max: u32) -> PageData<&'a T> {
    page_check(&page, max);

    if list.len() == 0 {
        return page.none();
    }

    // 取出所有的索引
    let mut index_list: Vec<usize> = (0..list.len()).into_iter().collect();
    index_list.reverse(); // 索引进行倒序

    let mut data = Vec::new();

    // 索引偏移序号
    let start = ((page.page - 1) * page.size) as usize;
    let end = ((page.page) * page.size) as usize;

    if end < index_list.len() {
        data = (&index_list[start..end])
            .iter()
            .map(|i| &list[*i]) // 取出实际的内容
            .collect();
    } else if start < index_list.len() {
        data = (&index_list[start..])
            .iter()
            .map(|i| &list[*i]) // 取出实际的内容
            .collect();
    }

    PageData {
        page: page.page,
        size: page.size,
        total: list.len() as u32,
        data,
    }
}

/// 倒序过滤分页查询
pub fn page_find_with_reserve_and_filter<'a, T, F>(
    list: &'a Vec<T>,
    page: &Page,
    max: u32,
    filter: F, // 过滤条件
) -> PageData<&'a T>
where
    F: Fn(&T) -> bool,
{
    page_check(&page, max);

    if list.len() == 0 {
        return page.none();
    }

    // 取出所有的索引
    let mut index_list: Vec<usize> = (0..list.len())
        .into_iter()
        .filter(|i| filter(&list[*i]))
        .collect();
    index_list.reverse(); // 索引进行倒序

    let mut data = Vec::new();

    // 索引偏移序号
    let start = ((page.page - 1) * page.size) as usize;
    let end = ((page.page) * page.size) as usize;

    if end < index_list.len() {
        data = (&index_list[start..end])
            .iter()
            .map(|i| &list[*i]) // 取出实际的内容
            .collect();
    } else if start < index_list.len() {
        data = (&index_list[start..])
            .iter()
            .map(|i| &list[*i]) // 取出实际的内容
            .collect();
    }

    PageData {
        page: page.page,
        size: page.size,
        total: index_list.len() as u32,
        data,
    }
}

/// 按条件分页查询
pub fn page_find_with_sort<T, F, C, S, R>(
    list: &Vec<T>,
    page: &Page,
    max: u32,
    filter: F,    // 过滤条件
    compare: C,   // 排序方法
    transform: S, // 变形方法
) -> PageData<R>
where
    F: Fn(&T) -> bool,
    C: Fn(&T, &T) -> Ordering,
    S: Fn(&T) -> R,
{
    page_check(&page, max);

    // 1. 过滤有效的结果
    let mut list: Vec<&T> = list.iter().filter(|item| filter(item)).collect();

    // 2. 进行排序
    list.sort_by(|a, b| compare(a, b));

    let mut data = Vec::new();

    // 按分页索引
    let start = ((page.page - 1) * page.size) as usize;
    let end = ((page.page) * page.size) as usize;

    if end < list.len() {
        data = (&list[start..end]).iter().map(|t| transform(t)).collect();
    } else if start < list.len() {
        data = (&list[start..]).iter().map(|t| transform(t)).collect();
    }

    PageData {
        page: page.page,
        size: page.size,
        total: list.len() as u32,
        data,
    }
}
