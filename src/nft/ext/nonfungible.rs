use candid::CandidType;
use serde::Deserialize;

use crate::identity::self_canister_id;
use crate::{results::MotokoResult, types::NftStorage};

use super::super::types::*;

// =============== 查询指定 NFT 的持有者 ===============
// 查询持有人的结果
pub type ExtBearerResult = MotokoResult<ExtAccountIdentifierHex, ExtCommonError>;

// =============== 铸币 ===============
// 铸币参数
#[derive(CandidType, Deserialize, Clone)]
pub struct ExtMintArgs {
    pub to: ExtUser, // 铸币目标用户，可以直接是字符串，也可以是 principal
    pub metadata: Option<Vec<u8>>,
}

// ================ 接口 =================

pub trait ExtNonFungible {
    fn bearer(&self, token: ExtTokenIdentifier) -> ExtBearerResult;
    fn mint_nft(&mut self, args: ExtMintArgs);
}

impl ExtNonFungible for NftStorage {
    // 1. bearer 查询持有者 EXT标准
    fn bearer(&self, token: ExtTokenIdentifier) -> ExtBearerResult {
        let index = match super::utils::parse_token_index_with_self_canister(&token) {
            Ok(index) => index as usize,
            Err(e) => return MotokoResult::Err(e),
        }; // token 标识的正确性也要检查

        match self.nfts.get(index) {
            Some(nft) => MotokoResult::Ok(ExtUser::to_hex(&nft.owner)),
            None => MotokoResult::Err(ExtCommonError::InvalidToken(token)),
        }
    }

    // 2. mintNFT 铸币
    fn mint_nft(&mut self, args: ExtMintArgs) {
        let receiver = args.to.to_account_identity();
        let content = args.metadata;

        let token_index = self.nfts.len(); // 先取得当前长度，同时也就是新铸币的编号

        // =========== 进行修改 ===========
        // 这里增加了一个NFT
        self.nfts.push(Nft {
            index: token_index, // nft 的 id，这个数字才是真正的 nft
            name: super::utils::parse_token_identifier(self_canister_id(), token_index as u32), // nft 的名字
            owner: receiver,           // 所属人
            approved: None,            // 授权人 最多只有一个
            rarity: String::from(""),  // 新铸币没有稀有度
            content,                   // nft的内容 新建时的内容，貌似没有用处
            metadata: vec![],          // 该 nft 的元数据
            thumbnail: None,           // 缩略图
            ownable: NFTOwnable::None, // 隐私信息
        });
        // 调用 check_hash 方法，把新的 nft 添加到哈希表中
        // check_hash_after_nft_changed(token_index, &_state); // TODO
        // =========== 修改完毕 ===========

        // token_index //? 应该返回生成代币的序号，但是标准没有，因此就不返回了
    }
}
