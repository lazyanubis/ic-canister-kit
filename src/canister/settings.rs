//! 更新罐子设置

/// 更新罐子设置
/// ! Only the controllers of the canister
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-update_settings
pub async fn update_settings(
    canister_id: crate::identity::CanisterId,
    settings: ic_cdk::management_canister::CanisterSettings,
) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk::management_canister::update_settings(
        &ic_cdk::management_canister::UpdateSettingsArgs {
            canister_id,
            settings,
        },
    )
    .await;
    call_result.map_err(|err| crate::canister::types::CanisterCallError {
        canister_id,
        method: "ic#update_settings".to_string(),
        message: err.to_string(),
    })
}
