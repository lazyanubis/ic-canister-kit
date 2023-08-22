use candid::{CandidType, Deserialize};

use crate::identity::{CanisterId, UserId};

use super::CallError;

/// ICRC1 标准接口

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

// https://icscan.io/canister/ryjl3-tyaaa-aaaaa-aaaba-cai

//  ============== 查询支持的标准 ==============
// icrc1_supported_standards : () -> (vec StandardRecord) query;
// type StandardRecord = record { url : text; name : text };

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ICRC1SupportedStandard {
    name: String, // ICRC-1
    url: String,  // https://github.com/dfinity/ICRC-1
}
pub type ICRC1SupportedStandards = Vec<ICRC1SupportedStandard>;
#[allow(unused)]
pub async fn icrc1_supported_standards(canister_id: CanisterId) -> ICRC1SupportedStandards {
    let _call_result: Result<(ICRC1SupportedStandards,), CallError> =
        ic_cdk::call(canister_id, "icrc1_supported_standards", ()).await;
    _call_result.unwrap().0
}
#[allow(unused)]
pub async fn icrc1_supported_standards_by(canister_id: CanisterId) -> Vec<String> {
    icrc1_supported_standards(canister_id)
        .await
        .iter()
        .map(|s| s.name.clone())
        .collect()
}

//  ============== 查询名称 ==============
// icrc1_name : () -> (text) query;

#[allow(unused)]
pub async fn icrc1_name(canister_id: CanisterId) -> String {
    let _call_result: Result<(String,), CallError> =
        ic_cdk::call(canister_id, "icrc1_name", ()).await;
    _call_result.unwrap().0
}

//  ============== 查询符号 ==============
// icrc1_symbol : () -> (text) query;

#[allow(unused)]
pub async fn icrc1_symbol(canister_id: CanisterId) -> String {
    let _call_result: Result<(String,), CallError> =
        ic_cdk::call(canister_id, "icrc1_symbol", ()).await;
    _call_result.unwrap().0
}

//  ============== 查询精度 ==============
// icrc1_decimals : () -> (nat8) query;

#[allow(unused)]
pub async fn icrc1_decimals(canister_id: CanisterId) -> u8 {
    let _call_result: Result<(u8,), CallError> =
        ic_cdk::call(canister_id, "icrc1_decimals", ()).await;
    _call_result.unwrap().0
}

//  ============== 查询总供应量 ==============
// icrc1_decimals : () -> (nat8) query;

pub type ICRC1TotalSupply = candid::Nat;
#[allow(unused)]
pub async fn icrc1_total_supply(canister_id: CanisterId) -> ICRC1TotalSupply {
    let _call_result: Result<(ICRC1TotalSupply,), CallError> =
        ic_cdk::call(canister_id, "icrc1_total_supply", ()).await;
    _call_result.unwrap().0
}

//  ============== 查询余额 ==============
// icrc1_balance_of : (Account) -> (nat) query;
// type Account = record { owner : principal; subaccount : opt vec nat8 };

pub type ICRC1Subaccount = Vec<u8>;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ICRC1Account {
    owner: UserId,
    subaccount: Option<ICRC1Subaccount>,
}
pub type ICRC1Balance = candid::Nat;
#[allow(unused)]
pub async fn icrc1_balance_of(canister_id: CanisterId, account: ICRC1Account) -> ICRC1Balance {
    let _call_result: Result<(ICRC1Balance,), CallError> =
        ic_cdk::call(canister_id, "icrc1_balance_of", ()).await;
    _call_result.unwrap().0
}
#[allow(unused)]
pub async fn icrc1_balance_of_by(
    canister_id: CanisterId,
    owner: UserId,
    subaccount: Option<ICRC1Subaccount>,
) -> ICRC1Balance {
    icrc1_balance_of(canister_id, ICRC1Account { owner, subaccount }).await
}

//  ============== 查询交易费用 ==============
// icrc1_fee : () -> (nat) query;

pub type ICRC1Fee = candid::Nat;
#[allow(unused)]
pub async fn icrc1_fee(canister_id: CanisterId) -> ICRC1Fee {
    let _call_result: Result<(ICRC1Fee,), CallError> =
        ic_cdk::call(canister_id, "icrc1_fee", ()).await;
    _call_result.unwrap().0
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

// 转账参数
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ICRC1TransferArgs {
    // 调用者指定的使用的子账户地址
    pub from_subaccount: Option<ICRC1Subaccount>,
    // 想要转给目标地址的数量
    pub amount: candid::Nat,
    // 目标地址
    pub to: ICRC1Account,
    // 交易费
    pub fee: Option<candid::Nat>,
    // 交易标识码
    pub memo: Option<Vec<u8>>,
    // 请求的时间节点
    pub created_at_time: Option<u64>,
}
// 转账可能出现的错误
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ICRC1TransferError {
    GenericError {
        error_code: candid::Nat,
        message: String,
    },
    TemporarilyUnavailable, // ? 临时不可用?
    BadBurn {
        min_burn_amount: candid::Nat,
    },
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
    InsufficientFunds {
        balance: candid::Nat,
    },
}
// 转账结果
pub type ICRC1TransferResult = Result<candid::Nat, ICRC1TransferError>;
#[allow(unused)]
pub async fn icrc1_transfer(
    canister_id: CanisterId,
    args: ICRC1TransferArgs,
) -> ICRC1TransferResult {
    let _call_result: Result<(ICRC1TransferResult,), CallError> =
        ic_cdk::call(canister_id, "icrc1_transfer", (args,)).await;
    _call_result.unwrap().0
}
#[allow(unused)]
pub async fn icrc1_transfer_by(
    canister_id: CanisterId,
    amount: candid::Nat,
    owner: UserId,
    subaccount: Option<ICRC1Subaccount>,
) -> ICRC1TransferResult {
    icrc1_transfer(
        canister_id,
        ICRC1TransferArgs {
            from_subaccount: None,
            amount,
            to: ICRC1Account { owner, subaccount },
            fee: None,
            memo: None,
            created_at_time: None,
        },
    )
    .await
}
