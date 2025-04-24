//! 罐子代码

use crate::identity::CanisterId;

/// 部署罐子代码
pub type CanisterCodeWasm = Vec<u8>;
/// 部署罐子代码初始化参数
pub type CanisterInitArg = Vec<u8>;

// ========================= 安装代码 =========================

/// 安装罐子代码
/// ! Only the controllers of the canister
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-install_code
pub async fn install_code(
    canister_id: CanisterId,
    wasm_module: CanisterCodeWasm,
    arg: CanisterInitArg,
) -> super::types::CanisterCallResult<()> {
    let call_result =
        ic_cdk::management_canister::install_code(&ic_cdk::management_canister::InstallCodeArgs {
            mode: ic_cdk::management_canister::CanisterInstallMode::Install,
            canister_id,
            wasm_module,
            arg,
        })
        .await;
    super::wrap_call_result(canister_id, "ic#install_code#install", call_result)
}

// ========================= 升级代码 =========================

/// 升级代码
/// ! Only the controllers of the canister
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-install_code
pub async fn upgrade_code(
    canister_id: CanisterId,
    wasm_module: CanisterCodeWasm,
    arg: Option<CanisterInitArg>,
    pre_upgrade: Option<ic_cdk::management_canister::UpgradeFlags>,
) -> super::types::CanisterCallResult<()> {
    let call_result =
        ic_cdk::management_canister::install_code(&ic_cdk::management_canister::InstallCodeArgs {
            mode: ic_cdk::management_canister::CanisterInstallMode::Upgrade(pre_upgrade),
            canister_id,
            wasm_module,
            arg: arg.unwrap_or_default(),
        })
        .await;
    super::wrap_call_result(canister_id, "ic#install_code#upgrade", call_result)
}

// ========================= 重新安装代码 =========================

/// 重新安装代码
/// ! Only the controllers of the canister
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-install_code
pub async fn reinstall_code(
    canister_id: CanisterId,
    wasm_module: CanisterCodeWasm,
    arg: CanisterInitArg,
) -> super::types::CanisterCallResult<()> {
    let call_result =
        ic_cdk::management_canister::install_code(&ic_cdk::management_canister::InstallCodeArgs {
            mode: ic_cdk::management_canister::CanisterInstallMode::Reinstall,
            canister_id,
            wasm_module,
            arg,
        })
        .await;
    super::wrap_call_result(canister_id, "ic#install_code#reinstall", call_result)
}

// ========================= 卸载代码 =========================

/// 卸载罐子代码
/// ! Only the controllers of the canister
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-uninstall_code
pub async fn uninstall_code(canister_id: CanisterId) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk::management_canister::uninstall_code(
        &ic_cdk::management_canister::UninstallCodeArgs { canister_id },
    )
    .await;
    super::wrap_call_result(canister_id, "ic#uninstall_code", call_result)
}
