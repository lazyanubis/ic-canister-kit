use std::fmt::Display;

use candid::CandidType;
use serde::Deserialize;

// ================== 罐子调用产生的错误信息 ==================

/// 罐子调用会产生的错误
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CanisterCallError {
    /// 罐子 id
    pub canister_id: crate::identity::CanisterId,

    /// 调用的方法
    pub method: String,

    /// 错误消息
    pub message: std::string::String,
}
impl std::fmt::Display for CanisterCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Canister({}) call({}) failed: {}",
            self.canister_id.to_text(),
            self.method,
            self.message
        )
    }
}
impl std::error::Error for CanisterCallError {}

impl CanisterCallError {
    /// 新建
    pub fn new<E: Display>(
        canister_id: crate::identity::CanisterId,
        method: impl Into<String>,
        err: E,
    ) -> Self {
        Self {
            canister_id,
            method: method.into(),
            message: err.to_string(),
        }
    }

    /// 管理调用
    pub fn from<E: Display>(method: impl Into<String>, err: E) -> Self {
        Self {
            canister_id: crate::identity::CanisterId::management_canister(),
            method: method.into(),
            message: err.to_string(),
        }
    }
}

/// 罐子调用结果
pub type CanisterCallResult<T> = Result<T, CanisterCallError>;

// ===================== 常用模块 =====================

pub use super::codes::{CanisterCodeWasm, CanisterInitArg};
