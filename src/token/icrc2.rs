use candid::{CandidType, Deserialize};

use crate::{
    canister::{call::call_canister, fetch_tuple0, types::CanisterCallResult},
    identity::CanisterId,
};

use super::{
    icrc1::Icrc1Memo,
    types::{Icrc1Account, Icrc1Subaccount},
};

/// ICRC2 标准接口
/// https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-2

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

/// 授权对象
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Icrc2AllowanceArgs {
    /// 被授权的账户 出钱的
    pub account: Icrc1Account,

    /// 授权的账户 花钱的
    pub spender: Icrc1Account,
}

/// 授权额度
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Icrc2Allowance {
    /// 授权的额度
    pub allowance: candid::Nat,
    /// 过期时间
    pub expires_at: Option<u64>,
}

/// 查询授权额度
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

/// 授权参数
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Icrc2ApproveArgs {
    /// 被授权的子账户
    pub from_subaccount: Option<Icrc1Subaccount>,

    /// 授权的账户
    pub spender: Icrc1Account,
    /// 授权的额度
    pub amount: candid::Nat,

    /// 授权的费用
    pub fee: Option<candid::Nat>,

    /// memo
    pub memo: Option<Icrc1Memo>,
    /// 创建时间
    pub created_at_time: Option<u64>,

    /// 期望转移的数量
    pub expected_allowance: Option<candid::Nat>,
    /// 过期时间
    pub expires_at: Option<u64>,
}

/// 授权错误
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum Icrc2ApproveError {
    /// 通用错误
    GenericError {
        /// 错误码
        error_code: candid::Nat,
        /// 错误消息
        message: String,
    },
    /// 临时不可用
    TemporarilyUnavailable,
    /// 交易重复
    Duplicate {
        /// 交易重复
        duplicate_of: candid::Nat,
    },
    /// 错误的费用
    BadFee {
        /// 期望的费用
        expected_fee: candid::Nat,
    },
    /// 授权额度发生变化
    AllowanceChanged {
        /// 当前的授权额度
        current_allowance: candid::Nat,
    },
    /// 未来的转账
    CreatedInFuture {
        /// 当前账本时间
        ledger_time: u64,
    },
    /// 订单太老
    TooOld,
    /// 订单过期
    Expired {
        /// 当前账本时间
        ledger_time: u64,
    },
    /// 余额不足
    InsufficientFunds {
        /// 当前余额
        balance: candid::Nat,
    },
}

/// 授权结果
pub type Icrc2ApproveResult = Result<candid::Nat, Icrc2ApproveError>;

/// 授权额度
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

/// 转账参数
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Icrc2TransferFromArgs {
    spender_subaccount: Option<Icrc1Subaccount>,
    from: Icrc1Account,
    to: Icrc1Account,
    amount: candid::Nat,
    fee: Option<candid::Nat>,
    memo: Option<Icrc1Memo>,
    created_at_time: Option<u64>,
}
/// 转账可能出现的错误
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum Icrc2TransferFromError {
    /// 通用错误
    GenericError {
        /// 错误码
        error_code: candid::Nat,
        /// 错误消息
        message: String,
    },
    /// 临时不可用
    TemporarilyUnavailable,
    /// 授权额度不足
    InsufficientAllowance {
        /// 授权额度
        allowance: candid::Nat,
    },
    /// 错误的燃烧数量
    BadBurn {
        /// 最小的燃烧数量
        min_burn_amount: candid::Nat,
    },
    /// 交易重复
    Duplicate {
        /// 重复的交易
        duplicate_of: candid::Nat,
    },
    /// 错误的费用
    BadFee {
        /// 期望的费用
        expected_fee: candid::Nat,
    },
    /// 未来的转账
    CreatedInFuture {
        /// 当前账本时间
        ledger_time: u64,
    },
    /// 订单太老
    TooOld,
    /// 余额不足
    InsufficientFunds {
        /// 当前余额
        balance: candid::Nat,
    },
}
/// 转账结果
pub type Icrc1TransferFromResult = Result<candid::Nat, Icrc2TransferFromError>;

/// 通过授权进行转账
#[allow(unused)]
pub async fn icrc2_transfer_from(
    canister_id: CanisterId,
    args: Icrc2TransferFromArgs,
) -> CanisterCallResult<Icrc1TransferFromResult> {
    call_canister::<_, (Icrc1TransferFromResult,)>(canister_id, "icrc2_transfer_from", (args,))
        .await
        .map(fetch_tuple0)
}
