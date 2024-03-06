use ic_cdk::api::management_canister::ecdsa::{EcdsaPublicKeyArgument, SignWithEcdsaArgument};

pub use ic_cdk::api::management_canister::ecdsa::{
    EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyResponse, SignWithEcdsaResponse,
};

use crate::{canister::fetch_tuple0, identity::CanisterId, types::CanisterCallError};

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

pub struct EcdsaIdentity {
    pub key_id: EcdsaKeyId,
    pub derivation_path: EcdsaDerivationPath,
}

pub struct MessageHash(Vec<u8>);
#[derive(Debug)]
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

// 查询公钥
// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key
pub async fn ecdsa_public_key(
    canister_id: Option<CanisterId>, // 不写则是自身 id
    identity: EcdsaIdentity,
) -> super::types::CanisterCallResult<EcdsaPublicKeyResponse> {
    ic_cdk::api::management_canister::ecdsa::ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id,
        // derivation_path: identity.derivation_path.0,
        derivation_path: identity.derivation_path,
        key_id: identity.key_id,
    })
    .await
    .map(fetch_tuple0)
    .map_err(|(rejection_code, message)| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#ecdsa_public_key".to_string(),
        rejection_code,
        message,
    })
}

// 进行签名
// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_ecdsa
pub async fn ecdsa_sign(
    identity: EcdsaIdentity,
    message_hash: MessageHash,
) -> super::types::CanisterCallResult<SignWithEcdsaResponse> {
    ic_cdk::api::management_canister::ecdsa::sign_with_ecdsa(SignWithEcdsaArgument {
        message_hash: message_hash.0,
        derivation_path: identity.derivation_path,
        key_id: identity.key_id,
    })
    .await
    .map(fetch_tuple0)
    .map_err(|(rejection_code, message)| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#sign_with_ecdsa".to_string(),
        rejection_code,
        message,
    })
}
