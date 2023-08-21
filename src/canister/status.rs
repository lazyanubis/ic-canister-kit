use ic_cdk::api::management_canister::main::{CanisterIdRecord, CanisterInfoRequest};

use crate::identity::CanisterId;

/// 和 罐子 的 状态 相关

/*

引入包后, 直接使用如下方法即可增加查询罐子状态的接口

#[ic_cdk::update(name = "canister_status")]
#[candid::candid_method(update, rename = "canister_status")]
async fn canister_status() -> CanisterStatusResult {
    ic_canister_kit::canister::status::canister_status(ic_canister_kit::identity::self_canister_id()).await
}

*/

// 罐子状态结果
pub type CanisterStatusResult = ic_cdk::api::management_canister::main::CanisterStatusResponse;

// 罐子信息结果
pub type CanisterInfoResult = ic_cdk::api::management_canister::main::CanisterInfoResponse;

pub async fn canister_status(canister_id: CanisterId) -> CanisterStatusResult {
    let response =
        ic_cdk::api::management_canister::main::canister_status(CanisterIdRecord { canister_id })
            .await
            .unwrap()
            .0;

    response
}

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
