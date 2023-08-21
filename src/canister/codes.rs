use ic_cdk::api::management_canister::main::{
    CanisterIdRecord, CanisterInstallMode, InstallCodeArgument,
};

use crate::identity::CanisterId;

/// 罐子代码

pub type CanisterCodeWasm = Vec<u8>; // 部署罐子代码
pub type CanisterInitArg = Vec<u8>; // 部署罐子代码初始化参数

// 安装罐子代码
pub async fn install_code(
    canister_id: CanisterId,
    wasm_module: CanisterCodeWasm,
    arg: CanisterInitArg,
) -> Result<(), String> {
    let call_result = ic_cdk::api::management_canister::main::install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module,
        arg,
    })
    .await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} install_code failed: {:?} {}",
            canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

// 升级代码
pub async fn upgrade_code(
    canister_id: CanisterId,
    wasm_module: CanisterCodeWasm,
    arg: Option<CanisterInitArg>,
) -> Result<(), String> {
    let call_result = ic_cdk::api::management_canister::main::install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade,
        canister_id,
        wasm_module,
        arg: arg.unwrap_or(vec![]),
    })
    .await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} upgrade_code failed: {:?} {}",
            canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

// 重新安装代码
pub async fn reinstall_code(
    canister_id: CanisterId,
    wasm_module: CanisterCodeWasm,
    arg: CanisterInitArg,
) -> Result<(), String> {
    let call_result = ic_cdk::api::management_canister::main::install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Reinstall,
        canister_id,
        wasm_module,
        arg,
    })
    .await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} reinstall_code failed: {:?} {}",
            canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

// 卸载罐子代码
pub async fn uninstall_code(canister_id: CanisterId) -> Result<(), String> {
    let call_result =
        ic_cdk::api::management_canister::main::uninstall_code(CanisterIdRecord { canister_id })
            .await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} uninstall_code failed: {:?} {}",
            canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}
