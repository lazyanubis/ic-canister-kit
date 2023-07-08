use crate::{results::MotokoResult, types::NftStorage};

use super::types::*;

// =============== 指定 NFT 的元数据查询 ===============

// 查询元数据结果
pub type ExtMetadataResult = MotokoResult<ExtTokenMetadata, ExtCommonError>;

// =============== 总量查询 ===============

// 查询总供应量结果
pub type ExtSupplyResult = MotokoResult<ExtBalance, ExtCommonError>;

// ================ 接口 =================

pub trait ExtCommon {
    fn metadata(&self, token: ExtTokenIdentifier) -> ExtMetadataResult;
    fn supply(&self, _token: ExtTokenIdentifier) -> ExtSupplyResult;
}

impl ExtCommon for NftStorage {
    // 1. metadata 查找某 token 的元数据 EXT标准
    fn metadata(&self, token: ExtTokenIdentifier) -> ExtMetadataResult {
        let index = match super::utils::parse_token_index(&token) {
            Ok(index) => index as usize,
            Err(e) => return MotokoResult::Err(e),
        }; // token 标识的正确性也要检查

        match self.nfts.get(index) {
            Some(nft) => {
                return MotokoResult::Ok(ExtTokenMetadata::nonfungible {
                    metadata: nft.content.clone(),
                })
            }
            None => return MotokoResult::Err(ExtCommonError::InvalidToken(token)), // token 不存在
        }
    }

    // 2. supply 查询总供应量 EXT标准
    fn supply(&self, _token: ExtTokenIdentifier) -> ExtSupplyResult {
        MotokoResult::Ok(candid::Nat::from(self.nfts.len()))
    }
}
