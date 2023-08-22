#[cfg(feature = "token_ledger")]
pub mod ledger;

#[cfg(feature = "token_icrc1")]
pub mod icrc1;

pub mod types;

type CallError = (ic_cdk::api::call::RejectionCode, std::string::String);
