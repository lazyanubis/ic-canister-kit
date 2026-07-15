use candid::CandidType;
pub use ic_cdk_management_canister::{
    Bip341, SchnorrAlgorithm, SchnorrAux, SchnorrKeyId, SchnorrPublicKeyArgs, SchnorrPublicKeyResult,
    SignWithSchnorrArgs, SignWithSchnorrResult,
};
use serde::{Deserialize, Serialize};

use crate::{identity::CanisterId, types::CanisterCallError};

/// Schnorr 私钥派生路径
pub type SchnorrDerivationPath = Vec<Vec<u8>>;

/// 罐子管理的 Schnorr 私钥路径，确定使用哪一个私钥
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct SchnorrIdentity {
    /// 阈值 Schnorr 密钥标识，包含算法和密钥名称
    pub key_id: SchnorrKeyId,

    /// 派生路径
    pub derivation_path: SchnorrDerivationPath,
}

/// 查询 Schnorr 公钥
/// <https://docs.internetcomputer.org/references/management-canister/#schnorr_public_key>
pub async fn schnorr_public_key(
    canister_id: Option<CanisterId>,
    identity: SchnorrIdentity,
) -> super::types::CanisterCallResult<SchnorrPublicKeyResult> {
    ic_cdk_management_canister::schnorr_public_key(&SchnorrPublicKeyArgs {
        canister_id,
        derivation_path: identity.derivation_path,
        key_id: identity.key_id,
    })
    .await
    .map_err(|err| CanisterCallError {
        canister_id: crate::identity::CanisterId::management_canister(),
        method: "ic#schnorr_public_key".to_string(),
        message: err.to_string(),
    })
}

/// 使用 Schnorr 对原始消息签名
/// <https://docs.internetcomputer.org/references/management-canister/#sign_with_schnorr>
pub async fn sign_with_schnorr(
    identity: SchnorrIdentity,
    message: Vec<u8>,
    aux: Option<SchnorrAux>,
) -> super::types::CanisterCallResult<SignWithSchnorrResult> {
    ic_cdk_management_canister::sign_with_schnorr(&SignWithSchnorrArgs {
        message,
        derivation_path: identity.derivation_path,
        key_id: identity.key_id,
        aux,
    })
    .await
    .map_err(|err| CanisterCallError {
        canister_id: crate::identity::CanisterId::management_canister(),
        method: "ic#sign_with_schnorr".to_string(),
        message: err.to_string(),
    })
}
