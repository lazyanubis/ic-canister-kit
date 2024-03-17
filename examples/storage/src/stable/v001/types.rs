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
    UploadFile = 0, // 上传文件
    DeleteFile = 1, // 删除文件

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
    pub stable_business: StableBusiness,
}

#[allow(clippy::derivable_impls)]
impl Default for InnerState {
    fn default() -> Self {
        InnerState {
            heap: HeapData::default(),

            stable_business: StableBusiness::default(),
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
    // pub assets: HashMap<HashDigest, AssetData>, // key 是 hash
    pub files: HashMap<String, AssetFile>,   // key 是 path
    hashes: HashMap<HashDigest, HashedPath>, // key 是 hash, value 是 path, 没有 path 的数据是没有保存意义的

    uploading: HashMap<String, UploadingFile>, // key 是 path
}

use ic_canister_kit::functions::stable;

const MEMORY_ID_ASSETS: MemoryId = MemoryId::new(0); // 存放实际文件，hash 为键
                                                     // const MEMORY_ID_FILES: MemoryId = MemoryId::new(1); // 存放文件描述信息，path 为键
                                                     // const MEMORY_ID_HASHES: MemoryId = MemoryId::new(2); //
                                                     // const MEMORY_ID_UPLOAD_FILES: MemoryId = MemoryId::new(3); // 测试 Log

fn init_assets_data() -> StableBTreeMap<HashDigest, AssetData> {
    stable::init_map_data(MEMORY_ID_ASSETS)
}

// fn init_files_data() -> StableBTreeMap<String, AssetFile> {
//     stable::init_map_data(MEMORY_ID_FILES)
// }

// fn init_hashes_data() -> StableBTreeMap<HashDigest, HashedPath> {
//     stable::init_map_data(MEMORY_ID_HASHES)
// }

// fn init_upload_files_data() -> StableBTreeMap<String, UploadingFile> {
//     stable::init_map_data(MEMORY_ID_UPLOAD_FILES)
// }

pub struct StableBusiness {
    pub assets: StableBTreeMap<HashDigest, AssetData>, // key 是 hash
}

impl Default for StableBusiness {
    fn default() -> Self {
        Self {
            assets: init_assets_data(),
        }
    }
}

impl Storable for HashDigest {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Borrowed(&self.0)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let mut value = [0; 32];
        value.copy_from_slice(&bytes[0..32]);
        Self(value)
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 32,
        is_fixed_size: true,
    };
}

impl Storable for AssetData {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(ic_canister_kit::functions::stable::common::to_bytes(self))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        ic_canister_kit::functions::stable::common::from_bytes(&bytes)
    }

    const BOUND: Bound = Bound::Unbounded;
}

// ============================== 文件数据 ==============================

#[derive(
    CandidType, Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct HashDigest([u8; 32]);

impl HashDigest {
    pub fn hex(&self) -> String {
        hex::encode(self.0)
    }
}

// 单个文件数据
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct AssetData {
    pub hash: HashDigest,
    pub size: u64,
    pub data: Vec<u8>, // 实际数据
}

