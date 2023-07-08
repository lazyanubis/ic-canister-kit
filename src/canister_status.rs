use super::identity::CanisterId;

// 罐子配置信息
pub type DefiniteCAnisterSettings =
    ic_cdk::api::management_canister::main::DefiniteCanisterSettings;
// 罐子状态类型
pub type CanisterStatus = ic_cdk::api::management_canister::main::CanisterStatusType;

// 罐子状态请求参数
pub type CanisterStatusArg = ic_cdk::api::management_canister::main::CanisterIdRecord;
// 罐子状态结果
pub type CanisterStatusResult = ic_cdk::api::management_canister::main::CanisterStatusResponse;

// 罐子状态请求参数
pub type CanisterInfoArg = ic_cdk::api::management_canister::main::CanisterInfoRequest;
// 罐子信息结果
pub type CanisterInfoResult = ic_cdk::api::management_canister::main::CanisterInfoResponse;

pub fn new_canister_status_arg(canister_id: CanisterId) -> CanisterStatusArg {
    CanisterStatusArg { canister_id }
}

pub async fn canister_status(arg: CanisterStatusArg) -> CanisterStatusResult {
    let response = ic_cdk::api::management_canister::main::canister_status(arg)
        .await
        .unwrap()
        .0;

    response
}

pub async fn canister_info(arg: CanisterInfoArg) -> CanisterInfoResult {
    let response = ic_cdk::api::management_canister::main::canister_info(arg)
        .await
        .unwrap()
        .0;

    response
}

// #[ic_cdk::update(name = "canister_status")]
// #[candid::candid_method(update, rename = "canister_status")]
// async fn canister_status() -> CanisterStatusResult {
//     let arg = ic_canister_kit::canister_status::new_canister_status_arg(ic_cdk::id());
//     ic_canister_kit::canister_status::canister_status(arg).await
// }
