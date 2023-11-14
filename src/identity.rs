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
pub type CanisterIdHex = String; // 16 进制文本

// 用户 ID
pub type UserId = candid::Principal; // 类型别名
pub type UserIdHex = String; // 16 进制文本

// 调用者 ID
pub type CallerId = candid::Principal; // 类型别名

// 子账户
pub type Subaccount = [u8; 32]; // 长度必须是 32 长度

// 账户 ID 通过 Principal 配合子账户计算得来
// 账户 一般是 account id，如果用户使用的是 principal 也要和 subaccount 一起转换成对应的 account id
pub type AccountIdentifier = [u8; 32]; // 账户
pub type AccountIdentifierHex = String; // 16 进制文本

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

// ! 列表变数值, 长度必须是 32
fn to_array(vec: &Vec<u8>) -> [u8; 32] {
    assert!(vec.len() == 32);
    let mut array: [u8; 32] = [0; 32];
    for i in 0..32 {
        array[i] = vec[i];
    }
    array
}

// 转变为有效的账户
pub fn to_account_identifier(account: &Vec<u8>) -> AccountIdentifier {
    assert!(account.len() == 32, "Invalid Account");
    to_array(&account)
}

// 还原 Account
pub fn unwrap_account_identifier_hex(
    account_identifier: AccountIdentifierHex,
) -> AccountIdentifier {
    let vec = hex::decode(&account_identifier).unwrap();
    assert!(vec.len() == 32, "Invalid Account Id");
    to_array(&vec)
}

// 文本 Account
pub fn wrap_account_identifier(account_identifier: &AccountIdentifier) -> AccountIdentifierHex {
    hex::encode(&account_identifier)
}

// 转换成 Account
pub fn parse_account_identifier(
    user_id: &UserId,
    subaccount: &Option<Subaccount>,
) -> AccountIdentifier {
    let subaccount: [u8; 32] = subaccount.clone().unwrap_or_else(|| [0; 32]); // 默认子账户 应该全是 0

    assert!(subaccount.len() == 32, "Invalid Subaccount");

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
pub fn parse_account_identifier_hex(
    user_id: &UserId,
    subaccount: &Option<Subaccount>,
) -> AccountIdentifierHex {
    wrap_account_identifier(&parse_account_identifier(user_id, subaccount))
}

// 数字变成子账户
pub fn parse_u64_to_subaccount(subaccount: u64) -> Subaccount {
    let mut list: [u8; 32] = [0; 32];
    for i in 0..8 {
        list[24 + i] = ((subaccount >> 8 * (7 - i)) & 0xff) as u8
    }
    list
}

// 转换成 Account
pub fn parse_account_identifier_by_vec(
    user_id: &UserId,
    subaccount: &Option<Vec<u8>>,
) -> AccountIdentifier {
    if let Some(vec) = subaccount {
        assert!(vec.len() == 32, "Invalid Subaccount");
    }
    let subaccount = if let Some(vec) = subaccount {
        Some(to_array(vec))
    } else {
        None
    };
    parse_account_identifier(user_id, &subaccount)
}

// 转换成文本 Account
pub fn parse_account_identifier_hex_by_vec(
    user_id: &UserId,
    subaccount: &Option<Vec<u8>>,
) -> AccountIdentifierHex {
    wrap_account_identifier(&parse_account_identifier_by_vec(user_id, subaccount))
}
