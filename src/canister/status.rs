//! 和 罐子 的 状态 相关

use crate::identity::CanisterId;

/*

引入包后, 直接使用如下方法即可增加查询罐子状态的接口

#[ic_cdk::update]
async fn canister_status() -> ic_cdk::api::management_canister::main::CanisterStatusResponse {
    use ic_canister_kit::{canister::status::canister_status, identity::self_canister_id};
    let response = canister_status(self_canister_id()).await;
    ic_canister_kit::common::trap::trap(response.map_err(|err| err.to_string()))
}

#[ic_cdk::update]
async fn canister_info(num_requested_changes: Option<u64>) -> ic_cdk::api::management_canister::main::CanisterInfoResponse {
    use ic_canister_kit::{canister::status::canister_info, identity::self_canister_id};
    let response = canister_info(self_canister_id(), num_requested_changes).await;
    ic_canister_kit::common::trap::trap(response.map_err(|err| err.to_string()))
}

*/
// ========================= 罐子状态信息 =========================

/// 查询罐子状态
/// ! Only the controllers of the canister
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-canister_status
pub async fn canister_status(
    canister_id: CanisterId,
) -> super::types::CanisterCallResult<ic_cdk::api::management_canister::main::CanisterStatusResponse>
{
    let call_result = ic_cdk::api::management_canister::main::canister_status(
        ic_cdk::api::management_canister::main::CanisterIdRecord { canister_id },
    )
    .await;
    super::fetch_and_wrap_call_result(canister_id, "ic#canister_status", call_result)
}

/// 查询罐子信息
/// ! 罐子可以调用，用户身份不可以调用
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-canister-info
pub async fn canister_info(
    canister_id: CanisterId,
    num_requested_changes: Option<u64>,
) -> super::types::CanisterCallResult<ic_cdk::api::management_canister::main::CanisterInfoResponse>
{
    let call_result = ic_cdk::api::management_canister::main::canister_info(
        ic_cdk::api::management_canister::main::CanisterInfoRequest {
            canister_id,
            num_requested_changes,
        },
    )
    .await;
    super::fetch_and_wrap_call_result(canister_id, "ic#canister_info", call_result)
}

// ========================= 自定义接口调用 =========================

/// 查询罐子状态
/// ! 必须实现 canister_status : () -> (CanisterStatusResponse) 接口
pub async fn call_canister_status(
    canister_id: CanisterId,
) -> super::types::CanisterCallResult<ic_cdk::api::management_canister::main::CanisterStatusResponse>
{
    let call_result = ic_cdk::api::call::call::<
        (),
        (ic_cdk::api::management_canister::main::CanisterStatusResponse,),
    >(canister_id, "canister_status", ())
    .await;
    super::fetch_and_wrap_call_result(canister_id, "canister_status", call_result)
}

/// 查询罐子信息
/// ! 必须实现 canister_info : () -> (CanisterInfoResponse) 接口
pub async fn call_canister_info(
    canister_id: CanisterId,
    num_requested_changes: Option<u64>,
) -> super::types::CanisterCallResult<ic_cdk::api::management_canister::main::CanisterInfoResponse>
{
    let call_result = ic_cdk::api::call::call::<
        (Option<u64>,),
        (ic_cdk::api::management_canister::main::CanisterInfoResponse,),
    >(canister_id, "canister_info", (num_requested_changes,))
    .await;
    super::fetch_and_wrap_call_result(canister_id, "canister_info", call_result)
}
