use candid::CandidType;
use serde::{Deserialize, Serialize};

// 和 principal 和 identity 相关

/*

引入包后, 直接使用如下方法即可增加查看当前用户的接口

#[ic_cdk::query]
async fn whoami() -> ic_canister_kit::types::UserId {
    ic_canister_kit::identity::caller()
}

*/

/// 罐子 ID
pub type CanisterId = candid::Principal; // 类型别名
/// 字符串格式罐子 id
pub type CanisterIdText = String; // ? 字符串格式

/// 集合 id NFT 集合
pub type CollectionId = candid::Principal; // 类型别名
/// 字符串格式的 集合 id
pub type CollectionIdText = String; // ? 字符串格式

/// 用户 ID
pub type UserId = candid::Principal; // 类型别名
/// 字符串格式的 用户 ID
pub type UserIdText = String; // ? 字符串格式

/// 调用者 ID
pub type CallerId = candid::Principal; // 类型别名
/// 字符串格式的 调用者 ID
pub type CallerIdText = String; // ? 字符串格式

/// 子账户
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Subaccount([u8; 32]); // 长度必须是 32 长度
/// 字符串格式的 子账户
pub type SubaccountHex = String; // ? 16 进制文本

/// 账户 ID 通过 Principal 配合子账户计算得来
/// 账户 一般是 account id，如果用户使用的是 principal 也要和 subaccount 一起转换成对应的 account id
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AccountIdentifier([u8; 32]); // 账户
/// 字符串格式的 账户
pub type AccountIdentifierHex = String; // ? 16 进制文本

/// 错误的转换长度
#[derive(Debug)]
pub enum FromVecError {
    /// 错误的长度 必须 32 长度
    InvalidLength, // length must be 32
}
impl std::fmt::Display for FromVecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "The bytes length of account must be 32"),
        }
    }
}
impl std::error::Error for FromVecError {}

/// 编码错误
#[derive(Debug)]
pub enum FromHexError {
    /// 错误的编码
    HexError(hex::FromHexError),
    /// 错误的长度 必须 32 长度
    InvalidLength, // length must be 32
}
impl std::fmt::Display for FromHexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HexError(e) => write!(f, "HexError: {e}"), // 这里的 e 是 FromHexError 的类型
            Self::InvalidLength => write!(f, "The bytes length of account must be 32"),
        }
    }
}
impl std::error::Error for FromHexError {}

impl From<FromVecError> for FromHexError {
    fn from(value: FromVecError) -> Self {
        match value {
            FromVecError::InvalidLength => FromHexError::InvalidLength,
        }
    }
}

impl From<[u8; 32]> for Subaccount {
    fn from(value: [u8; 32]) -> Self {
        Subaccount(value)
    }
}

impl From<u64> for Subaccount {
    fn from(value: u64) -> Self {
        let mut subaccount: [u8; 32] = [0; 32];
        subaccount[24..].copy_from_slice(&value.to_be_bytes());
        subaccount.into()
    }
}

impl Subaccount {
    #[allow(unused)]
    fn into_inner(self) -> [u8; 32] {
        self.0
    }
}

impl From<[u8; 32]> for AccountIdentifier {
    fn from(value: [u8; 32]) -> Self {
        AccountIdentifier(value)
    }
}

impl TryFrom<&[u8]> for AccountIdentifier {
    type Error = FromVecError;

    #[inline]
    fn try_from(account: &[u8]) -> Result<Self, Self::Error> {
        Ok(AccountIdentifier(parse_account(account)?))
    }
}

impl TryFrom<&str> for AccountIdentifier {
    type Error = FromHexError;

    #[inline]
    fn try_from(account_identifier_hex: &str) -> Result<Self, Self::Error> {
        let account = hex::decode(account_identifier_hex).map_err(FromHexError::HexError)?;
        Ok((&account[..]).try_into()?)
    }
}

impl AccountIdentifier {
    /// 取的内部数组数据
    #[allow(unused)]
    #[inline]
    pub fn into_inner(self) -> [u8; 32] {
        self.0
    }

    /// 构造
    #[inline]
    pub fn from(owner: &UserId, subaccount: &Option<Subaccount>) -> Self {
        let subaccount: Subaccount = (*subaccount).unwrap_or_default(); // 默认子账户 应该全是 0

        use sha2::Digest;
        let mut hasher = sha2::Sha224::new(); // 生成 28 个 byte 的 hash 值
        hasher.update(b"\x0Aaccount-id");
        hasher.update(owner.as_slice());
        hasher.update(&subaccount.0[..]);
        let hash: [u8; 28] = hasher.finalize().into();

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&hash);
        let crc32_bytes = hasher.finalize().to_be_bytes(); // 校验码

        let mut result: [u8; 32] = [0u8; 32];
        result[0..4].copy_from_slice(&crc32_bytes[..]); // 校验码放前面
        result[4..32].copy_from_slice(hash.as_ref());

        result.into()
    }

    /// 从数组转换
    #[inline]
    pub fn from_vec(owner: &UserId, subaccount: &Option<Vec<u8>>) -> Result<Self, FromVecError> {
        let subaccount = subaccount
            .as_ref()
            .map(|a| parse_account(a.as_ref()))
            .transpose()?
            .map(|subaccount| subaccount.into());
        Ok(Self::from(owner, &subaccount))
    }

    /// 转换成 16 进制文本
    #[inline]
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

// =================== 基础方法 ===================

/// ! 列表变数组, 长度必须是 32
#[inline]
fn parse_account(account: &[u8]) -> Result<[u8; 32], FromVecError> {
    if account.len() != 32 {
        return Err(FromVecError::InvalidLength);
    }
    let mut array: [u8; 32] = [0; 32];
    array.copy_from_slice(account);
    Ok(array)
}

/// 获取调用者 principal id
#[inline]
pub fn caller() -> CallerId {
    ic_cdk::api::caller()
}

/// 获取本罐子的 principal id
#[inline]
pub fn self_canister_id() -> CanisterId {
    ic_cdk::api::id()
}
