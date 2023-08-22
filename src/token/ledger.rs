/// icp 类的账本罐子接口
use crate::identity::CanisterId;
use candid::{CandidType, Deserialize};

// Ledger 标准
// name : () -> (Name) query;
// symbol : () -> (Symbol) query;
// decimals : () -> (Decimals) query;
// account_balance : (BinaryAccountBalanceArgs) -> (Tokens) query;
// transfer_fee : (record {}) -> (TransferFee) query;
// transfer : (TransferArgs) -> (Result_1);

// =================== 基本类型 ======================

type CallError = (ic_cdk::api::call::RejectionCode, std::string::String);

// =================== 账本方法 ===================

// https://icscan.io/canister/ryjl3-tyaaa-aaaaa-aaaba-cai

//  ============== 查询名称 ==============
// name : () -> (Name) query;
// type Name = record { name : text };

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct LedgerName {
    name: String,
}
#[allow(unused)]
pub async fn name(canister_id: CanisterId) -> LedgerName {
    let _call_result: Result<(LedgerName,), CallError> =
        ic_cdk::call(canister_id, "name", ()).await;
    _call_result.unwrap().0
}
#[allow(unused)]
pub async fn name_by(canister_id: CanisterId) -> String {
    name(canister_id).await.name
}

//  ============== 查询符号 ==============
// symbol : () -> (Symbol) query;
// type Symbol = record { symbol : text };

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct LedgerSymbol {
    symbol: String,
}
#[allow(unused)]
pub async fn symbol(canister_id: CanisterId) -> LedgerSymbol {
    let _call_result: Result<(LedgerSymbol,), CallError> =
        ic_cdk::call(canister_id, "symbol", ()).await;
    _call_result.unwrap().0
}
#[allow(unused)]
pub async fn symbol_by(canister_id: CanisterId) -> String {
    symbol(canister_id).await.symbol
}

//  ============== 查询精度 ==============
// decimals : () -> (Decimals) query;
// type Decimals = record { decimals : nat32 };

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct LedgerDecimals {
    decimals: u32,
}
#[allow(unused)]
pub async fn decimals(canister_id: CanisterId) -> LedgerDecimals {
    let _call_result: Result<(LedgerDecimals,), CallError> =
        ic_cdk::call(canister_id, "decimals", ()).await;
    _call_result.unwrap().0
}
#[allow(unused)]
pub async fn decimals_by(canister_id: CanisterId) -> u32 {
    decimals(canister_id).await.decimals
}

//  ============== 查询余额 ==============
// account_balance : (BinaryAccountBalanceArgs) -> (Tokens) query;
// type BinaryAccountBalanceArgs = record { account : vec nat8 };
// type Tokens = record { e8s : nat64 };

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct LedgerBinaryAccountBalanceArgs {
    pub account: LedgerAccountIdentifier,
}
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct LedgerTokens {
    pub e8s: u64, // ICP 接口指定 8 位精度的数值
}
#[allow(unused)]
pub async fn account_balance(
    canister_id: CanisterId,
    args: LedgerBinaryAccountBalanceArgs,
) -> LedgerTokens {
    let _call_result: Result<(LedgerTokens,), CallError> =
        ic_cdk::call(canister_id, "account_balance", (args,)).await;

    _call_result.unwrap().0
}
#[allow(unused)]
pub async fn account_balance_by(canister_id: CanisterId, account: LedgerAccountIdentifier) -> u64 {
    account_balance(canister_id, LedgerBinaryAccountBalanceArgs { account })
        .await
        .e8s
}