// 对外的路径数据 指向文件数据
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct AssetFile {
    pub path: String,
    pub created: TimestampNanos,
    pub modified: TimestampNanos,
    pub headers: Vec<(String, String)>,
    pub hash: HashDigest,
    pub size: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
pub struct HashedPath(HashSet<String>);

// =========== 上传过程中的对象 ===========

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct UploadingFile {
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub data: Vec<u8>, // 上传中的数据

    pub size: u64,          // 文件大小
    pub chunk_size: u64,    // 块大小
    pub chunks: u32,        // 需要上传的次数
    pub chunked: Vec<bool>, // 记录每一个块的上传状态
}

// 上传参数
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct UploadingArg {
    pub path: String,
    pub headers: Vec<(String, String)>, // 使用的 header
    pub size: u64,                      // 文件大小
    pub chunk_size: u64,                // 块大小
    pub index: u32,                     // 本次上传的数据
    pub chunk: Vec<u8>,                 // 上传中的数据
}

// =========== 查询的对象 ===========

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct QueryFile {
    pub path: String,
    pub size: u64,
    pub headers: Vec<(String, String)>,
    pub created: TimestampNanos,
    pub modified: TimestampNanos,
    pub hash: String,
}

impl InnerState {
    fn hash(file: &UploadingFile) -> HashDigest {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(&file.data[0..(file.size as usize)]);
        let digest: [u8; 32] = hasher.finalize().into();
        HashDigest(digest)
    }
    pub fn put_file(&mut self, file: UploadingFile) {
        // 1. 计算 hash
        let hash = Self::hash(&file);
        // 2. 插入 assets: hash -> data
        self.business_assets_insert(
            hash,
            AssetData {
                hash,
                size: file.size,
                data: file.data,
            },
        );
        // 3. 插入 files: path -> hash
        let now = ic_canister_kit::times::now();
        if let Some(exist) = self.heap.business.files.get_mut(&file.path) {
            exist.modified = now;
            exist.headers = file.headers;
            exist.hash = hash;
        } else {
            self.heap.business.files.insert(
                file.path.clone(),
                AssetFile {
                    path: file.path.clone(),
                    created: now,
                    modified: now,
                    headers: file.headers,
                    hash,
                    size: file.size,
                },
            );
        }

        // 4. 插入 hashes: hash -> [path]
        self.heap.business.hashes.entry(hash).or_default();
        if let Some(hash_path) = self.heap.business.hashes.get_mut(&hash) {
            if !hash_path.0.contains(&file.path) {
                hash_path.0.insert(file.path);
            }
        }
    }
    pub fn clean_file(&mut self, path: &String) {
        // 1. 找到文件
        let file = match self.heap.business.files.get(path) {
            Some(file) => file.clone(),
            None => return,
        };
        // 2. 清除 file
        self.heap.business.files.remove(path);
        // 3. 清除 hashes
        if let Some(HashedPath(path_set)) = self.heap.business.hashes.get_mut(&file.hash) {
            path_set.remove(&file.path);
            if path_set.is_empty() {
                // 需要清空
                self.heap.business.hashes.remove(&file.hash);
                // 4. 清空 assets
                self.business_assets_remove(&file.hash);
            }
        }
    }
    pub fn files(&self) -> Vec<QueryFile> {
        self.heap
            .business
            .files
            .iter()
            .map(|(path, file)| {
                #[allow(clippy::unwrap_used)] // ? SAFETY
                QueryFile {
                    path: path.to_string(),
                    size: file.size,
                    headers: file.headers.clone(),
                    created: file.created,
                    modified: file.modified,
                    hash: file.hash.hex(),
                }
            })
            .collect()
    }
    pub fn download(&self, path: String) -> Vec<u8> {
        #[allow(clippy::expect_used)] // ? SAFETY
        let file = self.heap.business.files.get(&path).expect("File not found");
        #[allow(clippy::expect_used)] // ? SAFETY
        let asset = self
            .business_assets_get(&file.hash)
            .expect("File not found");
        asset.data.clone()
    }
    pub fn download_by(&self, path: String, offset: u64, offset_end: u64) -> Vec<u8> {
        #[allow(clippy::expect_used)] // ? SAFETY
        let file = self.heap.business.files.get(&path).expect("File not found");
        #[allow(clippy::expect_used)] // ? SAFETY
        let asset = self
            .business_assets_get(&file.hash)
            .expect("File not found");
        (asset.data[(offset as usize)..(offset_end as usize)]).to_vec()
    }

    fn chunks(arg: &UploadingArg) -> u32 {
        let mut chunks = arg.size / arg.chunk_size; // 完整的块数
        if chunks * arg.chunk_size < arg.size {
            chunks += 1;
        }
        chunks as u32
    }
    fn offset(arg: &UploadingArg) -> (usize, usize) {
        let chunks = Self::chunks(arg);
        let offset = arg.chunk_size * arg.index as u64;
        let mut offset_end = offset + arg.chunk_size;
        if arg.index == chunks - 1 {
            offset_end = arg.size;
        }
        (offset as usize, offset_end as usize)
    }
    fn check_arg(arg: &UploadingArg) {
        // 1. 检查 路径名
        assert!(!arg.path.is_empty(), "must has path");
        assert!(arg.path.starts_with('/'), "path must start with /");
        // 2. 检查 headers
        // 3. 检查 size
        assert!(0 < arg.size, "size can not be 0");
        assert!(
            arg.size <= 1024 * 1024 * 1024 * 4, // 最大文件 4G
            "size must less than 4GB"
        );
        // 4. 检查 chunk_size
        assert!(0 < arg.chunk_size, "chunk size can not be 0");
        // 5. 检查 index
        let chunks = Self::chunks(arg);
        assert!(arg.index < chunks, "wrong index");
        // 6. 检查 data
        if arg.index < chunks - 1 || arg.size == arg.chunk_size * chunks as u64 {
            // 是前面完整的 或者 整好整除
            assert!(
                arg.chunk.len() as u64 == arg.chunk_size,
                "wrong chunk length"
            );
        } else {
            // 是剩下的
            assert!(
                arg.chunk.len() as u64 == arg.size % arg.chunk_size,
                "wrong chunk length"
            );
        }
    }
    fn check_file(&mut self, arg: &UploadingArg) {
        if let Some(file) = self.heap.business.uploading.get(&arg.path) {
            // 已经有这个文件了, 需要比较一下, 参数是否一致
            assert!(arg.path == file.path, "wrong path, system error.");
            let chunks = Self::chunks(arg);
            if arg.size != file.size // 文件长度不一致
                || file.data.len() < file.size as usize // 暂存长度不对
                || arg.chunk_size != file.chunk_size
                || chunks != file.chunks
                || file.chunked.len() < file.chunks as usize
            {
                // 非致命错误, 清空原来的文件就好
                self.heap.business.files.remove(&arg.path);
            }
        } else {
            // 原来没有的情况下
            let chunks = Self::chunks(arg);
            self.heap.business.uploading.insert(
                arg.path.clone(),
                UploadingFile {
                    path: arg.path.clone(),
                    headers: arg.headers.clone(),
                    data: vec![0; arg.size as usize],
                    size: arg.size,
                    chunk_size: arg.chunk_size,
                    chunks,
                    chunked: vec![false; chunks as usize],
                },
            );
        }
    }
    pub fn put_uploading(&mut self, arg: UploadingArg) {
        // 0. 检查参数是否有效
        Self::check_arg(&arg);

        // 1. 检查文件
        self.check_file(&arg);

        // 2. 找的对应的缓存文件
        let mut done = false;
        if let Some(file) = self.heap.business.uploading.get_mut(&arg.path) {
            // 3. 复制有效的信息
            let (offset, offset_end) = Self::offset(&arg);
            file.headers = arg.headers;
            file.data.splice(offset..offset_end, arg.chunk); // 复制内容
            file.chunked[arg.index as usize] = true;

            // 4. 是否已经完整
            for uploaded in file.chunked.iter() {
                if !uploaded {
                    return; // 还有没上传的
                }
            }
            done = true; // 已经完成的
        }
        if done {
            if let Some(file) = self.heap.business.uploading.remove(&arg.path) {
                self.put_file(file);
            }
        }
    }
    pub fn clean_uploading(&mut self, path: &String) {
        self.heap.business.files.remove(path);
    }
}
