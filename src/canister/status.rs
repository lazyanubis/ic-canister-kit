//! 和 罐子 的 状态 相关

use super::types::{CanisterInfoArgs, CanisterInfoResult, CanisterStatusArgs, CanisterStatusResult};
use crate::identity::CanisterId;

/*

引入包后, 直接使用如下方法即可增加查询罐子状态的接口

#[ic_cdk::update]
async fn canister_status() -> ic_canister_kit::types::CanisterStatusResult {
    use ic_canister_kit::{canister::status::canister_status, identity::self_canister_id};
    let response = canister_status(self_canister_id()).await;
    ic_canister_kit::common::trap(response)
}

#[ic_cdk::update]
async fn canister_info(num_requested_changes: Option<u64>) -> ic_canister_kit::types::CanisterInfoResult {
    use ic_canister_kit::{canister::status::canister_info, identity::self_canister_id};
    let response = canister_info(self_canister_id(), num_requested_changes).await;
    ic_canister_kit::common::trap(response)
}

*/
// ========================= 罐子状态信息 =========================

/// 查询罐子状态
/// ! Only the controllers of the canister
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-canister_status
pub async fn canister_status(canister_id: CanisterId) -> super::types::CanisterCallResult<CanisterStatusResult> {
    let call_result = ic_cdk_management_canister::canister_status(&CanisterStatusArgs { canister_id }).await;
    call_result.map_err(|err| crate::canister::types::CanisterCallError::new(canister_id, "ic#canister_status", err))
}

/// 查询罐子信息
/// ! 罐子可以调用，用户身份不可以调用
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-canister-info
pub async fn canister_info(
    canister_id: CanisterId,
    num_requested_changes: Option<u64>,
) -> super::types::CanisterCallResult<CanisterInfoResult> {
    let call_result = ic_cdk_management_canister::canister_info(&CanisterInfoArgs {
        canister_id,
        num_requested_changes,
    })
    .await;
    call_result.map_err(|err| crate::canister::types::CanisterCallError::new(canister_id, "ic#canister_status", err))
}

// ========================= 自定义接口调用 =========================

/// 查询罐子状态
/// ! 必须实现 canister_status : () -> (CanisterStatusResult) 接口
pub async fn call_canister_status(canister_id: CanisterId) -> super::types::CanisterCallResult<CanisterStatusResult> {
    let call_result = ic_cdk::call::Call::unbounded_wait(canister_id, "canister_status").await;
    super::fetch_and_wrap_call_result(canister_id, "canister_status", call_result)
}

/// 查询罐子信息
/// ! 必须实现 canister_info : () -> (CanisterInfoResult) 接口
pub async fn call_canister_info(
    canister_id: CanisterId,
    num_requested_changes: Option<u64>,
) -> super::types::CanisterCallResult<CanisterInfoResult> {
    let call_result = ic_cdk::call::Call::unbounded_wait(canister_id, "canister_info")
        .with_arg(num_requested_changes)
        .await;
    super::fetch_and_wrap_call_result(canister_id, "canister_info", call_result)
}
