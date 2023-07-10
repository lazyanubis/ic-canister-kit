use candid::CandidType;
use serde::Deserialize;

use crate::identity::{AccountIdentifier, AccountIdentifierHex, UserId, UserIdHex};

pub use super::storage::{NftStorage, NftStorageState};

#[cfg(feature = "nft_ext")]
pub use super::ext::types::*;

#[cfg(feature = "nft_ticket")]
pub use super::ticket::{ForbiddenDuration, NftTicket, NftTicketState, NftTicketStatus};

#[derive(CandidType, Deserialize, Default, Debug, Clone)]
pub struct InnerData {
    pub headers: Vec<(String, String)>,
    pub data: Vec<u8>, // 实际数据
}

#[derive(CandidType, Deserialize, Default, Debug, Clone)]
pub struct OuterData {
    pub headers: Vec<(String, String)>,
    pub url: String, // 实际 url
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum MediaData {
    Inner(InnerData),
    Outer(OuterData),
}

#[derive(CandidType, Deserialize, Debug, Default)]
pub struct NFTInfo {
    pub name: String,                   // NFT 的名称
    pub symbol: String,                 // NFT 的符号
    pub logo: Option<MediaData>,        // logo 信息
    pub thumbnail: Option<MediaData>,   // thumbnail 信息
    pub maintaining: Option<MediaData>, // 升级时候需要统一显示的 logo
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum NFTOwnable {
    None,
    Text(String),
    Media(MediaData),
    Data(Vec<u8>),
    List(Vec<NFTOwnable>),
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
    pub rarity: String,               // 稀有度
    pub content: Option<Vec<u8>>,     // nft的内容
    pub metadata: Vec<MediaData>,     // 该 nft 的元数据 图片数据放这里
    pub thumbnail: Option<MediaData>, // 缩略图
    pub ownable: NFTOwnable,          // 也许需要保存一个隐私信息
}

// NFT 的结构体
// 显示canister 核心的 nft 数据，即所有权和授权，只要抱住了这 2 个数据，那么其他数据都是无所谓的
#[derive(CandidType, Deserialize, Debug)]
pub struct NftView {
    pub name: String, // nft 的唯一，这个数字才是真正的 nft
    // 所有权
    pub owner: AccountIdentifierHex, // 所属人
    pub approved: Option<UserIdHex>, // 授权人 最多只有一个
}
