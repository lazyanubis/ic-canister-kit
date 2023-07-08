/// 接受转入 cycles 的结果类型
#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct WalletReceiveResult {
    accepted: u64,
}

/// 通用的查询罐子剩余 cycles 的接口
#[inline]
pub fn wallet_balance() -> candid::Nat {
    return candid::Nat::from(ic_cdk::api::canister_balance128()); // Cycles.balance()
}

/// 接受转入 cycles
// #[inline]
pub fn wallet_receive() -> WalletReceiveResult {
    // 获取调用者转入的可接受的 cycles 数量
    let available = ic_cdk::api::call::msg_cycles_available128(); // Cycles.available();

    if available == 0 {
        return WalletReceiveResult { accepted: 0 };
    }

    // 接受所有的转入
    let accepted = ic_cdk::api::call::msg_cycles_accept128(available); // Cycles.accept(available)

    // ! 判断是否接受成功，不成功就要报错
    assert!(accepted == available);

    // 返回接受的 cycles 数量
    WalletReceiveResult {
        accepted: accepted as u64,
    }
}

// #[ic_cdk::query(name = "wallet_balance")]
// #[candid::candid_method(query, rename = "wallet_balance")]
// pub fn wallet_balance() -> candid::Nat {
//     ic_canister_kit::cycles::wallet_balance()
// }

// #[ic_cdk::query(name = "wallet_receive")]
// #[candid::candid_method(query, rename = "wallet_receive")]
// pub fn wallet_receive() -> WalletReceiveResult {
//     ic_canister_kit::cycles::wallet_receive()
// }
