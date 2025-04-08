//! ICRC1 标准接口
//! https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-1

use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    canister::{call::call_canister, fetch_tuple0, types::CanisterCallResult},
    identity::{CanisterId, Subaccount, UserId},
};

// ICRC1 标准
// icrc1_supported_standards : () -> (vec StandardRecord) query;
// icrc1_name : () -> (text) query;
// icrc1_symbol : () -> (text) query;
// icrc1_decimals : () -> (nat8) query;
// icrc1_total_supply : () -> (nat) query;
// icrc1_balance_of : (Account) -> (nat) query;
// icrc1_fee : () -> (nat) query;
// icrc1_transfer : (TransferArg) -> (Result);
// 下面的接口应该用不到
// icrc1_metadata : () -> (vec record { text; MetadataValue }) query;
// icrc1_minting_account : () -> (opt Account) query;

// =================== 账本方法 ===================

// https://dashboard.internetcomputer.org/canister/ryjl3-tyaaa-aaaaa-aaaba-cai

//  ============== 查询支持的标准 ==============
// icrc1_supported_standards : () -> (vec StandardRecord) query;
// type StandardRecord = record { url : text; name : text };

/// 支持的标准
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct Icrc1SupportedStandard {
    name: String, // ICRC-1
    url: String,  // https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-1
}
/// 支持的标准
pub type Icrc1SupportedStandards = Vec<Icrc1SupportedStandard>;

/// 查询支持的标准
#[allow(unused)]
pub async fn icrc1_supported_standards(
    canister_id: CanisterId,
) -> CanisterCallResult<Icrc1SupportedStandards> {
    call_canister::<_, (Icrc1SupportedStandards,)>(canister_id, "icrc1_supported_standards", ())
        .await
        .map(fetch_tuple0)
}
/// 查询支持的标准
#[allow(unused)]
pub async fn icrc1_supported_standards_by(
    canister_id: CanisterId,
) -> CanisterCallResult<Vec<String>> {
    icrc1_supported_standards(canister_id)
        .await
        .map(|r| r.into_iter().map(|i| i.name).collect())
}

//  ============== 查询名称 ==============
// icrc1_name : () -> (text) query;

/// 查询名称
#[allow(unused)]
pub async fn icrc1_name(canister_id: CanisterId) -> CanisterCallResult<String> {
    call_canister::<_, (String,)>(canister_id, "icrc1_name", ())
        .await
        .map(fetch_tuple0)
}

//  ============== 查询符号 ==============
// icrc1_symbol : () -> (text) query;

/// 查询 symbol
#[allow(unused)]
pub async fn icrc1_symbol(canister_id: CanisterId) -> CanisterCallResult<String> {
    call_canister::<_, (String,)>(canister_id, "icrc1_symbol", ())
        .await
        .map(fetch_tuple0)
}

//  ============== 查询精度 ==============
// icrc1_decimals : () -> (nat8) query;

///  查询精度
#[allow(unused)]
pub async fn icrc1_decimals(canister_id: CanisterId) -> CanisterCallResult<u8> {
    call_canister::<_, (u8,)>(canister_id, "icrc1_decimals", ())
        .await
        .map(fetch_tuple0)
}

//  ============== 查询总供应量 ==============
// icrc1_decimals : () -> (nat8) query;

/// 供应量
pub type Icrc1TotalSupply = candid::Nat;

/// 查询总供应量
#[allow(unused)]
pub async fn icrc1_total_supply(canister_id: CanisterId) -> CanisterCallResult<Icrc1TotalSupply> {
    call_canister::<_, (Icrc1TotalSupply,)>(canister_id, "icrc1_total_supply", ())
        .await
        .map(fetch_tuple0)
}

//  ============== 查询余额 ==============
// icrc1_balance_of : (Account) -> (nat) query;
// type Account = record { owner : principal; subaccount : opt vec nat8 };

/// 子账户
// pub type Icrc1Subaccount = Vec<u8>;
pub type Icrc1Subaccount = Subaccount; // ! 修改为安全的参数

/// icrc1 账户对象
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct Icrc1Account {
    /// 所有者
    pub owner: UserId,
    /// 子账户
    pub subaccount: Option<Icrc1Subaccount>,
}
/// 余额
pub type Icrc1Balance = candid::Nat;
/// 查询余额
#[allow(unused)]
pub async fn icrc1_balance_of(
    canister_id: CanisterId,
    account: Icrc1Account,
) -> CanisterCallResult<Icrc1Balance> {
    call_canister::<_, (Icrc1Balance,)>(canister_id, "icrc1_balance_of", (account,))
        .await
        .map(fetch_tuple0)
}
/// 查询余额
#[allow(unused)]
pub async fn icrc1_balance_of_by(
    canister_id: CanisterId,
    owner: UserId,
    subaccount: Option<Icrc1Subaccount>,
) -> CanisterCallResult<Icrc1Balance> {
    icrc1_balance_of(canister_id, Icrc1Account { owner, subaccount }).await
}

