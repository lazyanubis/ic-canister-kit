#![doc = include_str!("../README.md")]
#![deny(unreachable_pub)] // ! lib 需要检查此项
#![deny(unsafe_code)] // 拒绝 unsafe 代码
#![deny(missing_docs)] // ! 必须写文档
#![warn(rustdoc::broken_intra_doc_links)] // 文档里面的链接有效性
#![warn(clippy::future_not_send)] // 异步代码关联的对象必须是 Send 的
#![deny(clippy::unwrap_used)] // 不许用 unwrap
#![deny(clippy::expect_used)] // 不许用 expect
#![deny(clippy::panic)] // 不许用 panic

/// 通用工具
#[cfg(feature = "common")]
pub mod common;

/// 时间相关
#[cfg(feature = "times")]
pub mod times;

/// 身份相关
#[cfg(feature = "identity")]
pub mod identity;

/// 罐子相关
#[cfg(feature = "canister")]
pub mod canister;

/// 数字相关
#[cfg(feature = "number")]
pub mod number;

/// 代币相关
#[cfg(feature = "token")]
pub mod token;

/// http 请求相关
#[cfg(feature = "http")]
pub mod http;

/// 签名相关
#[cfg(feature = "ecdsa")]
pub mod ecdsa;

/// 比特币相关
#[cfg(feature = "bitcoin")]
pub mod bitcoin;

/// 特殊功能
#[cfg(feature = "functions")]
pub mod functions;

/// 功能实现
#[cfg(feature = "stable")]
pub mod stable;

// #[cfg(feature = "nft")]
// pub mod nft;

// #[cfg(feature = "candid_type")]
// pub mod candid;

/// 所有类型
pub mod types;
