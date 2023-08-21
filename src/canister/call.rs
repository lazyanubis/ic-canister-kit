use super::CallError;
use crate::identity::CanisterId;

/// 调用罐子

pub async fn call_canister<
    T: candid::utils::ArgumentEncoder,
    R: for<'a> candid::utils::ArgumentDecoder<'a>,
>(
    canister_id: CanisterId,
    method: &str,
    args: T,
) -> Result<R, CallError> {
    ic_cdk::call(canister_id.clone(), &method, args).await
}
