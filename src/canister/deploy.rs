//! 和 罐子 的 部署 相关

use super::types::CanisterSettings;

async fn cleanup_failed_deployment(canister_id: crate::identity::CanisterId) -> super::types::CanisterCallResult<()> {
    crate::canister::life::stop_canister(canister_id).await?;
    crate::canister::life::delete_canister(canister_id).await
}

fn attach_cleanup_result(
    mut deploy_error: super::types::CanisterCallError,
    cleanup_result: super::types::CanisterCallResult<()>,
) -> super::types::CanisterCallError {
    deploy_error.message = match cleanup_result {
        Ok(()) => format!("{}; the newly created canister was cleaned up", deploy_error.message),
        Err(cleanup_error) => format!(
            "{}; automatic cleanup failed, manually recover canister {}: {}",
            deploy_error.message,
            deploy_error.canister_id.to_text(),
            cleanup_error
        ),
    };
    deploy_error
}

/// 部署罐子
pub async fn deploy_canister(
    settings: Option<CanisterSettings>,
    initial_cycles: u128,
    wasm_module: super::codes::CanisterCodeWasm,
    arg: super::codes::CanisterInitArg,
) -> super::types::CanisterCallResult<crate::identity::CanisterId> {
    // 1. 创建一个新的罐子
    let canister_id = crate::canister::life::create_canister(settings, initial_cycles)
        .await
        .map_err(|err| crate::canister::types::CanisterCallError::from("ic#create_canister", err))?;
    ic_cdk::println!("new canister id: {:?}", canister_id.to_text());

    // 2. 安装代码
    if let Err(install_error) = crate::canister::codes::install_code(canister_id, wasm_module, arg).await {
        let cleanup_result = cleanup_failed_deployment(canister_id).await;
        return Err(attach_cleanup_result(install_error, cleanup_result));
    }

    Ok(canister_id)
}

#[cfg(test)]
mod tests {
    use candid::Principal;

    use super::attach_cleanup_result;
    use crate::canister::types::CanisterCallError;

    fn error(method: &str, message: &str) -> CanisterCallError {
        CanisterCallError::new(Principal::from_slice(&[1]), method, message)
    }

    #[test]
    fn reports_successful_compensation() {
        let error = attach_cleanup_result(error("ic#install_code#install", "install failed"), Ok(()));
        assert!(error.message.contains("install failed"));
        assert!(error.message.contains("was cleaned up"));
    }

    #[test]
    fn preserves_canister_id_when_compensation_fails() {
        let error = attach_cleanup_result(
            error("ic#install_code#install", "install failed"),
            Err(error("ic#stop_canister", "stop failed")),
        );
        assert!(error.message.contains("manually recover canister"));
        assert!(error.message.contains("stop failed"));
    }
}
