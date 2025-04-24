//! icp 类的账本罐子接口

use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    canister::{call::call_canister, types::CanisterCallResult},
    identity::{AccountIdentifier, CanisterId, Subaccount},
};

// Ledger 标准
// name : () -> (Name) query;
// symbol : () -> (Symbol) query;
// decimals : () -> (Decimals) query;
// account_balance : (BinaryAccountBalanceArgs) -> (Tokens) query;
// transfer_fee : (record {}) -> (TransferFee) query;
// transfer : (TransferArgs) -> (Result_1);
// 下面的接口应该用不到
// archives : () -> (Archives) query;
// query_blocks : (GetBlocksArgs) -> (QueryBlocksResponse) query;
// query_encoded_blocks : (GetBlocksArgs) -> (QueryEncodedBlocksResponse) query;

// =================== 账本方法 ===================

// https://dashboard.internetcomputer.org/canister/ryjl3-tyaaa-aaaaa-aaaba-cai

//  ============== 查询名称 ==============
// name : () -> (Name) query;
// type Name = record { name : text };

/// 账本名称
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct LedgerName {
    name: String,
}

/// 查询名称
#[allow(unused)]
pub async fn ledger_name(canister_id: CanisterId) -> CanisterCallResult<LedgerName> {
    call_canister::<_, LedgerName>(canister_id, "name", ()).await
}
/// 查询名称
#[allow(unused)]
pub async fn ledger_name_by(canister_id: CanisterId) -> CanisterCallResult<String> {
    ledger_name(canister_id).await.map(|n| n.name)
}

//  ============== 查询符号 ==============
// symbol : () -> (Symbol) query;
// type Symbol = record { symbol : text };

/// symbol
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct LedgerSymbol {
    symbol: String,
}
/// 查询 symbol
#[allow(unused)]
pub async fn ledger_symbol(canister_id: CanisterId) -> CanisterCallResult<LedgerSymbol> {
    call_canister::<_, LedgerSymbol>(canister_id, "symbol", ()).await
}
/// 查询 symbol
#[allow(unused)]
pub async fn ledger_symbol_by(canister_id: CanisterId) -> CanisterCallResult<String> {
    ledger_symbol(canister_id).await.map(|s| s.symbol)
}

//  ============== 查询精度 ==============
// decimals : () -> (Decimals) query;
// type Decimals = record { decimals : nat32 };

/// 精度
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct LedgerDecimals {
    decimals: u32,
}
/// 查询精度
#[allow(unused)]
pub async fn ledger_decimals(canister_id: CanisterId) -> CanisterCallResult<LedgerDecimals> {
    call_canister::<_, LedgerDecimals>(canister_id, "decimals", ()).await
}
/// 查询精度
#[allow(unused)]
pub async fn ledger_decimals_by(canister_id: CanisterId) -> CanisterCallResult<u32> {
    ledger_decimals(canister_id).await.map(|d| d.decimals)
}

//  ============== 查询余额 ==============
// account_balance : (BinaryAccountBalanceArgs) -> (Tokens) query;
// type BinaryAccountBalanceArgs = record { account : vec nat8 };
// type Tokens = record { e8s : nat64 };

/// 查询余额参数
pub type LedgerBinaryAccountBalanceArgs = ic_ledger_types::AccountBalanceArgs;
// #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
// pub struct LedgerBinaryAccountBalanceArgs {
//     /// 账户识别
//     pub account: LedgerAccountIdentifier,
// }
/// 余额
pub type LedgerTokens = ic_ledger_types::Tokens;
// #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
// pub struct LedgerTokens {
//     ///  ICP 接口指定 8 位精度的数值
//     pub e8s: u64,
// }
/// 查询余额
#[allow(unused)]
pub async fn ledger_account_balance(
    canister_id: CanisterId,
    args: LedgerBinaryAccountBalanceArgs,
) -> CanisterCallResult<LedgerTokens> {
    call_canister::<_, LedgerTokens>(canister_id, "account_balance", (args,)).await
}
/// 查询余额
#[allow(unused)]
pub async fn ledger_account_balance_by(
    canister_id: CanisterId,
    account: LedgerAccountIdentifier,
) -> CanisterCallResult<u64> {
    ledger_account_balance(canister_id, LedgerBinaryAccountBalanceArgs { account })
        .await
        .map(|b| b.e8s())
}

//  ============== 查询转账费用 ==============
// transfer_fee : (record {}) -> (TransferFee) query;
// type TransferFee = record { transfer_fee : Tokens };

/// 查询转账费用参数
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct LedgerTransferFeeArg {}

/// 转账费用
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct LedgerTransferFee {
    /// The fee to pay to perform a transfer
    pub transfer_fee: LedgerTokens,
}
/// 查询转账费用
#[allow(unused)]
pub async fn ledger_transfer_fee(
    canister_id: CanisterId,
    // args: LedgerTransferFeeArg,
) -> CanisterCallResult<LedgerTransferFee> {
    call_canister::<_, LedgerTransferFee>(
        canister_id,
        "transfer_fee",
        // (args,),
        (LedgerTransferFeeArg {},),
    )
    .await
}
/// 查询转账费用, 简化参数
#[allow(unused)]
pub async fn ledger_transfer_fee_by(canister_id: CanisterId) -> CanisterCallResult<u64> {
    ledger_transfer_fee(
        canister_id,
        // LedgerTransferFeeArg {}
    )
    .await
    .map(|f| f.transfer_fee.e8s())
}

