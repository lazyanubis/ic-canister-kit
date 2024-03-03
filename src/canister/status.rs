use crate::identity::CanisterId;

/// 和 罐子 的 状态 相关

/*

引入包后, 直接使用如下方法即可增加查询罐子状态的接口

#[ic_cdk::update]
async fn canister_status() -> ic_cdk::api::management_canister::main::CanisterStatusResponse {
    #[allow(clippy::unwrap_used)]
    ic_canister_kit::canister::status::canister_status(ic_canister_kit::identity::self_canister_id()).await.unwrap()
}

#[ic_cdk::update]
async fn canister_info(num_requested_changes: Option<u64>) -> ic_cdk::api::management_canister::main::CanisterInfoResponse {
    #[allow(clippy::unwrap_used)]
    ic_canister_kit::canister::status::canister_info(ic_canister_kit::identity::self_canister_id(), num_requested_changes).await.unwrap()
}

*/

// ========================= 罐子状态信息 =========================

// 查询罐子状态
// ! Only the controllers of the canister
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

// 查询罐子信息
// ! 罐子可以调用，用户身份不可以调用
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

// ========================= 改变罐子状态 =========================

// 启动罐子
// ! Only the controllers of the canister
pub async fn start_canister(canister_id: CanisterId) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk::api::management_canister::main::start_canister(
        ic_cdk::api::management_canister::main::CanisterIdRecord { canister_id },
    )
    .await;
    super::wrap_call_result(canister_id, "ic#start_canister", call_result)
}

// 停止罐子
// ! Only the controllers of the canister
pub async fn stop_canister(canister_id: CanisterId) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk::api::management_canister::main::stop_canister(
        ic_cdk::api::management_canister::main::CanisterIdRecord { canister_id },
    )
    .await;
    super::wrap_call_result(canister_id, "ic#stop_canister", call_result)
}

// 删除罐子
// ! Only the controllers of the canister
// ! already be stopped
pub async fn delete_canister(canister_id: CanisterId) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk::api::management_canister::main::delete_canister(
        ic_cdk::api::management_canister::main::CanisterIdRecord { canister_id },
    )
    .await;
    super::wrap_call_result(canister_id, "ic#delete_canister", call_result)
}

// ========================= 自定义接口调用 =========================

// 查询罐子状态
// ! 必须实现 canister_status : () -> (CanisterStatusResponse) 接口
pub async fn call_canister_status(
    canister_id: CanisterId,
) -> ic_cdk::api::management_canister::main::CanisterStatusResponse {
    let call_result = ic_cdk::api::call::call::<
        (),
        (ic_cdk::api::management_canister::main::CanisterStatusResponse,),
    >(canister_id, "canister_status", ())
    .await;
    #[allow(clippy::unwrap_used)]
    super::fetch_and_wrap_call_result(canister_id, "canister_status", call_result).unwrap()
}

// 查询罐子信息
// ! 必须实现 canister_info : () -> (CanisterInfoResponse) 接口
pub async fn call_canister_info(
    canister_id: CanisterId,
    num_requested_changes: Option<u64>,
) -> ic_cdk::api::management_canister::main::CanisterInfoResponse {
    let call_result = ic_cdk::api::call::call::<
        (Option<u64>,),
        (ic_cdk::api::management_canister::main::CanisterInfoResponse,),
    >(canister_id, "canister_info", (num_requested_changes,))
    .await;
    #[allow(clippy::unwrap_used)]
    super::fetch_and_wrap_call_result(canister_id, "canister_info", call_result).unwrap()
}
