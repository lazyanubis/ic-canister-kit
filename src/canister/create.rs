use ic_cdk::api::management_canister::main::{CanisterSettings, CreateCanisterArgument};

use crate::identity::CanisterId;

/// 创建新的罐子

// 创建罐子
pub async fn create_canister(
    settings: Option<CanisterSettings>,
    cycles: u128,
) -> Result<CanisterId, String> {
    let result = ic_cdk::api::management_canister::main::create_canister(
        CreateCanisterArgument {
            settings: settings.clone(),
        },
        cycles,
    )
    .await;
    if result.is_err() {
        let err = result.unwrap_err();
        return Result::Err(format!(
            "create canister {:?} {} failed: {:?} {}",
            settings, cycles, err.0, err.1
        ));
    }
    Ok(result.unwrap().0.canister_id)
}
