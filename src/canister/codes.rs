//! 罐子代码

use super::types::{
    CanisterInstallMode, ChunkHash, InstallChunkedCodeArgs, InstallCodeArgs, UninstallCodeArgs, UpgradeFlags,
};
use crate::identity::CanisterId;

/// 部署罐子代码
pub type CanisterCodeWasm = Vec<u8>;
/// 部署罐子代码初始化参数
pub type CanisterInitArg = Vec<u8>;
/// Canister Wasm 模块的 SHA-256 hash。
pub type CanisterCodeHash = Vec<u8>;

// ========================= 安装代码 =========================

/// 安装罐子代码
/// ! Only the controllers of the canister
/// <https://docs.internetcomputer.org/references/management-canister/#install_code>
pub async fn install_code(
    canister_id: CanisterId,
    wasm_module: CanisterCodeWasm,
    arg: CanisterInitArg,
) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk_management_canister::install_code(&InstallCodeArgs {
        mode: CanisterInstallMode::Install,
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
/// <https://docs.internetcomputer.org/references/management-canister/#install_code>
pub async fn upgrade_code(
    canister_id: CanisterId,
    wasm_module: CanisterCodeWasm,
    arg: Option<CanisterInitArg>,
    pre_upgrade: Option<UpgradeFlags>,
) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk_management_canister::install_code(&InstallCodeArgs {
        mode: CanisterInstallMode::Upgrade(pre_upgrade),
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
/// <https://docs.internetcomputer.org/references/management-canister/#install_code>
pub async fn reinstall_code(
    canister_id: CanisterId,
    wasm_module: CanisterCodeWasm,
    arg: CanisterInitArg,
) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk_management_canister::install_code(&InstallCodeArgs {
        mode: CanisterInstallMode::Reinstall,
        canister_id,
        wasm_module,
        arg,
    })
    .await;
    super::wrap_call_result(canister_id, "ic#install_code#reinstall", call_result)
}

// ========================= 安装分块代码 =========================

/// 使用已上传到 chunk store 的分块安装、重装或升级 Canister 代码。
///
/// `chunk_hashes_list` 必须按 Wasm 的拼接顺序传入，`wasm_module_hash` 必须是拼接后完整 Wasm 的 SHA-256 hash。
/// `store_canister` 为 `None` 时使用 `target_canister` 的 chunk store。目标 Canister 与存储 Canister 必须位于同一子网。
///
/// 调用者必须有权为目标 Canister 安装代码；同时必须是存储 Canister 的 controller，或者是存储 Canister 自身。
/// <https://docs.internetcomputer.org/references/management-canister/#install_chunked_code>
pub async fn install_chunked_code(
    target_canister: CanisterId,
    mode: CanisterInstallMode,
    store_canister: Option<CanisterId>,
    chunk_hashes_list: Vec<ChunkHash>,
    wasm_module_hash: CanisterCodeHash,
    arg: CanisterInitArg,
) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk_management_canister::install_chunked_code(&InstallChunkedCodeArgs {
        mode,
        target_canister,
        store_canister,
        chunk_hashes_list,
        wasm_module_hash,
        arg,
    })
    .await;
    super::wrap_call_result(target_canister, "ic#install_chunked_code", call_result)
}

// ========================= 卸载代码 =========================

/// 卸载罐子代码
/// ! Only the controllers of the canister
/// <https://docs.internetcomputer.org/references/management-canister/#uninstall_code>
pub async fn uninstall_code(canister_id: CanisterId) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk_management_canister::uninstall_code(&UninstallCodeArgs { canister_id }).await;
    super::wrap_call_result(canister_id, "ic#uninstall_code", call_result)
}
