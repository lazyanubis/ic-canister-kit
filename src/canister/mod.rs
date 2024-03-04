pub mod cycles;

pub mod status;

pub mod life;

pub mod codes;

pub mod deploy;

pub mod settings;

// #[cfg(feature = "canister_call")]
// pub mod call;

// #[cfg(feature = "canister_candid")]
// pub mod candid;

// #[cfg(feature = "canister_managed")]
// pub mod managed;

pub mod types;

// ========================= 基本方法 =========================

pub use ic_cdk::api::stable::WASM_PAGE_SIZE_IN_BYTES as WASM_PAGE_SIZE;

#[inline]
pub fn self_canister_cycles() -> u128 {
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::canister_balance128()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

#[inline]
pub fn self_canister_stable_memory_size() -> u128 {
    #[cfg(target_arch = "wasm32")]
    {
        (ic_cdk::api::stable::stable64_size() as u128) * WASM_PAGE_SIZE as u128
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

#[inline]
pub fn self_canister_heap_memory_size() -> u128 {
    #[cfg(target_arch = "wasm32")]
    {
        (core::arch::wasm32::memory_size(0) as u128) * WASM_PAGE_SIZE as u128
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

#[inline]
pub fn self_canister_current_memory_size() -> u128 {
    self_canister_stable_memory_size() + self_canister_heap_memory_size()
}

// ========================= 错误处理 =========================

// use self::types::CanisterCallError;

// 调用结果
#[allow(unused)]
#[inline]
fn fetch_and_wrap_call_result<R>(
    canister_id: crate::identity::CanisterId,
    method: &str,
    call_result: Result<(R,), (ic_cdk::api::call::RejectionCode, String)>,
) -> types::CanisterCallResult<R> {
    call_result
        .map(|(r,)| r)
        .map_err(|(rejection_code, message)| types::CanisterCallError {
            canister_id,
            method: method.to_string(),
            rejection_code,
            message,
        })
}

#[allow(unused)]
#[inline]
fn wrap_call_result(
    canister_id: crate::identity::CanisterId,
    method: &str,
    call_result: Result<(), (ic_cdk::api::call::RejectionCode, String)>,
) -> types::CanisterCallResult<()> {
    call_result.map_err(|(rejection_code, message)| types::CanisterCallError {
        canister_id,
        method: method.to_string(),
        rejection_code,
        message,
    })
}
