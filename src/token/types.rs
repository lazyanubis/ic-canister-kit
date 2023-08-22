// use crate::identity::{AccountIdentifierHex, UserId};

// 暂时不知道哪里使用
// 转账的目标用户
// #[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Debug, Clone)]
// pub enum TransferUser {
//     #[serde(rename = "address")]
//     Address(AccountIdentifierHex),
//     #[serde(rename = "principal")]
//     Principal(UserId),
// }

#[cfg(feature = "token_ledger")]
pub use super::ledger::{
    LedgerAccountIdentifier, LedgerBinaryAccountBalanceArgs, LedgerBlockIndex, LedgerDecimals,
    LedgerName, LedgerSubAccount, LedgerSymbol, LedgerTimestamp, LedgerTokens, LedgerTransferArgs,
    LedgerTransferError, LedgerTransferFee, LedgerTransferFeeArg, LedgerTransferResult,
};
