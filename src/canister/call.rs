/// 调用罐子
pub async fn call_canister<
    T: candid::utils::ArgumentEncoder + Send,
    R: for<'a> candid::utils::ArgumentDecoder<'a>,
>(
    canister_id: crate::identity::CanisterId,
    method: &str,
    args: T,
) -> super::types::CanisterCallResult<R> {
    ic_cdk::println!("call canister: {} -> {}", canister_id.to_text(), method);
    let call_result = ic_cdk::call::<T, R>(canister_id, method, args).await;
    call_result.map_err(
        |(rejection_code, message)| crate::canister::types::CanisterCallError {
            canister_id,
            method: method.to_string(),
            rejection_code,
            message,
        },
    )
}
