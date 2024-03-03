// ================== 罐子调用产生的错误信息 ==================
// 罐子调用会产生的错误
#[derive(Debug)]
pub struct CanisterCallError {
    pub canister_id: crate::identity::CanisterId,
    pub method: String,
    pub rejection_code: ic_cdk::api::call::RejectionCode,
    pub message: std::string::String,
}
impl std::fmt::Display for CanisterCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Canister({}) call({}) failed. {:?} ({})",
            self.canister_id.to_text(),
            self.method,
            self.rejection_code,
            self.message
        )
    }
}
impl std::error::Error for CanisterCallError {}

pub type CanisterCallResult<T> = Result<T, CanisterCallError>;

// ===================== 常用模块 =====================

// #[cfg(feature = "canister_call")]
// pub use super::deploy::{CanisterInfo, CanisterInfoShow};

// #[cfg(feature = "canister_managed")]
// pub use super::managed::{
//     ManagedCanisterChecking, ManagedCanisterConfig, ManagedCanisterInitial,
//     ManagedCanisterMaintained, ManagedCanisterRefreshResult, ManagedCanisterStates,
// };