//  ============== 查询转账费用 ==============
// transfer_fee : (record {}) -> (TransferFee) query;
// type TransferFee = record { transfer_fee : Tokens };

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct LedgerTransferFeeArg {}
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct LedgerTransferFee {
    // The fee to pay to perform a transfer
    pub transfer_fee: LedgerTokens,
}
#[allow(unused)]
pub async fn transfer_fee(
    canister_id: CanisterId,
    args: LedgerTransferFeeArg,
) -> LedgerTransferFee {
    let _call_result: Result<(LedgerTransferFee,), CallError> =
        ic_cdk::call(canister_id, "transfer_fee", (args,)).await;

    _call_result.unwrap().0
}
/// 查询转账费用, 简化参数
#[allow(unused)]
pub async fn transfer_fee_by(canister_id: CanisterId) -> u64 {
    transfer_fee(canister_id, LedgerTransferFeeArg {})
        .await
        .transfer_fee
        .e8s
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

pub type LedgerMemo = u64; // 转账需要记录的标识码
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct LedgerTimestamp {
    pub timestamp_nanos: u64, // 时间戳, 纳秒
}
// 账户 ID 是长度为 32 的byte数组
// 前 4 位是大端法编码的后面 28 位数字的 CRC32 校验码
// The first 4 bytes is big-endian encoding of a CRC32 checksum of the last 28 bytes.
pub type LedgerAccountIdentifier = Vec<u8>;
// 子账户是任意长度为 32 的byte数组
// 使用子账户机制, 让一个用户 principal 控制大量的账本账户
// Ledger uses subaccounts to compute the source address, which enables one
// principal to control multiple ledger accounts.
pub type LedgerSubAccount = Vec<u8>;
// 转账参数
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct LedgerTransferArgs {
    // 调用者指定的使用的子账户地址
    // 如果没有, 则默认全 0 的子账户
    // The subaccount from which the caller wants to transfer funds.
    // If null, the ledger uses the default (all zeros) subaccount to compute the source address.
    pub from_subaccount: Option<LedgerSubAccount>,
    // 想要转给目标地址的数量
    pub amount: LedgerTokens,
    // 目标地址, 长度为 32 的byte数组, 转账成功, 目标地址的余额会增加 amount 的数量
    // The destination account. If the transfer is successful, the balance of this address increases by `amount`.
    pub to: LedgerAccountIdentifier,
    // 调用者必须支付的交易费, 必须是 10000 e8s
    // The amount that the caller pays for the transaction. Must be 10000 e8s.
    pub fee: LedgerTokens,
    // 交易标识码 u64的数字
    pub memo: LedgerMemo,
    // 请求的时间节点, 如果是空, 则默认 IC 系统当前时间
    // The point in time when the caller created this request. If null, the ledger uses current IC time as the timestamp.
    pub created_at_time: Option<LedgerTimestamp>,
}
pub type LedgerBlockIndex = u64; // 转账成功后返回的交易高度
                                 // 转账可能出现的错误
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum LedgerTransferError {
    // 手续费不正确
    // The fee that the caller specified in the transfer request was not the one that ledger expects.
    // The caller can change the transfer fee to the `expected_fee` and retry the request.
    BadFee { expected_fee: LedgerTokens },
    // 余额不足
    // The account specified by the caller doesn't have enough funds.
    InsufficientFunds { balance: LedgerTokens },
    // 交易过期了, 请求时间太早了, 距离 IC 系统当前时间 24 小时内的请求可以被接受
    // The request is too old.
    // The ledger only accepts requests created within 24 hours window.
    // This is a non-recoverable error.
    TxTooOld { allowed_window_nanos: u64 },
    // 未来的交易, 指定交易时间在未来
    // The caller specified `created_at_time` that is too far in future.
    // The caller can retry the request later.
    TxCreatedInFuture,
    // 重复交易 // ! 猜测是通过交易请求时间判断是否重复的
    // The ledger has already executed the request.
    // `duplicate_of` field is equal to the index of the block containing the original transaction.
    TxDuplicate { duplicate_of: LedgerBlockIndex },
}
// 转账结果
pub type LedgerTransferResult = Result<LedgerBlockIndex, LedgerTransferError>;

#[allow(unused)]
pub async fn transfer(canister_id: CanisterId, args: LedgerTransferArgs) -> LedgerTransferResult {
    let _call_result: Result<(LedgerTransferResult,), CallError> =
        ic_cdk::call(canister_id, "transfer", (args,)).await;
    _call_result.unwrap().0
}
#[allow(unused)]
pub async fn transfer_by(
    canister_id: CanisterId,
    amount: u64,
    to: LedgerAccountIdentifier,
    fee: u64,
    memo: u64,
) -> LedgerTransferResult {
    transfer(
        canister_id,
        LedgerTransferArgs {
            from_subaccount: None,
            amount: LedgerTokens { e8s: amount },
            to,
            fee: LedgerTokens { e8s: fee },
            memo,
            created_at_time: None,
        },
    )
    .await
}
