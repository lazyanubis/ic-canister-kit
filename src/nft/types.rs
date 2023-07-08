use std::collections::HashMap;

use candid::CandidType;
use serde::Deserialize;

use crate::identity::{AccountIdentifier, UserId};

pub use super::storage::{NftStorage, NftStorageState};

#[cfg(feature = "nft_ext")]
pub use super::ext::types::*;

#[cfg(feature = "nft_ticket")]
pub use super::ticket::{NftTicket, NftTicketState, NftTicketStatus};

#[derive(CandidType, Deserialize, Default, Debug)]
pub struct MediaData {
    pub headers: Vec<(String, String)>,
    pub data: Vec<u8>, // 实际数据
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Media {
    Url(String),
    Media(MediaData),
}

// 元数据的响应头值类型
// #[allow(clippy::enum_variant_names)]
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum MetadataValue {
    Text(String),  // 文本类型
    Blob(Vec<u8>), // 二进制类型
    Nat(u128),     // 数字类型 rust 居然有 128 位的数字
    Nat8(u8),      // u8 类型
    Nat16(u16),    // u16 类型
    Nat32(u32),    // u32 类型
    Nat64(u64),    // u64 类型
}

// 元数据的键值对
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub header: HashMap<String, MetadataValue>, // 键值对
    pub data: Vec<u8>,                          // 实际数据
}

#[derive(CandidType, Deserialize, Debug)]
pub enum MetadataMedia {
    Url(String),
    Metadata(Metadata),
}

#[derive(CandidType, Deserialize, Debug, Default)]
pub struct NFTInfo {
    pub name: String,               // NFT 的名称
    pub symbol: String,             // NFT 的符号
    pub logo: Option<Media>,        // logo 信息
    pub maintaining: Option<Media>, // 升级时候需要统一显示的 logo
}

// NFT 的结构体
#[derive(CandidType, Deserialize, Debug)]
pub struct Nft {
    pub index: usize, // nft 的 id，这个数字才是真正的 nft
    pub name: String, // 也许 NFT 有名称
    // 所有权
    pub owner: AccountIdentifier, // 所属人
    pub approved: Option<UserId>, // 授权人 最多只有一个
    // 伴生数据
    pub rarity: String,           // 稀有度
    pub content: Option<Vec<u8>>, // nft的内容
    pub metadata: Vec<Metadata>,  // 该 nft 的元数据 图片数据放这里
    pub thumbnail: Option<Media>, // 缩略图
    pub secret: String,           // 也许需要保存一个隐私信息
}
