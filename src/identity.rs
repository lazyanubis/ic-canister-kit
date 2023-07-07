pub type CanisterId = candid::Principal;

pub type UserId = candid::Principal;

pub type CallerId = candid::Principal;

pub type SubAccount = Vec<u8>; // 子账户 必须是 32 长度

pub type AccountIdentifier = Vec<u8>; // 账户
pub type AccountIdentifierHex = String; // 账户 一般是 account id，如果用户使用的是 principal 也要和 sub_account 一起转换成对应的 account id

pub fn unwrap_account_identifier_hex(
    account_identifier: AccountIdentifierHex,
) -> AccountIdentifier {
    hex::decode(&account_identifier).unwrap()
}

pub fn wrap_account_identifier(account_identifier: AccountIdentifier) -> AccountIdentifierHex {
    hex::encode(&account_identifier)
}

pub fn parse_account_identifier(
    user_id: &UserId,
    sub_account: &Option<SubAccount>,
) -> AccountIdentifier {
    parse_account_identifier_bytes(user_id, &sub_account)
}

pub fn parse_account_identifier_hex(
    user_id: &UserId,
    sub_account: &Option<SubAccount>,
) -> AccountIdentifierHex {
    wrap_account_identifier(parse_account_identifier(user_id, sub_account))
}

fn parse_account_identifier_bytes(
    user_id: &UserId,
    sub_account: &Option<SubAccount>,
) -> AccountIdentifier {
    let sub_account: Vec<u8> = sub_account.clone().unwrap_or_else(|| [0; 32].to_vec()); // 默认子账户 应该全是 0

    assert!(sub_account.len() == 32, "Invalid SubAccount");

    // ! 惊险啊，这个数组的长度是有区别的啊
    // ? 不用补齐 32 位
    // let mut sub_account: Vec<u8> = sub_account;
    // loop {
    //     if sub_account.len() >= 32 {
    //         break;
    //     }
    //     sub_account.insert(0, 0);
    // }

    use sha2::Digest;
    let mut hasher = sha2::Sha224::new();
    hasher.update(b"\x0Aaccount-id");
    hasher.update(user_id.as_slice());
    hasher.update(&sub_account[..]);
    let hash: [u8; 28] = hasher.finalize().into();

    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&hash);
    let crc32_bytes = hasher.finalize().to_be_bytes();

    let mut result = [0u8; 32];
    result[0..4].copy_from_slice(&crc32_bytes[..]);
    result[4..32].copy_from_slice(hash.as_ref());

    result.to_vec()
}

pub fn parse_u64_to_sub_account(sub_account: u64) -> AccountIdentifier {
    let mut list: [u8; 32] = [0; 32];
    for i in 0..8 {
        list[24 + i] = (sub_account >> 8 * (7 - i)) as u8
    }
    list.to_vec()
}
