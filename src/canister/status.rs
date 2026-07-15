//! 和 罐子 的 状态 相关

use super::types::{
    CanisterInfoArgs, CanisterInfoResult, CanisterMetricsArgs, CanisterMetricsResult, CanisterStatusArgs,
    CanisterStatusResult,
};
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
/// <https://docs.internetcomputer.org/references/management-canister/#canister_status>
pub async fn canister_status(canister_id: CanisterId) -> super::types::CanisterCallResult<CanisterStatusResult> {
    let call_result = ic_cdk_management_canister::canister_status(&CanisterStatusArgs { canister_id }).await;
    call_result.map_err(|err| crate::canister::types::CanisterCallError::new(canister_id, "ic#canister_status", err))
}

/// 查询罐子信息
/// ! 罐子可以调用，用户身份不可以调用
/// <https://docs.internetcomputer.org/references/management-canister/#canister_info>
pub async fn canister_info(
    canister_id: CanisterId,
    num_requested_changes: Option<u64>,
) -> super::types::CanisterCallResult<CanisterInfoResult> {
    let call_result = ic_cdk_management_canister::canister_info(&CanisterInfoArgs {
        canister_id,
        num_requested_changes,
    })
    .await;
    call_result.map_err(|err| crate::canister::types::CanisterCallError::new(canister_id, "ic#canister_info", err))
}

/// 查询 Canister 自创建以来按用途累计消耗的 cycles；旧 Canister 从该指标启用时开始累计。
///
/// 这些值是单调递增的累计计数器，不是当前 cycles 余额。调用者必须是目标 Canister 的 controller
/// 或子网管理员。
/// <https://docs.internetcomputer.org/references/management-canister/#canister_metrics>
pub async fn canister_metrics(canister_id: CanisterId) -> super::types::CanisterCallResult<CanisterMetricsResult> {
    let call_result = ic_cdk::call::Call::bounded_wait(CanisterId::management_canister(), "canister_metrics")
        .with_arg(&CanisterMetricsArgs { canister_id })
        .await;
    super::fetch_and_wrap_call_result(canister_id, "ic#canister_metrics", call_result)
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
