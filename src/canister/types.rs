#[cfg(feature = "canister_cycles")]
pub use super::cycles::WalletReceiveResult;

#[cfg(feature = "canister_status")]
pub use super::status::CanisterStatusResult;
