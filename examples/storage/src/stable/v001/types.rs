use std::str::FromStr;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

pub use ic_canister_kit::types::*;

#[allow(unused)]
pub use super::super::{Business, ParsePermission, ScheduleTask};

#[allow(unused)]
pub use super::super::business::*;
#[allow(unused)]
pub use super::business::*;
#[allow(unused)]
pub use super::permission::*;
#[allow(unused)]
pub use super::schedule::schedule_task;

#[allow(unused)]
#[derive(Debug, Clone, Copy, EnumIter, EnumString, strum_macros::Display)]
pub enum RecordTopics {
    // ! 新的权限类型从 0 开始
    Example = 0,              // 模版样例
    ExampleCell = 1,          // 模版样例
    ExampleVec = 2,           // 模版样例
    ExampleMap = 3,           // 模版样例
    ExampleLog = 4,           // 模版样例
    ExamplePriorityQueue = 5, // 模版样例

    // ! 系统倒序排列
    CyclesCharge = 249, // 充值
    Upgrade = 250,      // 升级
    Schedule = 251,     // 定时任务
    Record = 252,       // 记录
    Permission = 253,   // 权限
    Pause = 254,        // 维护
    Initial = 255,      // 初始化
}
#[allow(unused)]
impl RecordTopics {
    pub fn topic(&self) -> RecordTopic {
        *self as u8
    }
    pub fn topics() -> Vec<String> {
        RecordTopics::iter().map(|x| x.to_string()).collect()
    }
    pub fn from(topic: &str) -> Result<Self, strum::ParseError> {
        RecordTopics::from_str(topic)
    }
}

pub struct InnerState {
    pub heap: HeapData, // 保存在堆内存上的数据 最大 4G

    // ! 大的业务数据可以放这里
    pub example_cell: StableCell<ExampleCell>,
    pub example_vec: StableVec<ExampleVec>,
    pub example_map: StableBTreeMap<u64, String>,
    pub example_log: StableLog<String>,
    pub example_priority_queue: StablePriorityQueue<ExampleVec>,
}

#[allow(clippy::derivable_impls)]
impl Default for InnerState {
    fn default() -> Self {
        InnerState {
            heap: HeapData::default(),

            example_cell: init_example_cell_data(),
            example_vec: init_example_vec_data(),
            example_map: init_example_map_data(),
            example_log: init_example_log_data(),
            example_priority_queue: init_example_priority_queue_data(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
pub struct HeapData {
    pub pause: Pause,             // 记录维护状态
    pub permissions: Permissions, // 记录自身权限
    pub records: Records,         // 记录操作记录
    pub schedule: Schedule,       // 记录定时任务

    // ! 小的业务数据可以放这里
    pub business: InnerBusiness,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
pub struct InnerBusiness {
    pub example_data: String,
}

use ic_canister_kit::functions::stable;

const MEMORY_ID_EXAMPLE_CELL: MemoryId = MemoryId::new(0); // 测试 Cell
const MEMORY_ID_EXAMPLE_VEC: MemoryId = MemoryId::new(1); // 测试 Vec
const MEMORY_ID_EXAMPLE_MAP: MemoryId = MemoryId::new(2); // 测试 Map
const MEMORY_ID_EXAMPLE_LOG_ID: MemoryId = MemoryId::new(3); // 测试 Log
const MEMORY_ID_EXAMPLE_LOG_DATA: MemoryId = MemoryId::new(4); // 测试 Log
const MEMORY_ID_EXAMPLE_PRIORITY_QUEUE: MemoryId = MemoryId::new(5); // 测试 PriorityQueue

fn init_example_cell_data() -> StableCell<ExampleCell> {
    stable::init_cell_data(MEMORY_ID_EXAMPLE_CELL, ExampleCell::default())
}

fn init_example_vec_data() -> StableVec<ExampleVec> {
    stable::init_vec_data(MEMORY_ID_EXAMPLE_VEC)
}

fn init_example_map_data() -> StableBTreeMap<u64, String> {
    stable::init_map_data(MEMORY_ID_EXAMPLE_MAP)
}

fn init_example_log_data() -> StableLog<String> {
    stable::init_log_data(MEMORY_ID_EXAMPLE_LOG_ID, MEMORY_ID_EXAMPLE_LOG_DATA)
}

fn init_example_priority_queue_data() -> StablePriorityQueue<ExampleVec> {
    stable::init_priority_queue_data(MEMORY_ID_EXAMPLE_PRIORITY_QUEUE)
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
pub struct ExampleCell {
    pub cell_data: String,
}

impl Storable for ExampleCell {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(ic_canister_kit::functions::stable::common::to_bytes(self))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        ic_canister_kit::functions::stable::common::from_bytes(&bytes)
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ExampleVec {
    pub vec_data: u64,
}

impl Storable for ExampleVec {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = vec![];
        ic_canister_kit::functions::stable::common::u64_to_bytes(&mut bytes, self.vec_data);
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self {
            vec_data: ic_canister_kit::functions::stable::common::u64_from_bytes(&bytes),
        }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 8,
        is_fixed_size: true,
    };
}
