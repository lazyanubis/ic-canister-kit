use super::identity::CanisterId;

pub mod types;

pub type NFTTokenIndex = u32;
pub type NFTTokenIdentifier = String;

const TDS: [u8; 4] = [10, 116, 105, 100]; //b"\x0Atid"

pub fn parse_token_identifier(canister_id: CanisterId, index: NFTTokenIndex) -> NFTTokenIdentifier {
    let mut array = vec![];

    array.extend_from_slice(&TDS); // 加上前缀

    array.extend_from_slice(canister_id.as_slice());

    array.extend_from_slice(&index.to_be_bytes()); // 加上序号

    // ic_cdk::println!("calc_token_identifier {:?}", array);

    candid::Principal::try_from_slice(&array).unwrap().to_text() // 不会转换失败的
}
