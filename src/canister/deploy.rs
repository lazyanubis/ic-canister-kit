//! 和 罐子 的 部署 相关

/// 部署罐子
pub async fn deploy_canister(
    settings: Option<ic_cdk::management_canister::CanisterSettings>,
    initial_cycles: u128,
    wasm_module: super::codes::CanisterCodeWasm,
    arg: super::codes::CanisterInitArg,
) -> super::types::CanisterCallResult<crate::identity::CanisterId> {
    // 1. 创建一个新的罐子
    let canister_id = crate::canister::life::create_canister(settings, initial_cycles)
        .await
        .map_err(|err| crate::canister::types::CanisterCallError::from("", err))?;
    ic_cdk::println!("new canister id: {:?}", canister_id.to_text());

    // 2. 安装代码
    crate::canister::codes::install_code(canister_id, wasm_module, arg).await?;

    // 3. 启动罐子
    crate::canister::life::start_canister(canister_id).await?;

    Ok(canister_id)
}
