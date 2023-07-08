use crate::{identity::UserId, types::NftStorage};

use super::types::*;

pub trait ExtState {
    fn get_registry(&self) -> Vec<(ExtTokenIndex, ExtAccountIdentifierHex)>;

    fn get_allowances(&self) -> Vec<(ExtTokenIndex, UserId)>;

    fn get_metadata(&self) -> Vec<(ExtTokenIndex, ExtTokenMetadata)>;

    fn get_tokens(&self) -> Vec<(ExtTokenIndex, ExtTokenMetadata)>;

    fn get_tokens_by_ids(
        &self,
        token_ids: Vec<ExtTokenIndex>,
    ) -> Vec<(ExtTokenIndex, ExtTokenMetadata)>;
}

impl ExtState for NftStorage {
    fn get_registry(&self) -> Vec<(ExtTokenIndex, ExtAccountIdentifierHex)> {
        self.nfts
            .iter()
            .map(|n| (n.index as u32, ExtUser::to_hex(&n.owner)))
            .collect()
    }

    fn get_allowances(&self) -> Vec<(ExtTokenIndex, UserId)> {
        self.nfts
            .iter()
            .filter(|n| n.approved.is_some())
            .map(|n| (n.index as u32, n.approved.unwrap().clone()))
            .collect()
    }

    fn get_metadata(&self) -> Vec<(ExtTokenIndex, ExtTokenMetadata)> {
        self.nfts
            .iter()
            .filter(|n| n.approved.is_some())
            .map(|n| {
                (
                    n.index as u32,
                    ExtTokenMetadata::nonfungible {
                        metadata: n.content.clone(),
                    },
                )
            })
            .collect()
    }

    fn get_tokens(&self) -> Vec<(ExtTokenIndex, ExtTokenMetadata)> {
        self.nfts
            .iter()
            .filter(|n| n.approved.is_some())
            .map(|n| {
                (
                    n.index as u32,
                    ExtTokenMetadata::nonfungible {
                        metadata: n.content.clone(), // yumi 需要，但是 entrepot 给的示例是空的，没的说
                                                     // metadata: None, // 和 getMetadata 的区别是，这个方法不返回内容
                    },
                )
            })
            .collect()
    }

    fn get_tokens_by_ids(
        &self,
        token_ids: Vec<ExtTokenIndex>,
    ) -> Vec<(ExtTokenIndex, ExtTokenMetadata)> {
        token_ids
            .iter()
            .map(|_id| match self.nfts.get(*_id as usize) {
                Some(n) => Some((
                    *_id,
                    ExtTokenMetadata::nonfungible {
                        metadata: n.content.clone(),
                    },
                )),
                None => None, // 如果对应的没有，就直接过滤了
            })
            .filter(|r: &Option<(u32, ExtTokenMetadata)>| r.is_some())
            .map(|r| r.unwrap())
            .collect()
    }
}
