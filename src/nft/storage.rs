use std::collections::HashMap;

use crate::{
    identity::wrap_account_identifier,
    types::{NFTOwnable, UserId},
};

use super::types::{MediaData, NFTInfo, Nft, NftView};

// 存储相关

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct NftStorage {
    pub info: NFTInfo,
    pub nfts: Vec<Nft>,
    pub nfts_map: HashMap<String, usize>,
}

impl NftStorage {
    pub fn set_name(&mut self, name: String) {
        self.info.name = name;
    }
    pub fn set_symbol(&mut self, symbol: String) {
        self.info.symbol = symbol;
    }
    pub fn set_logo(&mut self, logo: Option<MediaData>) {
        self.info.logo = logo;
    }
    pub fn set_thumbnail(&mut self, thumbnail: Option<MediaData>) {
        self.info.thumbnail = thumbnail;
    }
    pub fn set_maintaining(&mut self, maintaining: Option<MediaData>) {
        self.info.maintaining = maintaining;
    }
    pub fn get_nft(&self, index: usize) -> Option<&Nft> {
        self.nfts.get(index)
    }
    pub fn set_nft_rarity(&mut self, index: usize, rarity: String) {
        if let Some(nft) = self.nfts.get_mut(index) {
            nft.rarity = rarity;
        }
    }
    pub fn set_nft_content(&mut self, index: usize, content: Option<Vec<u8>>) {
        if let Some(nft) = self.nfts.get_mut(index) {
            nft.content = content;
        }
    }
    pub fn set_nft_metadata(&mut self, index: usize, no: usize, media: Option<MediaData>) {
        if let Some(nft) = self.nfts.get_mut(index) {
            let length = nft.metadata.len();
            match media {
                Some(media) => {
                    // 是想要添加内容
                    if no < length {
                        nft.metadata[no] = media;
                    } else if no == length {
                        nft.metadata.push(media);
                    } else {
                        ic_cdk::trap("Wrong token_id for setting metadata of nft"); // ! 中止执行
                    }
                }
                None => {
                    // 是想要移除指定数据
                    if no < length {
                        nft.metadata.remove(no);
                    } else {
                        ic_cdk::trap("Wrong token_id for removing metadata of nft"); // ! 中止执行
                    }
                }
            }
        }
    }
    pub fn set_nft_thumbnail(&mut self, index: usize, thumbnail: Option<MediaData>) {
        if let Some(nft) = self.nfts.get_mut(index) {
            nft.thumbnail = thumbnail;
        }
    }
    pub fn set_nft_ownable(&mut self, index: usize, ownable: NFTOwnable) {
        if let Some(nft) = self.nfts.get_mut(index) {
            nft.ownable = ownable;
        }
    }
    pub fn get_nft_rarity(&self, index: usize) -> String {
        if let Some(nft) = self.nfts.get(index) {
            return nft.rarity.clone();
        }
        ic_cdk::trap("Wrong token_id for nft"); // ! 中止执行
    }
    pub fn get_nft_metadata(&self, index: usize, no: usize) -> Option<MediaData> {
        if let Some(nft) = self.nfts.get(index) {
            return nft.metadata.get(no).and_then(|m| Some(m.clone()));
        }
        ic_cdk::trap("Wrong token_id for nft"); // ! 中止执行
    }
    pub fn get_nft_all(&self) -> Vec<NftView> {
        self.nfts
            .iter()
            .map(|n| NftView {
                name: n.name.clone(),
                owner: wrap_account_identifier(&n.owner),
                approved: n.approved.and_then(|a| Some(UserId::to_text(&a))),
            })
            .collect()
    }
    pub fn len(&self) -> usize {
        self.nfts.len()
    }
}
