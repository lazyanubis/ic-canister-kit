use ic_cdk::api::management_canister::main::{CanisterIdRecord, CanisterInfoRequest};

use crate::identity::CanisterId;

use super::{unwrap_call_result, CallError};

/// 和 罐子 的 状态 相关

/*

引入包后, 直接使用如下方法即可增加查询罐子状态的接口

#[ic_cdk::update(name = "canister_status")]
#[candid::candid_method(update, rename = "canister_status")]
async fn canister_status() -> ic_canister_kit::canister::status::CanisterStatusResult {
    ic_canister_kit::canister::status::canister_status(ic_canister_kit::identity::self_canister_id()).await
}

*/

// 罐子状态结果
pub type CanisterStatusResult = ic_cdk::api::management_canister::main::CanisterStatusResponse;

// 罐子信息结果
pub type CanisterInfoResult = ic_cdk::api::management_canister::main::CanisterInfoResponse;

// 查询罐子状态
pub async fn canister_status(canister_id: CanisterId) -> CanisterStatusResult {
    let response =
        ic_cdk::api::management_canister::main::canister_status(CanisterIdRecord { canister_id })
            .await
            .unwrap()
            .0;

    response
}

// 查询罐子信息
pub async fn canister_info(
    canister_id: CanisterId,
    num_requested_changes: Option<u64>,
) -> CanisterInfoResult {
    let response = ic_cdk::api::management_canister::main::canister_info(CanisterInfoRequest {
        canister_id,
        num_requested_changes,
    })
    .await
    .unwrap()
    .0;

    response
}

// 启动罐子
pub async fn start_canister(canister_id: CanisterId) -> Result<(), String> {
    let call_result =
        ic_cdk::api::management_canister::main::start_canister(CanisterIdRecord { canister_id })
            .await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} start_canister failed: {:?} {}",
            canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

// 停止罐子
pub async fn stop_canister(canister_id: CanisterId) -> Result<(), String> {
    let call_result =
        ic_cdk::api::management_canister::main::stop_canister(CanisterIdRecord { canister_id })
            .await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} stop_canister failed: {:?} {}",
            canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

// 删除罐子
pub async fn delete_canister(canister_id: CanisterId) -> Result<(), String> {
    let call_result =
        ic_cdk::api::management_canister::main::delete_canister(CanisterIdRecord { canister_id })
            .await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} delete_canister failed: {:?} {}",
            canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

// 查询罐子状态
pub async fn call_canister_status(canister_id: &CanisterId) -> CanisterStatusResult {
    let call_result: Result<(CanisterStatusResult,), CallError> =
        ic_cdk::call(canister_id.clone(), "canister_status", ()).await;
    unwrap_call_result(canister_id, "canister_status", call_result)
}
