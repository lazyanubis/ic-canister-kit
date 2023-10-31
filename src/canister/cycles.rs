use ic_cdk::api::management_canister::provisional::CanisterIdRecord;

use crate::identity::CanisterId;

use super::{types::CallError, unwrap_call_result};

/// 和 罐子 的 Cycles 相关
/*

引入包后, 直接使用如下方法即可增加查询和接收 cycles 的接口

#[ic_cdk::query]
pub fn wallet_balance() -> candid::Nat {
    ic_canister_kit::canister::cycles::wallet_balance()
}

#[ic_cdk::update]
pub fn wallet_receive() -> ic_canister_kit::canister::cycles::WalletReceiveResult {
    ic_canister_kit::canister::cycles::wallet_receive()
}

*/

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
pub fn wallet_receive<F>(callback: F) -> WalletReceiveResult
where
    F: Fn(u128),
{
    // 获取调用者转入的可接受的 cycles 数量
    let available = ic_cdk::api::call::msg_cycles_available128(); // Cycles.available();

    if available == 0 {
        return WalletReceiveResult { accepted: 0 };
    }

    // 接受所有的转入
    let accepted = ic_cdk::api::call::msg_cycles_accept128(available); // Cycles.accept(available)

    // ! 判断是否接受成功，不成功就要报错
    assert!(accepted == available);

    callback(accepted); // 回调

    // 返回接受的 cycles 数量
    WalletReceiveResult {
        accepted: accepted as u64,
    }
}

// 充值余额
// ! 是 Controller 才能充值
pub async fn deposit_cycles(canister_id: CanisterId, cycles: u128) -> Result<(), String> {
    let call_result = ic_cdk::api::management_canister::main::deposit_cycles(
        CanisterIdRecord { canister_id },
        cycles,
    )
    .await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} deposit_cycles {} failed: {:?} {}",
            canister_id.to_text(),
            cycles,
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

// 查询罐子余额
pub async fn call_wallet_balance(canister_id: &CanisterId) -> candid::Nat {
    let call_result: Result<(candid::Nat,), CallError> =
        ic_cdk::api::call::call(canister_id.clone(), "wallet_balance", ()).await;
    unwrap_call_result(canister_id, "wallet_balance", call_result)
}

// 充值罐子余额
pub async fn call_wallet_receive(canister_id: &CanisterId, cycles: u64) -> WalletReceiveResult {
    let call_result: Result<(WalletReceiveResult,), CallError> =
        ic_cdk::api::call::call_with_payment(canister_id.clone(), "wallet_receive", (), cycles)
            .await;
    unwrap_call_result(canister_id, "wallet_receive", call_result)
}
