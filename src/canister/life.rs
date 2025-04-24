use crate::identity::CanisterId;

// ========================= 创建罐子 =========================

/// 创建罐子
/// ! Only the controllers of the canister
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-create_canister
pub async fn create_canister(
    settings: Option<ic_cdk::management_canister::CanisterSettings>,
    cycles: u128,
) -> Result<CanisterId, ic_cdk::call::Error> {
    let call_result = ic_cdk::management_canister::create_canister_with_extra_cycles(
        &ic_cdk::management_canister::CreateCanisterArgs {
            settings: settings.clone(),
        },
        cycles,
    )
    .await;
    call_result.map(|r| r.canister_id)
}

// ========================= 改变罐子状态 =========================

/// 启动罐子
/// ! Only the controllers of the canister
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-start_canister
pub async fn start_canister(canister_id: CanisterId) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk::management_canister::start_canister(
        &ic_cdk::management_canister::StartCanisterArgs { canister_id },
    )
    .await;
    super::wrap_call_result(canister_id, "ic#start_canister", call_result)
}

/// 停止罐子
/// ! Only the controllers of the canister
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-stop_canister
pub async fn stop_canister(canister_id: CanisterId) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk::management_canister::stop_canister(
        &ic_cdk::management_canister::StopCanisterArgs { canister_id },
    )
    .await;
    super::wrap_call_result(canister_id, "ic#stop_canister", call_result)
}

// ========================= 删除罐子 =========================

/// 删除罐子
/// ! Only the controllers of the canister
/// ! already be stopped
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-delete_canister
pub async fn delete_canister(canister_id: CanisterId) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk::management_canister::delete_canister(
        &ic_cdk::management_canister::DeleteCanisterArgs { canister_id },
    )
    .await;
    super::wrap_call_result(canister_id, "ic#delete_canister", call_result)
}
