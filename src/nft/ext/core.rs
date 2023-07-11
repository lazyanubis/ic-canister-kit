use candid::CandidType;
use serde::Deserialize;

use crate::{
    identity::caller,
    results::MotokoResult,
    types::{CanisterId, NftStorage},
};

use super::types::*;

// ================ 查询余额 =================

// 查询余额参数
#[derive(CandidType, Deserialize)]
pub struct ExtBalanceArgs {
    pub user: ExtUser,
    pub token: ExtTokenIdentifier,
}

// 查询余额的结果
pub type ExtBalanceResult = MotokoResult<ExtBalance, ExtCommonError>;

// ================ 转账 =================
// 转账参数
#[derive(CandidType, Deserialize)]
pub struct ExtTransferArgs {
    pub from: ExtUser,
    pub to: ExtUser,
    pub token: ExtTokenIdentifier,         // 是文本
    pub amount: ExtBalance,                // 数量 NFT转账必须是 1
    pub memo: Vec<u8>,                     // 转账附带的 memo
    pub notify: bool,                      // 是否要通知对应的 canister
    pub subaccount: Option<ExtSubAccount>, // 注意，这里指的是调用者的子账户
}

#[derive(CandidType, Deserialize)]
pub enum ExtTransferError {
    Unauthorized(ExtAccountIdentifierHex),
    InsufficientBalance,
    Rejected, // Rejected by canister
    InvalidToken(ExtTokenIdentifier),
    CannotNotify(ExtAccountIdentifierHex),
    Other(String),
}

#[derive(Clone)]
pub struct StableTransferArgs {
    pub from: ExtUser,
    pub to: ExtUser,
    pub token: ExtTokenIdentifier,         // 是文本
    pub amount: ExtBalance,                // 数量 NFT转账必须是 1
    pub memo: Vec<u8>,                     // 转账附带的 memo
    pub notify: bool,                      // 是否要通知对应的 canister
    pub subaccount: Option<ExtSubAccount>, // 注意，这里指的是调用者的子账户
}

// 转账返回的结果
pub type ExtTransferResult = MotokoResult<ExtBalance, ExtTransferError>;

// ================ 接口 =================

pub trait ExtCore {
    fn extensions(&self) -> Vec<String>;
    fn balance(&self, args: ExtBalanceArgs) -> ExtBalanceResult;
    fn transfer(
        &mut self,
        args: ExtTransferArgs,
    ) -> (ExtTransferResult, Option<StableTransferArgs>);
}

impl ExtCore for NftStorage {
    // 1. extensions 获取拓展信息 EXT标准
    fn extensions(&self) -> Vec<String> {
        vec![
            // ? 用户应当自己选择实现的方法
            "@ext/common".to_string(), // 1. metadata 元数据查询  2. supply 总供应量
            "@ext/allowance".to_string(), // 1. allowance 查询允许额度 2. approve 授权额度
            "@ext/nonfungible".to_string(), // 1. bearer 查询所有人 2. mintNFT 铸币
            "@ext/batch".to_string(),  // 1. balance_batch 批量查询余额 2. transfer_batch 批量转账
        ]
    }

    // 2. balance 查询余额 对于 NFT 查询余额就是 有就是 1 无就是 0 EXT标准
    fn balance(&self, args: ExtBalanceArgs) -> ExtBalanceResult {
        let index = match super::utils::parse_token_index_with_self_canister(&args.token) {
            Ok(index) => index as usize,
            Err(e) => return MotokoResult::Err(e),
        }; // token 标识的正确性也要检查

        let owner = args.user.to_account_identity(); // 参数指定的所有者

        let balance = self.nfts.get(index).and_then(|nft| {
            if nft.owner == owner {
                return Some(1);
            }
            return Some(0);
        });

        match balance {
            Some(num) => MotokoResult::Ok(candid::Nat::from(num)),
            None => MotokoResult::Err(ExtCommonError::InvalidToken(args.token)),
        }
    }

