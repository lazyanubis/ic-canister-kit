use crate::identity::CanisterId;

use self::types::CallError;

#[cfg(feature = "canister_cycles")]
pub mod cycles;

#[cfg(feature = "canister_status")]
pub mod status;

#[cfg(feature = "canister_create")]
pub mod create;

#[cfg(feature = "canister_settings")]
pub mod settings;

#[cfg(feature = "canister_codes")]
pub mod codes;

#[cfg(feature = "canister_deploy")]
pub mod deploy;

#[cfg(feature = "canister_call")]
pub mod call;

#[cfg(feature = "canister_managed")]
pub mod managed;

pub mod types;

// ========================= 错误处理 =========================

// 解开方法调用结果
fn unwrap_call_result<R: std::fmt::Debug>(
    canister_id: &CanisterId,
    method: &str,
    call_result: Result<(R,), CallError>,
) -> R {
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        panic!(
            "canister: {} call: {} failed: {:?} {}",
            canister_id.to_text(),
            method,
            err.0,
            err.1
        );
    }

    call_result.unwrap().0
}

// 解开方法调用结果
#[allow(dead_code)]
fn unwrap_call_result_with_error<R: std::fmt::Debug>(
    canister_id: &CanisterId,
    method: &str,
    call_result: Result<(R,), CallError>,
) -> Result<R, String> {
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} call: {} failed: {:?} {}",
            canister_id.to_text(),
            method,
            err.0,
            err.1
        ));
    }

    Result::Ok(call_result.unwrap().0)
}

// 错误信息
pub fn call_error_to_string(call_error: &CallError) -> String {
    format!("error code: {:?} message: {}", call_error.0, call_error.1)
}
