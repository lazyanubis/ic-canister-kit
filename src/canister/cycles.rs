//! 和 罐子 的 Cycles 相关

use crate::identity::CanisterId;

/*

引入包后, 直接使用如下方法即可增加查询和接收 cycles 的接口

#[ic_cdk::query]
pub fn wallet_balance() -> candid::Nat {
    ic_canister_kit::canister::cycles::wallet_balance()
}

#[ic_cdk::update]
pub fn wallet_receive() -> candid::Nat {
    ic_canister_kit::canister::cycles::wallet_receive()
}

*/

/// 通用的查询罐子剩余 cycles 的接口
#[inline]
pub fn wallet_balance() -> candid::Nat {
    candid::Nat::from(super::self_canister_cycles()) // Cycles.balance()
}

/// 接受转入 cycles
#[inline]
pub fn wallet_receive<F>(callback: F) -> candid::Nat
where
    F: FnOnce(u128),
{
    // 获取调用者转入的可接受的 cycles 数量
    let available = ic_cdk::api::msg_cycles_available(); // Cycles.available();

    if available == 0 {
        return candid::Nat::from(0_u128);
    }

    // 接受所有的转入
    let accepted = ic_cdk::api::msg_cycles_accept(available); // Cycles.accept(available)

    // ! 判断是否接受成功，不成功就要报错
    assert!(accepted == available);

    callback(accepted); // 回调

    // 返回接受的 cycles 数量
    candid::Nat::from(accepted)
}

// ========================= 充值 cycles =========================

/// 充值余额
/// ! 任何人都可以调用
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-deposit_cycles
pub async fn deposit_cycles(
    canister_id: CanisterId,
    cycles: u128,
) -> Result<(), super::types::CanisterCallError> {
    let call_result = ic_cdk::management_canister::deposit_cycles(
        &ic_cdk::management_canister::DepositCyclesArgs { canister_id },
        cycles,
    )
    .await;
    super::wrap_call_result(canister_id, "ic#deposit_cycles", call_result)
}

// ========================= 自定义接口调用 =========================

/// 查询罐子余额
/// ! 必须实现 wallet_balance : () -> (nat) query 接口
pub async fn call_wallet_balance(
    canister_id: CanisterId,
) -> super::types::CanisterCallResult<candid::Nat> {
    let call_result = ic_cdk::call::Call::unbounded_wait(canister_id, "wallet_balance").await;
    super::fetch_and_wrap_call_result(canister_id, "deposit_cycles", call_result)
}

/// 充值罐子余额
/// ! 必须实现 wallet_receive : () -> (nat) 接口
pub async fn call_wallet_receive(
    canister_id: CanisterId,
    cycles: u64,
) -> super::types::CanisterCallResult<candid::Nat> {
    let call_result = ic_cdk::call::Call::unbounded_wait(canister_id, "wallet_receive")
        .with_cycles(cycles as u128)
        .await;
    super::fetch_and_wrap_call_result(canister_id, "wallet_receive", call_result)
}
