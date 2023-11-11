use super::{call_error_to_string, CallError};
use crate::identity::CanisterId;

/// 查询罐子的 candid

pub async fn canister_did(canister_id: CanisterId) -> Result<String, String> {
    let result: Result<(String,), CallError> = super::call::call_canister::<(), (String,)>(
        canister_id,
        "__get_candid_interface_tmp_hack",
        (),
    )
    .await;
    match result {
        Ok(result) => Ok(result.0),
        Err(err) => Err(call_error_to_string(&err)),
    }
}
