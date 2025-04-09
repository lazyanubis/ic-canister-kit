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
pub type Subaccount = icrc_ledger_types::icrc1::account::Subaccount; // 长度必须是 32 长度
/// 字符串格式的 子账户
pub type SubaccountHex = String; // ? 16 进制文本

/// 账户 ID 通过 Principal 配合子账户计算得来
/// 账户 一般是 account id，如果用户使用的是 principal 也要和 subaccount 一起转换成对应的 account id
pub type AccountIdentifier = ic_ledger_types::AccountIdentifier; // 账户
/// 字符串格式的 账户
pub type AccountIdentifierHex = String; // ? 16 进制文本

// =================== 基础方法 ===================

/// 数字转 subaccount
pub fn u64_to_subaccount(value: u64) -> Subaccount {
    let mut subaccount: [u8; 32] = [0; 32];
    subaccount[24..].copy_from_slice(&value.to_be_bytes());
    subaccount
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
