use candid::CandidType;
use ic_cdk::management_canister::{EcdsaPublicKeyArgs, SignWithEcdsaArgs};
use serde::{Deserialize, Serialize};

pub use ic_cdk::management_canister::{
    EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyResult, SignWithEcdsaResult,
};

use crate::{identity::CanisterId, types::CanisterCallError};

/// 私钥派生路径
/// 不知道有没有长度要求的
pub type EcdsaDerivationPath = Vec<Vec<u8>>;

// #[derive(Debug)]
// pub struct EcdsaDerivationPath(Vec<Vec<u8>>);

// #[derive(Debug)]
// pub enum EcdsaDerivationPathError {
//     WrongLength,     // len <= 256
//     WrongPathLength, // len <= 256
// }
// impl std::fmt::Display for EcdsaDerivationPathError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             EcdsaDerivationPathError::WrongLength => write!(f, "wrong length"),
//             EcdsaDerivationPathError::WrongPathLength => write!(f, "wrong path length"),
//         }
//     }
// }
// impl std::error::Error for EcdsaDerivationPathError {}

// impl TryFrom<Vec<Vec<u8>>> for EcdsaDerivationPath {
//     type Error = EcdsaDerivationPathError;

//     fn try_from(value: Vec<Vec<u8>>) -> Result<Self, Self::Error> {
//         if 256 < value.len() {
//             return Err(EcdsaDerivationPathError::WrongLength);
//         }
//         for path in &value {
//             if 256 < path.len() {
//                 return Err(EcdsaDerivationPathError::WrongPathLength);
//             }
//         }
//         Ok(EcdsaDerivationPath(value))
//     }
// }

/// 罐子管理的私钥路径，确定使用哪一个私钥
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct EcdsaIdentity {
    /// 加密曲线
    pub key_id: EcdsaKeyId,

    /// 派生路径
    pub derivation_path: EcdsaDerivationPath,
}

/// 消息 hash 对象，必须 32 长度
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct MessageHash(Vec<u8>);

/// 消息 hash 错误
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct MessageHashError;
impl std::fmt::Display for MessageHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hash of the message with length of 32 bytes.")
    }
}
impl std::error::Error for MessageHashError {}
impl TryFrom<Vec<u8>> for MessageHash {
    type Error = MessageHashError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 32 {
            return Err(MessageHashError);
        }
        Ok(MessageHash(value))
    }
}

/// 查询公钥
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key
pub async fn ecdsa_public_key(
    canister_id: Option<CanisterId>, // 不写则是自身 id
    identity: EcdsaIdentity,
) -> super::types::CanisterCallResult<EcdsaPublicKeyResult> {
    ic_cdk::management_canister::ecdsa_public_key(&EcdsaPublicKeyArgs {
        canister_id,
        // derivation_path: identity.derivation_path.0,
        derivation_path: identity.derivation_path,
        key_id: identity.key_id,
    })
    .await
    .map_err(|err| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#ecdsa_public_key".to_string(),
        message: err.to_string(),
    })
}

/// 进行签名
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_ecdsa
pub async fn ecdsa_sign(
    identity: EcdsaIdentity,
    message_hash: MessageHash,
) -> super::types::CanisterCallResult<SignWithEcdsaResult> {
    ic_cdk::management_canister::sign_with_ecdsa(&SignWithEcdsaArgs {
        message_hash: message_hash.0,
        derivation_path: identity.derivation_path,
        key_id: identity.key_id,
    })
    .await
    .map_err(|err| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#sign_with_ecdsa".to_string(),
        message: err.to_string(),
    })
}
