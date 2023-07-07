use super::identity::{AccountIdentifierHex, UserId};

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct Balance {
    pub e8s: u64, // 价格，icp，注意是 10^8 形式的
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Price {
    pub e8s: u64, // 价格，icp，注意是 10^8 形式的
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct TransferFee {
    pub e8s: u64, // 价格，icp，注意是 10^8 形式的
}

#[allow(non_camel_case_types)]
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Debug, Clone)]
pub enum TransferUser {
    address(AccountIdentifierHex),
    principal(UserId),
}

pub mod ledger_canister {
    use crate::identity::CanisterId;
    use candid::{CandidType, Deserialize};

    // =================== 基本类型 ======================

    pub type BlockIndex = u64; // 转账成功后返回

    pub type Memo = u64;

    #[derive(CandidType, Deserialize, Debug, Clone)]
    pub struct Tokens {
        pub e8s: u64,
    }

    #[derive(CandidType, Deserialize, Debug, Clone)]
    pub struct TimeStamp {
        pub timestamp_nanos: u64,
    }

    // AccountIdentifier is a 32-byte array.
    // The first 4 bytes is big-endian encoding of a CRC32 checksum of the last 28 bytes.
    pub type AccountIdentifier = Vec<u8>;

    // Subaccount is an arbitrary 32-byte byte array.
    // Ledger uses subaccounts to compute the source address, which enables one
    // principal to control multiple ledger accounts.
    type SubAccount = Vec<u8>;

    #[derive(CandidType, Deserialize, Debug, Clone)]
    pub struct TransferArgs {
        pub memo: Memo,     // Transaction memo. See comments for the `Memo` type.
        pub amount: Tokens, // The amount that the caller wants to transfer to the destination address.
        pub fee: Tokens, // The amount that the caller pays for the transaction. Must be 10000 e8s.
        // The subaccount from which the caller wants to transfer funds.
        // If null, the ledger uses the default (all zeros) subaccount to compute the source address.
        // See comments for the `SubAccount` type.
        pub from_subaccount: Option<SubAccount>,
        // The destination account. If the transfer is successful, the balance of this address increases by `amount`.
        pub to: AccountIdentifier,
        // The point in time when the caller created this request. If null, the ledger uses current IC time as the timestamp.
        pub created_at_time: Option<TimeStamp>,
    }

    #[derive(CandidType, Deserialize, Debug, Clone)]
    pub enum TransferError {
        // The fee that the caller specified in the transfer request was not the one that ledger expects.
        // The caller can change the transfer fee to the `expected_fee` and retry the request.
        BadFee { expected_fee: Tokens },
        // The account specified by the caller doesn't have enough funds.
        InsufficientFunds { balance: Tokens },
        // The request is too old.
        // The ledger only accepts requests created within 24 hours window.
        // This is a non-recoverable error.
        TxTooOld { allowed_window_nanos: u64 },
        // The caller specified `created_at_time` that is too far in future.
        // The caller can retry the request later.
        TxCreatedInFuture,
        // The ledger has already executed the request.
        // `duplicate_of` field is equal to the index of the block containing the original transaction.
        TxDuplicate { duplicate_of: BlockIndex },
    }

    pub type TransferResult = Result<BlockIndex, TransferError>;

    // Arguments for the `account_balance` call.
    #[derive(CandidType, Deserialize, Debug, Clone)]
    pub struct AccountBalanceArgs {
        pub account: AccountIdentifier,
    }

    #[derive(CandidType, Deserialize, Debug, Clone)]
    pub struct TransferFeeArg {}

    #[derive(CandidType, Deserialize, Debug, Clone)]
    pub struct TransferFee {
        // The fee to pay to perform a transfer
        pub transfer_fee: Tokens,
    }

    // =================== 账本方法 ===================

    /// icp 转账
    #[allow(unused)]
    pub async fn transfer(canister_id: CanisterId, args: TransferArgs) -> TransferResult {
        let _call_result: Result<
            (TransferResult,),
            (ic_cdk::api::call::RejectionCode, std::string::String),
        > = ic_cdk::call(canister_id, "transfer", (args,)).await;

        _call_result.unwrap().0
    }

    /// 查询余额
    #[allow(unused)]
    pub async fn account_balance(canister_id: CanisterId, args: AccountBalanceArgs) -> Tokens {
        let _call_result: Result<
            (Tokens,),
            (ic_cdk::api::call::RejectionCode, std::string::String),
        > = ic_cdk::call(canister_id, "account_balance", (args,)).await;

        _call_result.unwrap().0
    }

    /// 查询转账费用
    #[allow(unused)]
    pub async fn transfer_fee(canister_id: CanisterId, args: TransferFeeArg) -> TransferFee {
        let _call_result: Result<
            (TransferFee,),
            (ic_cdk::api::call::RejectionCode, std::string::String),
        > = ic_cdk::call(canister_id, "transfer_fee", (args,)).await;

        _call_result.unwrap().0
    }
}
