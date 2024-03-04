/// 更新罐子设置

// 更新罐子设置
pub async fn update_settings(
    canister_id: crate::identity::CanisterId,
    settings: ic_cdk::api::management_canister::main::CanisterSettings,
) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk::api::management_canister::main::update_settings(
        ic_cdk::api::management_canister::main::UpdateSettingsArgument {
            canister_id,
            settings,
        },
    )
    .await;
    super::wrap_call_result(canister_id, "ic#update_settings", call_result)
}