    // 3. transfer 转账接口 EXT标准
    fn transfer(
        &mut self,
        args: ExtTransferArgs,
    ) -> (ExtTransferResult, Option<StableTransferArgs>) {
        // NFT 转账的数量必须是 1
        if args.amount != candid::Nat::from(1) {
            return (
                MotokoResult::Err(ExtTransferError::Other(format!("Must use amount of 1"))),
                None,
            );
        };
        // token 标识的正确性也要检查
        let index = match super::utils::parse_token_index_with_self_canister(&args.token) {
            Ok(index) => index as usize,
            Err(_) => {
                return (
                    MotokoResult::Err(ExtTransferError::InvalidToken(args.token)),
                    None,
                )
            }
        };

        let ExtTransferArgs {
            from,
            to,
            token,
            amount,
            memo,
            notify,
            subaccount,
        } = args;

        // 调用存储模块的转账方法
        // ! 该方法会检查权限以及修改系统
        let args = StableTransferArgs {
            from,
            to,
            token,
            amount: amount.clone(),
            memo: memo.clone(),
            notify,
            subaccount,
        };
        let result = do_transfer(self, index as usize, args.clone());

        match result {
            Ok((_owner, _receiver)) => {
                // 说明转账成功
                (MotokoResult::Ok(candid::Nat::from(amount)), Some(args)) // 把 amount 用起来
            }
            Err(r) => (MotokoResult::Err(r.into()), None),
        }
    }
}

// 进行转账
// ! 有进行权限检查
fn do_transfer(
    storage: &mut NftStorage,
    index: usize,
    args: StableTransferArgs,
) -> Result<(ExtAccountIdentifierHex, ExtAccountIdentifierHex), ExtTransferError> {
    // 取出参数中的信息
    let caller = ExtUser::parse_account_identifier(&caller(), &args.subaccount); // 调用者 使用 NFT 的人
    let owner = args.from.to_account_identity(); // 参数指定的所有者
    let receiver = args.to.to_account_identity(); // 参数指定的接收者

    // 1. 先检查错误
    if let Some(err) = match storage.nfts.get_mut(index) {
        Some(_nft) => {
            // 1. 检查是不是所有者
            if _nft.owner != owner {
                // 连所属人都指定错误，别转账了
                return Err(ExtTransferError::Unauthorized(ExtUser::to_hex(&owner)));
            }

            if owner != caller {
                // 花费的人不是当前所有者，需要检查授权信息对不对
                match &_nft.approved {
                    Some(_approved) => {
                        if ExtUser::parse_account_identifier(_approved, &None) != caller {
                            // 授权对象不是当前调用者，不能转账
                            return Err(ExtTransferError::Unauthorized(ExtUser::to_hex(&caller)));
                        }
                        // else 的情况授权的人就是 caller(spender)
                    }
                    None => return Err(ExtTransferError::Unauthorized(ExtUser::to_hex(&caller))), // 从来没有授权，不能转账
                }
            }
            // else 的情况所有者和花费者是同一人

            None //
        }
        None => Some(ExtTransferError::InvalidToken(args.token.clone())),
    } {
        return Err(err); // 如果有错误，就直接返回
    }

    // 2. 这里不通知，直接修改返回即可
    if let Some(_nft) = storage.nfts.get_mut(index) {
        // =========== 进行修改 ===========
        // 移除授权信息
        _nft.approved = None;
        // 更改所有者
        _nft.owner = receiver.clone();
        // =========== 修改完毕 ===========
    }

    return Ok((ExtUser::to_hex(&owner), ExtUser::to_hex(&receiver)));
}

#[inline]
pub async fn notify_transfer_message(
    canister_id: CanisterId,
    args: StableTransferArgs,
) -> Result<Option<candid::Nat>, String> {
    let method = "tokenTransferNotification";
    let call_result: Result<(Option<candid::Nat>,), (ic_cdk::api::call::RejectionCode, String)> =
        crate::canister_call::call_canister(
            canister_id,
            method,
            (args.token, args.from, args.amount, args.memo),
        )
        .await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        let err = format!(
            "canister: {} call: {} failed: {:?} {}",
            canister_id.to_text(),
            method,
            err.0,
            err.1
        );
        // panic!("{}", err);
        return Err(err);
    }
    Ok(call_result.unwrap().0)
}
