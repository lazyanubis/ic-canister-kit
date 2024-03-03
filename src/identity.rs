// 和 principal 和 identity 相关

/*

引入包后, 直接使用如下方法即可增加查看当前用户的接口

#[ic_cdk::query]
async fn whoami() -> ic_canister_kit::types::UserId {
    ic_canister_kit::identity::caller()
}

*/

// 罐子 ID
pub type CanisterId = candid::Principal; // 类型别名
pub type CanisterIdText = String; // 字符串格式
pub type CollectionId = candid::Principal; // 类型别名
pub type CollectionIdText = String; // 字符串格式

// 用户 ID
pub type UserId = candid::Principal; // 类型别名
pub type UserIdText = String; // 字符串格式

// 调用者 ID
pub type CallerId = candid::Principal; // 类型别名
pub type CallerIdText = String; // 字符串格式

// 子账户
pub type Subaccount = [u8; 32]; // 长度必须是 32 长度
pub type SubaccountHex = String; // 16 进制文本

// 账户 ID 通过 Principal 配合子账户计算得来
// 账户 一般是 account id，如果用户使用的是 principal 也要和 subaccount 一起转换成对应的 account id
pub type AccountIdentifier = [u8; 32]; // 账户
pub type AccountIdentifierHex = String; // 16 进制文本

#[derive(Debug)]
pub enum FromVecError {
    WrongLength, // length must be 32
}
impl std::fmt::Display for FromVecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongLength => write!(f, "The bytes length of account must be 32"),
        }
    }
}
impl std::error::Error for FromVecError {}

#[derive(Debug)]
pub enum FromHexError {
    HexError(hex::FromHexError),
    WrongLength, // length must be 32
}
impl std::fmt::Display for FromHexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HexError(e) => write!(f, "HexError: {e}"), // 这里的 e 是 FromHexError 的类型
            Self::WrongLength => write!(f, "The bytes length of account must be 32"),
        }
    }
}
impl std::error::Error for FromHexError {}

impl From<FromVecError> for FromHexError {
    fn from(value: FromVecError) -> Self {
        match value {
            FromVecError::WrongLength => FromHexError::WrongLength,
        }
    }
}

// 获取调用者 principal id
#[inline]
pub fn caller() -> CallerId {
    ic_cdk::api::caller()
}

// 获取本罐子的 principal id
#[inline]
pub fn self_canister_id() -> CanisterId {
    ic_cdk::api::id()
}

// ! 列表变数组, 长度必须是 32
#[inline]
fn to_account(vec: &Vec<u8>) -> Result<[u8; 32], FromVecError> {
    if vec.len() != 32 {
        return Err(FromVecError::WrongLength);
    }
    let mut array: [u8; 32] = [0; 32];
    array[..32].copy_from_slice(&vec[..32]);
    Ok(array)
}

// 转变为有效的账户
#[inline]
pub fn to_account_identifier(account: &Vec<u8>) -> Result<AccountIdentifier, FromVecError> {
    to_account(account)
}

// 还原 Account
#[inline]
pub fn unwrap_account_identifier_hex(
    account_identifier: &str, // AccountIdentifierHex
) -> Result<AccountIdentifier, FromHexError> {
    let vec = hex::decode(account_identifier).map_err(FromHexError::HexError)?;
    Ok(to_account(&vec)?)
}

// 文本 Account
#[inline]
pub fn wrap_account_identifier(account_identifier: &AccountIdentifier) -> AccountIdentifierHex {
    hex::encode(account_identifier)
}

// 转换成 Account
#[inline]
pub fn parse_account_identifier(
    user_id: &UserId,
    subaccount: &Option<Subaccount>,
) -> AccountIdentifier {
    let subaccount: [u8; 32] = (*subaccount).unwrap_or([0; 32]); // 默认子账户 应该全是 0

    use sha2::Digest;
    let mut hasher = sha2::Sha224::new(); // 生成 28 个 byte 的 hash 值
    hasher.update(b"\x0Aaccount-id");
    hasher.update(user_id.as_slice());
    hasher.update(&subaccount[..]);
    let hash: [u8; 28] = hasher.finalize().into();

    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&hash);
    let crc32_bytes = hasher.finalize().to_be_bytes(); // 校验码

    let mut result: [u8; 32] = [0u8; 32];
    result[0..4].copy_from_slice(&crc32_bytes[..]); // 校验码放前面
    result[4..32].copy_from_slice(hash.as_ref());

    result
}

// 转换成文本 Account
#[inline]
pub fn parse_account_identifier_hex(
    user_id: &UserId,
    subaccount: &Option<Subaccount>,
) -> AccountIdentifierHex {
    wrap_account_identifier(&parse_account_identifier(user_id, subaccount))
}

// 数字变成子账户
#[inline]
pub fn parse_u64_to_subaccount(subaccount: u64) -> Subaccount {
    let mut list: [u8; 32] = [0; 32];
    list[24..].copy_from_slice(&subaccount.to_be_bytes());
    list
}

// 转换成 Account
#[inline]
pub fn parse_account_identifier_by_vec(
    user_id: &UserId,
    subaccount: &Option<Vec<u8>>,
) -> Result<AccountIdentifier, FromVecError> {
    let subaccount = subaccount.as_ref().map(to_account).transpose()?;
    Ok(parse_account_identifier(user_id, &subaccount))
}

// 转换成文本 Account
#[inline]
pub fn parse_account_identifier_hex_by_vec(
    user_id: &UserId,
    subaccount: &Option<Vec<u8>>,
) -> Result<AccountIdentifierHex, FromVecError> {
    Ok(wrap_account_identifier(&parse_account_identifier_by_vec(
        user_id, subaccount,
    )?))
}
