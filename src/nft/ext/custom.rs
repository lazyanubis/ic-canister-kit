use crate::types::NftStorage;

use super::super::types::*;

pub trait ExtCustom {
    fn set_nft_rarity_by_token_identifier(
        &mut self,
        token_id: ExtTokenIdentifier,
        rarity: String,
        content: Option<Option<Vec<u8>>>,
    );
    fn set_nft_content_by_token_identifier(
        &mut self,
        token_id: ExtTokenIdentifier,
        content: Option<Vec<u8>>,
    );
    fn set_nft_metadata_by_token_identifier(
        &mut self,
        token_id: ExtTokenIdentifier,
        no: usize,
        media: Option<MediaData>,
    );
    fn set_nft_thumbnail_by_token_identifier(
        &mut self,
        token_id: ExtTokenIdentifier,
        media: Option<MediaData>,
    );

    fn get_nft_rarity_by_token_identifier(&self, token_id: ExtTokenIdentifier) -> String;
    fn get_nft_metadata_by_token_identifier(
        &self,
        token_id: ExtTokenIdentifier,
        no: usize,
    ) -> Option<MediaData>;
}

impl ExtCustom for NftStorage {
    fn set_nft_rarity_by_token_identifier(
        &mut self,
        token_id: ExtTokenIdentifier,
        rarity: String,
        content: Option<Option<Vec<u8>>>,
    ) {
        let token_index = super::utils::parse_token_index_with_self_canister(&token_id);
        if let Ok(index) = token_index {
            let index = index as usize;
            self.set_nft_rarity(index, rarity);

            if let Some(content) = content {
                self.set_nft_content(index, content);
            }
        }
    }
    fn set_nft_content_by_token_identifier(
        &mut self,
        token_id: ExtTokenIdentifier,
        content: Option<Vec<u8>>,
    ) {
        let token_index = super::utils::parse_token_index_with_self_canister(&token_id);
        if let Ok(index) = token_index {
            let index = index as usize;
            self.set_nft_content(index, content);
        }
    }
    fn set_nft_metadata_by_token_identifier(
        &mut self,
        token_id: ExtTokenIdentifier,
        no: usize,
        media: Option<MediaData>,
    ) {
        let token_index = super::utils::parse_token_index_with_self_canister(&token_id);
        if let Ok(index) = token_index {
            let index = index as usize;
            self.set_nft_metadata(index, no as usize, media);
        }
    }
    fn set_nft_thumbnail_by_token_identifier(
        &mut self,
        token_id: ExtTokenIdentifier,
        thumbnail: Option<MediaData>,
    ) {
        let token_index = super::utils::parse_token_index_with_self_canister(&token_id);
        if let Ok(index) = token_index {
            let index = index as usize;
            self.set_nft_thumbnail(index, thumbnail);
        }
    }

    fn get_nft_rarity_by_token_identifier(&self, token_id: ExtTokenIdentifier) -> String {
        let token_index = super::utils::parse_token_index_with_self_canister(&token_id);
        if let Ok(index) = token_index {
            let index = index as usize;
            return self.get_nft_rarity(index);
        }
        panic!("Wrong token_id for nft: {}", token_id);
    }
    fn get_nft_metadata_by_token_identifier(
        &self,
        token_id: ExtTokenIdentifier,
        no: usize,
    ) -> Option<MediaData> {
        let token_index = super::utils::parse_token_index_with_self_canister(&token_id);
        if let Ok(index) = token_index {
            let index = index as usize;
            return self.get_nft_metadata(index, no);
        }
        panic!("Wrong token_id for nft: {}", token_id);
    }
}
