#[cfg(feature = "canister_cycles")]
pub use super::cycles::WalletReceiveResult;

#[cfg(feature = "canister_status")]
pub use super::status::{CanisterInfoResult, CanisterStatusResult};

#[cfg(feature = "canister_call")]
pub use super::deploy::{CanisterInfo, CanisterInfoShow};

#[cfg(feature = "canister_managed")]
pub use super::managed::{
    ManagedCanisterChecking, ManagedCanisterConfig, ManagedCanisterInitial,
    ManagedCanisterMaintained, ManagedCanisterRefreshResult, ManagedCanisterStates,
};

// 罐子调用会产生的错误
pub type CallError = (ic_cdk::api::call::RejectionCode, std::string::String);
