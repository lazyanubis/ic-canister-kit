/// 查询罐子的 candid
pub async fn canister_did(
    canister_id: crate::identity::CanisterId,
) -> super::types::CanisterCallResult<String> {
    super::call::call_canister::<_, String>(canister_id, "__get_candid_interface_tmp_hack", ())
        .await
}
