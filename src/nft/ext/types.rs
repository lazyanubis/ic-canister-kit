use candid::CandidType;
use serde::Deserialize;

use crate::identity::UserId;

pub use super::allowance::{ExtAllowanceArgs, ExtAllowanceResult, ExtApproveArgs};
pub use super::batch::{
    ExtBalanceBatchArgs, ExtBalanceBatchResult, ExtBatchError, ExtTransferBatchArgs,
    ExtTransferBatchResult,
};
pub use super::common::{ExtMetadataResult, ExtSupplyResult};
pub use super::core::{
    ExtBalanceArgs, ExtBalanceResult, ExtTransferArgs, ExtTransferError, ExtTransferResult,
    StableTransferArgs,
};
pub use super::nonfungible::{ExtBearerResult, ExtMintArgs};

pub type ExtTokenIndex = u32; // 每个币序号  不能是 64 位的，为了防止别人计算的 token identifier 不一样
pub type ExtTokenIdentifier = String; // 合约标识符，实际是: 0x0Atid + canister id + index   u8 数组和起来后形成一个像 principal 一样的字符串
pub type ExtBalance = candid::Nat; // 余额 是自然数
pub type ExtAccountIdentifier = Vec<u8>; // 账户识别 一般是 account id，如果用户使用的是 principal 也要和 subaccount 一起转换成对应的 account id
pub type ExtAccountIdentifierHex = String; // 账户识别 一般是 account id，如果用户使用的是 principal 也要和 subaccount 一起转换成对应的 account id
pub type ExtSubAccount = Vec<u8>; // 子账户

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Debug, Clone)]
pub enum ExtUser {
    #[serde(rename = "address")]
    Address(ExtAccountIdentifierHex),
    #[serde(rename = "principal")]
    Principal(UserId),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ExtCommonError {
    InvalidToken(ExtTokenIdentifier),
    Other(String),
}

#[allow(non_camel_case_types)]
#[derive(CandidType, Deserialize)]
pub enum ExtTokenMetadata {
    fungible {
        name: String,
        symbol: String,
        decimals: u8,
        metadata: Option<Vec<u8>>,
    },
    nonfungible {
        metadata: Option<Vec<u8>>,
    },
}

impl ExtUser {
    pub fn to_account_identity_hex(&self) -> ExtAccountIdentifierHex {
        match self {
            ExtUser::Address(account_hex) => account_hex.clone(),
            ExtUser::Principal(user_id) => ExtUser::parse_account_identifier_hex(user_id, &None), // 默认子账户
        }
    }
    pub fn to_account_identity(&self) -> ExtAccountIdentifier {
        ExtUser::parse_to_bytes(&self.to_account_identity_hex())
    }
    #[allow(unused)]
    pub fn to_principal(&self) -> Option<UserId> {
        match self {
            ExtUser::Address(_) => None,
            ExtUser::Principal(user_id) => Some(user_id.clone()), // 默认子账户
        }
    }
    pub fn parse_account_identifier_hex(
        user_id: &UserId,
        subaccount: &Option<ExtSubAccount>,
    ) -> ExtAccountIdentifierHex {
        let result = ExtUser::parse_account_identifier_bytes(user_id, &subaccount);

        hex::encode(&result)
    }
    pub fn parse_account_identifier(
        user_id: &UserId,
        subaccount: &Option<ExtSubAccount>,
    ) -> ExtAccountIdentifier {
        ExtUser::parse_account_identifier_bytes(user_id, &subaccount)
    }

    pub fn to_hex(account_id: &ExtAccountIdentifier) -> ExtAccountIdentifierHex {
        hex::encode(&account_id)
    }

    pub fn parse_account_identifier_bytes(
        user_id: &UserId,
        subaccount: &Option<ExtSubAccount>,
    ) -> Vec<u8> {
        let subaccount: Vec<u8> = subaccount.clone().unwrap_or_else(|| [0; 32].to_vec()); // 默认子账户 应该全是 0

        assert!(subaccount.len() == 32, "Invalid subaccount");

        // ! 惊险啊，这个数组的长度是有区别的啊
        // ? 不用补齐 32 位
        // loop {
        //     if subaccount.len() >= 32 {
        //         break;
        //     }
        //     subaccount.insert(0, 0);
        // }

        use sha2::Digest;
        let mut hasher = sha2::Sha224::new();
        hasher.update(b"\x0Aaccount-id");
        hasher.update(user_id.as_slice());
        hasher.update(&subaccount[..]);
        let hash: [u8; 28] = hasher.finalize().into();

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&hash);
        let crc32_bytes = hasher.finalize().to_be_bytes();

        let mut result = [0u8; 32];
        result[0..4].copy_from_slice(&crc32_bytes[..]);
        result[4..32].copy_from_slice(hash.as_ref());

        result.to_vec()
    }

    pub fn parse_to_bytes(account_identifier: &ExtAccountIdentifierHex) -> Vec<u8> {
        let id = account_identifier.clone().into_bytes();
        let mut r = vec![];

        fn parse(i: u8) -> u8 {
            if i <= 57 {
                i - 48
            } else {
                i - 97 + 10
            }
        }

        for i in 0..(id.len() / 2) {
            let b = parse(id[i * 2]) * 16 + parse(id[i * 2 + 1]);
            r.push(b);
        }

        r
    }

    pub fn nat_to_subaccount(n: candid::Nat) -> ExtSubAccount {
        let mut n = n.0.to_bytes_be();
        loop {
            if n.len() >= 32 {
                break;
            }
            n.insert(0, 0);
        }
        loop {
            if n.len() <= 32 {
                break;
            }
            n.remove(0);
        }
        n
    }
}
