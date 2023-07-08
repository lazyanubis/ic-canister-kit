use std::collections::HashMap;

use crate::stable::Stable;

use super::types::{Media, NFTInfo, Nft};

#[derive(Debug, Default)]
pub struct NftStorage {
    pub info: NFTInfo,
    pub nfts: Vec<Nft>,
    pub nfts_map: HashMap<String, usize>,
}

// 对象持久化
pub type NftStorageState = (NFTInfo, Vec<Nft>);

impl Stable<NftStorageState, NftStorageState> for NftStorage {
    fn save(&mut self) -> NftStorageState {
        let info = std::mem::take(&mut self.info);
        let nfts = std::mem::take(&mut self.nfts);
        (info, nfts)
    }

    fn restore(&mut self, state: NftStorageState) {
        let _ = std::mem::replace(&mut self.info, state.0);
        let _ = std::mem::replace(&mut self.nfts, state.1);
        self.nfts_map = {
            let mut map = HashMap::with_capacity(self.nfts.len());
            for (i, v) in self.nfts.iter().enumerate() {
                map.insert(v.name.clone(), i);
            }
            map
        };
    }
}

impl NftStorage {
    pub fn set_name(&mut self, name: String) {
        self.info.name = name;
    }
    pub fn set_symbol(&mut self, symbol: String) {
        self.info.symbol = symbol;
    }
    pub fn set_logo(&mut self, logo: Option<Media>) {
        self.info.logo = logo;
    }
    pub fn set_maintaining(&mut self, maintaining: Option<Media>) {
        self.info.maintaining = maintaining;
    }
}
