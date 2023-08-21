#[cfg(feature = "identity")]
pub mod identity;

pub mod canister;


#[cfg(feature = "canister_call")]
pub mod canister_call;

#[cfg(feature = "canister_managed")]
pub mod canister_managed;

#[cfg(feature = "times")]
pub mod times;

#[cfg(feature = "random")]
pub mod random;

#[cfg(feature = "mix")]
pub mod mix;

#[cfg(feature = "ledger")]
pub mod ledger;

#[cfg(feature = "logs")]
pub mod logs;

#[cfg(feature = "pages")]
pub mod pages;

#[cfg(feature = "results")]
pub mod results;

#[cfg(feature = "tasks")]
pub mod tasks;

#[cfg(feature = "stable")]
pub mod stable;

#[cfg(feature = "initial")]
pub mod initial;

#[cfg(feature = "permissions")]
pub mod permissions;

#[cfg(feature = "maintainable")]
pub mod maintainable;

#[cfg(feature = "uploads")]
pub mod uploads;

#[cfg(feature = "nft")]
pub mod nft;

#[cfg(feature = "http")]
pub mod http;

pub mod types;
