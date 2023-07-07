// ============= 分页查询 =============

use std::cmp::Ordering;

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct Page {
    pub page: u32,
    pub size: u32,
}

impl Page {
    pub fn none<T>(&self) -> PageData<T> {
        PageData {
            page: self.page,
            size: self.size,
            data: Vec::new(),
            all: 0,
        }
    }
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct PageData<T> {
    pub page: u32,
    pub size: u32,
    pub data: Vec<T>,
    pub all: u32,
}

// 检查分页选项是否有效
pub fn page_check(page: &Page, max: u32) {
    if page.page == 0 {
        panic!("page can not be 0")
    }
    if page.size > max {
        panic!("max page size is {} < {:?}", max, page.size)
    }
}

/// 直接分页查询
pub fn page_find<T: Clone>(list: &Vec<T>, page: Page) -> PageData<T> {
    let mut data = Vec::new();

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
        data,
        all: list.len() as u32,
    }
}

/// 倒序分页查询
pub fn page_find_with_reserve<T: Clone>(list: &Vec<T>, page: Page) -> PageData<T> {
    if list.len() == 0 {
        return page.none();
    }

    let mut index_list: Vec<usize> = (0..list.len()).into_iter().collect();
    index_list.reverse();

    let mut data = Vec::new();

    let start = ((page.page - 1) * page.size) as usize;
    let end = ((page.page) * page.size) as usize;

    if end < index_list.len() {
        data = (&index_list[start..end])
            .iter()
            .map(|i| list[*i].clone())
            .collect();
    } else if start < index_list.len() {
        data = (&index_list[start..])
            .iter()
            .map(|i| list[*i].clone())
            .collect();
    }

    PageData {
        page: page.page,
        size: page.size,
        data,
        all: list.len() as u32,
    }
}

/// 按条件分页查询
pub fn page_find_with_sort<T, F, C, S, R>(
    list: &Vec<T>,
    page: Page,
    filter: F,
    compare: C,
    transform: S,
) -> PageData<R>
where
    F: Fn(&T) -> bool,
    C: Fn(&T, &T) -> Ordering,
    S: Fn(&T) -> R,
{
    let mut list: Vec<&T> = list.iter().filter(|item| filter(item)).collect();

    list.sort_by(|a, b| compare(a, b));

    let mut data = Vec::new();

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
        data,
        all: list.len() as u32,
    }
}
