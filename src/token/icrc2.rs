use candid::{CandidType, Deserialize};

use crate::{
    canister::{call::call_canister, fetch_tuple0, types::CanisterCallResult},
    identity::CanisterId,
};

use super::types::{Icrc1Account, Icrc1Subaccount};

/// ICRC2 标准接口

// ICRC2 标准
// icrc2_allowance : (AllowanceArgs) -> (Allowance) query;
// icrc2_approve : (ApproveArgs) -> (Result_1);
// icrc2_transfer_from : (TransferFromArgs) -> (Result_2);

// =================== 账本方法 ===================

// https://dashboard.internetcomputer.org/canister/ryjl3-tyaaa-aaaaa-aaaba-cai

//  ============== 查询是否授权 ==============
// icrc2_allowance : (AllowanceArgs) -> (Allowance) query;
// type AllowanceArgs = record { account : Account; spender : Account };
// type Allowance = record { allowance : nat; expires_at : opt nat64 };

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Icrc2AllowanceArgs {
    pub account: Icrc1Account,
    pub spender: Icrc1Account,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Icrc2Allowance {
    pub allowance: candid::Nat,
    pub expires_at: Option<u64>,
}

#[allow(unused)]
pub async fn icrc2_allowance(
    canister_id: CanisterId,
    args: Icrc2AllowanceArgs,
) -> CanisterCallResult<Icrc2Allowance> {
    call_canister::<_, (Icrc2Allowance,)>(canister_id, "icrc2_allowance", (args,))
        .await
        .map(fetch_tuple0)
}

//  ============== 进行授权 ==============
// icrc2_approve : (ApproveArgs) -> (Result_1);
// type ApproveArgs = record {
//   fee : opt nat;
//   memo : opt vec nat8;
//   from_subaccount : opt vec nat8;
//   created_at_time : opt nat64;
//   amount : nat;
//   expected_allowance : opt nat;
//   expires_at : opt nat64;
//   spender : Account;
// };
// type Result_1 = variant { Ok : nat; Err : ApproveError };
// type ApproveError = variant {
//   GenericError : record { message : text; error_code : nat };
//   TemporarilyUnavailable;
//   Duplicate : record { duplicate_of : nat };
//   BadFee : record { expected_fee : nat };
//   AllowanceChanged : record { current_allowance : nat };
//   CreatedInFuture : record { ledger_time : nat64 };
//   TooOld;
//   Expired : record { ledger_time : nat64 };
//   InsufficientFunds : record { balance : nat };
// };

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Icrc2ApproveArgs {
    pub from_subaccount: Option<Icrc1Subaccount>,
    pub spender: Icrc1Account, // 被授权的账户
    pub amount: candid::Nat,   // 被授权的额度
    pub fee: Option<candid::Nat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,

    pub expected_allowance: Option<candid::Nat>, // 期望转移的数量
    pub expires_at: Option<u64>,                 // 过期时间
}
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum Icrc2ApproveError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    AllowanceChanged {
        current_allowance: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
    Expired {
        ledger_time: u64,
    },
    InsufficientFunds {
        balance: candid::Nat,
    },
}

pub type Icrc2ApproveResult = Result<candid::Nat, Icrc2ApproveError>;

#[allow(unused)]
pub async fn icrc2_approve(
    canister_id: CanisterId,
    args: Icrc2ApproveArgs,
) -> CanisterCallResult<Icrc2ApproveResult> {
    call_canister::<_, (Icrc2ApproveResult,)>(canister_id, "icrc2_approve", (args,))
        .await
        .map(fetch_tuple0)
}

//  ============== 转账 ==============
// icrc2_transfer_from : (TransferFromArgs) -> (Result_2);
// type TransferFromArgs = record {
//   to : Account;
//   fee : opt nat;
//   spender_subaccount : opt vec nat8;
//   from : Account;
//   memo : opt vec nat8;
//   created_at_time : opt nat64;
//   amount : nat;
// };
// type Result_2 = variant { Ok : nat; Err : TransferFromError };
// type TransferFromError = variant {
//   GenericError : record { message : text; error_code : nat };
//   TemporarilyUnavailable;
//   InsufficientAllowance : record { allowance : nat };
//   BadBurn : record { min_burn_amount : nat };
//   Duplicate : record { duplicate_of : nat };
//   BadFee : record { expected_fee : nat };
//   CreatedInFuture : record { ledger_time : nat64 };
//   TooOld;
//   InsufficientFunds : record { balance : nat };
// };

// 转账参数
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Icrc2TransferFromArgs {
    spender_subaccount: Option<Icrc1Subaccount>,
    from: Icrc1Account,
    to: Icrc1Account,
    amount: candid::Nat,
    fee: Option<candid::Nat>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}
// 转账可能出现的错误
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum Icrc2TransferFromError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    InsufficientAllowance {
        allowance: candid::Nat,
    },
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
pub type Icrc1TransferFromResult = Result<candid::Nat, Icrc2TransferFromError>;
#[allow(unused)]
pub async fn icrc2_transfer_from(
    canister_id: CanisterId,
    args: Icrc2TransferFromArgs,
) -> CanisterCallResult<Icrc1TransferFromResult> {
    call_canister::<_, (Icrc1TransferFromResult,)>(canister_id, "icrc2_transfer_from", (args,))
        .await
        .map(fetch_tuple0)
}