//  ============== 转账 ==============
// transfer : (TransferArgs) -> (Result_1);
// type TimeStamp = record { timestamp_nanos : nat64 };
// type Tokens = record { e8s : nat64 };
// type TransferArgs = record {
//   to : vec nat8;
//   fee : Tokens;
//   memo : nat64;
//   from_subaccount : opt vec nat8;
//   created_at_time : opt TimeStamp;
//   amount : Tokens;
// };
// type Result_1 = variant { Ok : nat64; Err : TransferError_1 };
// type TransferError_1 = variant {
//   TxTooOld : record { allowed_window_nanos : nat64 };
//   BadFee : record { expected_fee : Tokens };
//   TxDuplicate : record { duplicate_of : nat64 };
//   TxCreatedInFuture;
//   InsufficientFunds : record { balance : Tokens };
// };

/// 转账标识
pub type LedgerMemo = u64; // 转账需要记录的标识码
/// 账本时间戳
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct LedgerTimestamp {
    /// 时间戳, 纳秒
    pub timestamp_nanos: u64,
}
/// 账户 ID 是长度为 32 的byte数组
/// 前 4 位是大端法编码的后面 28 位数字的 CRC32 校验码
/// The first 4 bytes is big-endian encoding of a CRC32 checksum of the last 28 bytes.
// pub type LedgerAccountIdentifier = Vec<u8>;
pub type LedgerAccountIdentifier = AccountIdentifier; // ! 修改为安全的参数

/// 子账户是任意长度为 32 的byte数组
/// 使用子账户机制, 让一个用户 principal 控制大量的账本账户
/// Ledger uses subaccounts to compute the source address, which enables one
/// principal to control multiple ledger accounts.
// pub type LedgerSubaccount = Vec<u8>;
pub type LedgerSubaccount = Subaccount; // ! 修改为安全的参数

/// 转账参数
pub type LedgerTransferArgs = ic_ledger_types::TransferArgs;
// #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
// pub struct LedgerTransferArgs {
//     /// 调用者指定的使用的子账户地址
//     /// 如果没有, 则默认全 0 的子账户
//     /// The subaccount from which the caller wants to transfer funds.
//     /// If null, the ledger uses the default (all zeros) subaccount to compute the source address.
//     pub from_subaccount: Option<LedgerSubaccount>,
//     /// 目标地址, 长度为 32 的byte数组, 转账成功, 目标地址的余额会增加 amount 的数量
//     /// The destination account. If the transfer is successful, the balance of this address increases by `amount`.
//     pub to: LedgerAccountIdentifier,
//     /// 想要转给目标地址的数量
//     pub amount: LedgerTokens,
//     /// 调用者必须支付的交易费, 必须是 10000 e8s
//     /// The amount that the caller pays for the transaction. Must be 10000 e8s.
//     pub fee: LedgerTokens,
//     /// 交易标识码 u64的数字
//     pub memo: LedgerMemo,
//     /// 请求的时间节点, 如果是空, 则默认 IC 系统当前时间
//     /// The point in time when the caller created this request. If null, the ledger uses current IC time as the timestamp.
//     pub created_at_time: Option<LedgerTimestamp>,
// }

///  转账成功后返回的交易高度
pub type LedgerBlockIndex = u64;

/// 转账可能出现的错误
pub type LedgerTransferError = ic_ledger_types::TransferError;
// #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
// pub enum LedgerTransferError {
//     /// 手续费不正确
//     /// The fee that the caller specified in the transfer request was not the one that ledger expects.
//     /// The caller can change the transfer fee to the `expected_fee` and retry the request.
//     BadFee {
//         /// 期望的手续费
//         expected_fee: LedgerTokens,
//     },
//     /// 余额不足
//     /// The account specified by the caller doesn't have enough funds.
//     InsufficientFunds {
//         /// 余额不足
//         balance: LedgerTokens,
//     },
//     /// 交易过期了, 请求时间太早了, 距离 IC 系统当前时间 24 小时内的请求可以被接受
//     /// The request is too old.
//     /// The ledger only accepts requests created within 24 hours window.
//     /// This is a non-recoverable error.
//     TxTooOld {
//         /// 允许的时间窗口
//         allowed_window_nanos: u64,
//     },
//     /// 未来的交易, 指定交易时间在未来
//     /// The caller specified `created_at_time` that is too far in future.
//     /// The caller can retry the request later.
//     TxCreatedInFuture,
//     /// 重复交易 // ! 猜测是通过交易请求时间判断是否重复的
//     /// The ledger has already executed the request.
//     /// `duplicate_of` field is equal to the index of the block containing the original transaction.
//     TxDuplicate {
//         /// 重复的交易
//         duplicate_of: LedgerBlockIndex,
//     },
// }
/// 转账结果
pub type LedgerTransferResult = Result<LedgerBlockIndex, LedgerTransferError>;

/// 进行转账
#[allow(unused)]
pub async fn ledger_transfer(
    canister_id: CanisterId,
    args: LedgerTransferArgs,
) -> CanisterCallResult<LedgerTransferResult> {
    call_canister::<_, LedgerTransferResult>(canister_id, "transfer", (args,)).await
}
/// 进行转账
#[allow(unused)]
pub async fn ledger_transfer_by(
    canister_id: CanisterId,
    to: LedgerAccountIdentifier,
    amount: u64,
    fee: u64,
    memo: u64,
) -> CanisterCallResult<LedgerTransferResult> {
    ledger_transfer(
        canister_id,
        LedgerTransferArgs {
            from_subaccount: None,
            to,
            amount: LedgerTokens::from_e8s(amount),
            fee: LedgerTokens::from_e8s(fee),
            memo: ic_ledger_types::Memo(memo),
            created_at_time: None,
        },
    )
    .await
}