//  ============== 查询交易费用 ==============
// icrc1_fee : () -> (nat) query;

/// 手续费
pub type Icrc1Fee = candid::Nat;
/// 查询手续费
#[allow(unused)]
pub async fn icrc1_fee(canister_id: CanisterId) -> CanisterCallResult<Icrc1Fee> {
    call_canister::<_, (Icrc1Fee,)>(canister_id, "icrc1_fee", ())
        .await
        .map(fetch_tuple0)
}

//  ============== 转账 ==============
// icrc1_transfer : (TransferArg) -> (Result);
// type TransferArg = record {
//     to : Account;
//     fee : opt nat;
//     memo : opt vec nat8;
//     from_subaccount : opt vec nat8;
//     created_at_time : opt nat64;
//     amount : nat;
// };
// type Result = variant { Ok : nat; Err : TransferError };
// type TransferError = variant {
//     GenericError : record { message : text; error_code : nat };
//     TemporarilyUnavailable;
//     BadBurn : record { min_burn_amount : nat };
//     Duplicate : record { duplicate_of : nat };
//     BadFee : record { expected_fee : nat };
//     CreatedInFuture : record { ledger_time : nat64 };
//     TooOld;
//     InsufficientFunds : record { balance : nat };
// };

// /// 转账 memo
// /// 长度不限制
// pub type Icrc1Memo = Vec<u8>;

/// 转账参数
pub type Icrc1TransferArgs = icrc_ledger_types::icrc1::transfer::TransferArg;
// #[derive(CandidType, Deserialize, Debug, Clone)]
// pub struct Icrc1TransferArgs {
//     /// 调用者指定的使用的子账户地址
//     pub from_subaccount: Option<Icrc1Subaccount>,
//     /// 目标地址
//     pub to: Icrc1Account,
//     /// 想要转给目标地址的数量
//     pub amount: candid::Nat,
//     /// 交易费
//     pub fee: Option<candid::Nat>,
//     /// 交易标识码
//     pub memo: Option<Icrc1Memo>,
//     /// 请求的时间节点
//     pub created_at_time: Option<u64>,
// }
/// 转账可能出现的错误
pub type Icrc1TransferError = icrc_ledger_types::icrc1::transfer::TransferError;
// #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
// pub enum Icrc1TransferError {
//     /// 通用错误
//     GenericError {
//         /// 错误码
//         error_code: candid::Nat,
//         /// 错误消息
//         message: String,
//     },
//     /// 临时不可用
//     TemporarilyUnavailable,
//     /// 错误的燃烧数量
//     BadBurn {
//         /// 最小的燃烧数量
//         min_burn_amount: candid::Nat,
//     },
//     /// 交易重复
//     Duplicate {
//         /// 重复的交易
//         duplicate_of: candid::Nat,
//     },
//     /// 错误的费用
//     BadFee {
//         /// 期望的费用
//         expected_fee: candid::Nat,
//     },
//     /// 未来的转账
//     CreatedInFuture {
//         /// 当前账本时间
//         ledger_time: u64,
//     },
//     /// 订单太老
//     TooOld,
//     /// 余额不足
//     InsufficientFunds {
//         /// 当前余额
//         balance: candid::Nat,
//     },
// }
/// 转账结果
pub type Icrc1TransferResult =
    Result<candid::Nat, icrc_ledger_types::icrc1::transfer::TransferError>;
/// 进行转账
#[allow(unused)]
pub async fn icrc1_transfer(
    canister_id: CanisterId,
    args: Icrc1TransferArgs,
) -> CanisterCallResult<Icrc1TransferResult> {
    call_canister::<_, (Icrc1TransferResult,)>(canister_id, "icrc1_transfer", (args,))
        .await
        .map(fetch_tuple0)
}
/// 进行转账
#[allow(unused)]
pub async fn icrc1_transfer_by(
    canister_id: CanisterId,
    amount: candid::Nat,
    owner: UserId,
    subaccount: Option<Icrc1Subaccount>,
) -> CanisterCallResult<Icrc1TransferResult> {
    icrc1_transfer(
        canister_id,
        Icrc1TransferArgs {
            from_subaccount: None,
            to: icrc_ledger_types::icrc1::account::Account {
                owner,
                subaccount: subaccount.map(|s| s.0),
            },
            amount,
            fee: None,
            memo: None,
            created_at_time: None,
        },
    )
    .await
}
