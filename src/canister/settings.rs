use ic_cdk::api::management_canister::main::{CanisterSettings, UpdateSettingsArgument};

use crate::identity::CanisterId;

/// 更新罐子设置

// 更新罐子设置
pub async fn update_settings(
    canister_id: CanisterId,
    settings: CanisterSettings,
) -> Result<(), String> {
    let call_result =
        ic_cdk::api::management_canister::main::update_settings(UpdateSettingsArgument {
            canister_id,
            settings: settings.clone(),
        })
        .await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} update_settings: {:?} failed: {:?} {}",
            canister_id.to_text(),
            settings,
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}
