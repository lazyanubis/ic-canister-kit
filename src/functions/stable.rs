use std::cell::RefCell;

use ic_stable_structures::{memory_manager::MemoryManager, DefaultMemoryImpl};

/// 简化虚拟内存
pub type VirtualMemory =
    ic_stable_structures::memory_manager::VirtualMemory<ic_stable_structures::DefaultMemoryImpl>;
pub use ic_stable_structures::memory_manager::MemoryId;
pub use ic_stable_structures::storable::Bound;
pub use ic_stable_structures::writer::Writer;
pub use ic_stable_structures::Memory;
pub use ic_stable_structures::Storable;
pub use std::borrow::Cow;

/// 序列化 heap data
pub trait StorableHeapData {
    /// 序列化 堆数据
    fn heap_data_to_bytes(&self) -> Vec<u8>;

    /// 反序列化 堆数据
    fn heap_data_from_bytes(&mut self, bytes: &[u8]);
}

/// 稳定对象
/// ! 读取和写入都是全量操作，成本比较大
pub type StableCell<T, M> = ic_stable_structures::Cell<T, M>;
/// 稳定列表
/// ! 存储有限长度数据，若不固定长度，则按照最大长度存储，不均匀的数据使用空间浪费比较严重
/// ! push 和 pop 没有任意位置删除的功能
/// ! 若不在乎顺序，则移动末尾元素到被删除的位置可实现任意删除。结合 StableBTreeMap 存储双向的缩影数据，可实现任意位置删除。
/// ! 最大的问题还是数据长度问题，任意删除功能不是核心难题。
pub type StableVec<T, M> = ic_stable_structures::Vec<T, M>;
/// 稳定映射
pub type StableBTreeMap<K, V, M> = ic_stable_structures::BTreeMap<K, V, M>;
/// 稳定日志
/// ! 支持变长 无法删除和移动
pub type StableLog<T, INDEX, DATA> = ic_stable_structures::Log<T, INDEX, DATA>;
/// 稳定优先级队列 按照排序方式存放数据
/// ! 内部使用 vec 方式实现，优缺点一致
pub type StablePriorityQueue<T, M> = ic_stable_structures::MinHeap<T, M>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

const MEMORY_ID_UPGRADED: MemoryId = MemoryId::new(254);

/// 获取虚拟内存
#[inline]
pub fn get_virtual_memory(memory_id: MemoryId) -> VirtualMemory {
    MEMORY_MANAGER.with(|memory_manager| memory_manager.borrow().get(memory_id))
}

/// 获取升级用的虚拟内存
#[inline]
pub fn get_upgrades_memory() -> VirtualMemory {
    get_virtual_memory(MEMORY_ID_UPGRADED)
}

/// 包装升级内存
pub struct WriteUpgradeMemory<'a, M> {
    writer: Writer<'a, M>,
}

/// 包装升级内存
pub struct ReadUpgradeMemory<'a, M> {
    memory: &'a M,
    offset: u64,
}

impl<'a, M: Memory> WriteUpgradeMemory<'a, M> {
    /// 构造升级对象
    pub fn new(memory: &'a mut M) -> WriteUpgradeMemory<'a, M> {
        Self {
            writer: Writer::new(memory, 0),
        }
    }

    /// 写入升级数据
    pub fn write(&mut self, bytes: &[u8]) {
        #[allow(clippy::expect_used)] // ? SAFETY
        self.writer
            .write(bytes)
            .expect("failed to write to upgrade memory");
    }

    /// 写入 u32
    pub fn write_u32(&mut self, value: u32) {
        let mut bytes = Vec::with_capacity(4);
        common::u32_to_bytes(&mut bytes, value);
        self.write(&bytes);
    }

    /// 写入 u64
    pub fn write_u64(&mut self, value: u64) {
        let mut bytes = Vec::with_capacity(8);
        common::u64_to_bytes(&mut bytes, value);
        self.write(&bytes);
    }
}

impl<'a, M: Memory> ReadUpgradeMemory<'a, M> {
    /// 构造升级对象
    pub fn new(memory: &'a M) -> ReadUpgradeMemory<'a, M> {
        Self { memory, offset: 0 }
    }

    /// 读取升级数据
    pub fn read(&mut self, bytes: &mut [u8]) {
        self.memory.read(self.offset, bytes);
        self.offset += bytes.len() as u64;
    }

    /// 读取 u32
    pub fn read_u32(&mut self) -> u32 {
        let mut bytes = Vec::with_capacity(4);
        self.read(&mut bytes);
        common::u32_from_bytes(&bytes)
    }

    /// 读取 u64
    pub fn read_u64(&mut self) -> u64 {
        let mut bytes = Vec::with_capacity(8);
        self.read(&mut bytes);
        common::u64_from_bytes(&bytes)
    }
}

/// 一些可能用到的工具方法
pub mod common {
    use super::*;

    /// usize -> 4 bytes
    #[inline]
    pub fn usize_to_4bytes(buf: &mut Vec<u8>, value: usize) {
        buf.extend(&(value as u32).to_bytes()[..]);
    }

    /// 4 bytes -> usize
    #[inline]
    pub fn usize_from_4bytes(bytes: &[u8]) -> usize {
        u32::from_bytes(Cow::Borrowed(&bytes[..4])) as usize
    }

    /// u32 -> 4 bytes
    #[inline]
    pub fn u32_to_bytes(buf: &mut Vec<u8>, value: u32) {
        buf.extend(&value.to_bytes()[..]);
    }

    /// 4 bytes -> u32
    #[inline]
    pub fn u32_from_bytes(bytes: &[u8]) -> u32 {
        u32::from_bytes(Cow::Borrowed(&bytes[..4]))
    }

    /// u64 -> 8 bytes
    #[inline]
    pub fn u64_to_bytes(buf: &mut Vec<u8>, value: u64) {
        buf.extend(&value.to_bytes()[..]);
    }

    /// 8 bytes -> u64
    #[inline]
    pub fn u64_from_bytes(bytes: &[u8]) -> u64 {
        u64::from_bytes(Cow::Borrowed(&bytes[..8]))
    }
}
