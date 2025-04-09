use crate::identity::{CanisterId, UserId, self_canister_id};

use super::types::*;

const TDS: [u8; 4] = [10, 116, 105, 100]; //b"\x0Atid"

pub fn parse_token_identifier_with_self_canister(index: ExtTokenIndex) -> ExtTokenIdentifier {
    parse_token_identifier(self_canister_id(), index)
}

pub fn parse_token_identifier(canister_id: CanisterId, index: ExtTokenIndex) -> ExtTokenIdentifier {
    let mut array = vec![];

    array.extend_from_slice(&TDS); // 加上前缀

    array.extend_from_slice(canister_id.as_slice());

    array.extend_from_slice(&index.to_be_bytes()); // 加上序号

    // ic_cdk::println!("calc_token_identifier {:?}", array);

    #[allow::clippy(unwrap_used)] // ? SAFETY
    candid::Principal::try_from_slice(&array).unwrap().to_text() // 不会转换失败的
}

// 检查 token 标识是否合法
pub fn parse_token_index_with_self_canister(
    _token_identifier: &ExtTokenIdentifier,
) -> Result<ExtTokenIndex, ExtCommonError> {
    parse_token_index(self_canister_id(), _token_identifier)
}

pub fn parse_token_index(
    canister_id: CanisterId,
    _token_identifier: &ExtTokenIdentifier,
) -> Result<ExtTokenIndex, ExtCommonError> {
    let (canister, index) = _parse_token_identifier(_token_identifier);
    if &canister[..] != canister_id.as_slice() {
        // canister 不是本 canister 的 id，说明 token 不对
        return Err(ExtCommonError::InvalidToken(_token_identifier.to_string()));
    }
    Ok(index)
}

// 解析 token 标识
fn _parse_token_identifier(_token_identifier: &ExtTokenIdentifier) -> (Vec<u8>, ExtTokenIndex) {
    #[allow::clippy(unwrap_used)] // ? SAFETY
    let array = UserId::from_text(_token_identifier)
        .unwrap()
        .as_slice()
        .to_vec();

    // ic_cdk::println!("parse_token_identifier {:?}", array);

    // 1. 检查前 4 位的前缀是否是 TDS，如果不是直接返回
    if array.len() <= 4 || &array[0..4] != TDS {
        return (array, 0); // 直接返回
    }

    if array.len() <= 8 {
        return (array, 0); // 直接返回
    }

    // 2. 去掉前 4 位的前缀, 剩下的是 canister id 和序号

    let canister = &array[4..array.len() - 4];
    let index = &array[array.len() - 4..array.len()];
    let index = (index[0] as u32) << 24
        | (index[1] as u32) << 16
        | (index[2] as u32) << 8
        | (index[3] as u32);

    (canister.into(), index)
}
