use candid::CandidType;
use serde::Deserialize;

use crate::{common::result::MotokoResult, identity::UserId, types::NftStorage};

use super::types::*;

// =============== 查询指定 NFT 授权情况 ===============
// 查询额度参数
#[derive(CandidType, Deserialize)]
pub struct ExtAllowanceArgs {
    pub token: ExtTokenIdentifier,
    pub owner: ExtUser,
    pub spender: UserId,
}
// 查询额度的结果
pub type ExtAllowanceResult = MotokoResult<ExtBalance, ExtCommonError>;

// =============== 授权额度 ===============
// 授权参数
#[derive(CandidType, Deserialize)]
pub struct ExtApproveArgs {
    pub subaccount: Option<ExtSubaccount>, // 注意，这里指的是所有者的子账户，也就是调用者的子账户
    pub token: ExtTokenIdentifier,
    pub allowance: ExtBalance,
    pub spender: UserId,
}

// ================ 接口 =================

pub trait ExtAllowance {
    fn allowance(&self, args: ExtAllowanceArgs) -> ExtAllowanceResult;
    fn approve(&mut self, args: ExtApproveArgs) -> bool;
}

impl ExtAllowance for NftStorage {
    // 1. allowance 查询允许额度
    fn allowance(&self, args: ExtAllowanceArgs) -> ExtAllowanceResult {
        let index = match super::utils::parse_token_index_with_self_canister(&args.token) {
            Ok(index) => index as usize,
            Err(e) => return MotokoResult::Err(e),
        }; // token 标识的正确性也要检查

        let owner = args.owner.to_account_identity(); // 参数指定的所有者

        match self.nfts.get(index) {
            Some(nft) => {
                if nft.owner.to_vec() != owner {
                    // token 和 指定的 owner 不一致
                    return MotokoResult::Err(ExtCommonError::Other(format!("Invalid owner")));
                }
                // 下面检测有没有授权
                if let Some(approved) = nft.approved {
                    if approved == args.spender {
                        // 授权就是这个人
                        return MotokoResult::Ok(candid::Nat::from(1)); // 返回 1
                    }
                }
                return MotokoResult::Ok(candid::Nat::from(0)); // 没有授权给这个人 返回 0
            }
            None => return MotokoResult::Err(ExtCommonError::InvalidToken(args.token)),
        }
    }

    // 2. approve 授权额度
    // ? 标准接口返回值应该是 () 但是有人需要以布尔值表示授权成功或失败，因此修改
    fn approve(&mut self, args: ExtApproveArgs) -> bool {
        let index =
            super::utils::parse_token_index_with_self_canister(&args.token).unwrap() as usize; // token 标识的正确性也要检查

        let caller = ExtUser::parse_account_identifier(&ic_cdk::api::caller(), &args.subaccount); // 授权必须是调用者本人

        let spender = args.spender;

        // 调用存储模块的授权方法
        match self.nfts.get_mut(index) {
            Some(nft) => {
                // 需要检查权限
                // 1. 检查是不是当前的所有者
                if nft.owner.to_vec() != caller {
                    return false;
                }

                // =========== 进行修改 ===========
                // 更改授权信息
                nft.approved = Some(spender);
                // =========== 修改完毕 ===========

                true
            }
            None => false, // token 不存在，不能授权
        }
    }
}
