/// cycles
pub mod cycles;

/// status
pub mod status;

/// life
pub mod life;

/// codes
pub mod codes;

/// deploy
pub mod deploy;

/// settings
pub mod settings;

/// call
pub mod call;

/// common
pub mod common;

/// 类型
pub mod types;

// ========================= 基本方法 =========================

use std::fmt::Display;

use candid::CandidType;
use ic_cdk::call::Response;
pub use ic_cdk::stable::WASM_PAGE_SIZE_IN_BYTES as WASM_PAGE_SIZE;
use serde::Deserialize;

/// 罐子自身的 cycles
#[inline]
pub fn self_canister_cycles() -> u128 {
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::canister_cycle_balance()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

/// 罐子自己的稳定内存使用量
#[inline]
pub fn self_canister_stable_memory_size() -> u128 {
    #[cfg(target_arch = "wasm32")]
    {
        (ic_cdk::api::stable_size() as u128) * WASM_PAGE_SIZE as u128
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

/// 罐子自己的堆内存使用量
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

/// 罐子自身的总内存
#[inline]
pub fn self_canister_current_memory_size() -> u128 {
    self_canister_stable_memory_size() + self_canister_heap_memory_size()
}

// ========================= 错误处理 =========================

// use self::types::CanisterCallError;

// 调用结果
#[allow(unused)]
#[inline]
pub(crate) fn fetch_and_wrap_call_result<R: CandidType + for<'de> Deserialize<'de>, E: Display>(
    canister_id: crate::identity::CanisterId,
    method: &str,
    call_result: Result<Response, E>,
) -> types::CanisterCallResult<R> {
    call_result
        .map_err(|err| crate::canister::types::CanisterCallError::new(canister_id, method, err))?
        .candid()
        .map_err(|err| crate::canister::types::CanisterCallError::new(canister_id, method, err))
}

#[allow(unused)]
#[inline]
fn wrap_call_result<E: Display>(
    canister_id: crate::identity::CanisterId,
    method: &str,
    call_result: Result<(), E>,
) -> types::CanisterCallResult<()> {
    call_result.map_err(|err| crate::canister::types::CanisterCallError::new(canister_id, method, err))
}
