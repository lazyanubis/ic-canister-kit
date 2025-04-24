use candid::CandidType;
use serde::Deserialize;

/// 调用罐子
pub async fn call_canister<T: CandidType + Send, R: CandidType + for<'de> Deserialize<'de>>(
    canister_id: crate::identity::CanisterId,
    method: &str,
    args: T,
) -> super::types::CanisterCallResult<R> {
    ic_cdk::println!("call canister: {} -> {}", canister_id.to_text(), method);
    let call_result = ic_cdk::call::Call::unbounded_wait(canister_id, method)
        .with_arg(args)
        .await;
    call_result
        .map_err(|err| crate::canister::types::CanisterCallError {
            canister_id,
            method: method.to_string(),
            message: err.to_string(),
        })?
        .candid()
        .map_err(|err| crate::canister::types::CanisterCallError {
            canister_id,
            method: method.to_string(),
            message: err.to_string(),
        })
}
