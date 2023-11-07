use std::cell::RefCell;

#[cfg(feature = "stable_initial")]
pub mod initial;

#[cfg(feature = "stable_upgrade")]
pub mod upgrade;

#[cfg(feature = "stable_maintainable")]
pub mod maintainable;

#[cfg(feature = "stable_permissable")]
pub mod permissable;

#[cfg(feature = "stable_recordable")]
pub mod recordable;

#[cfg(feature = "stable_notifiable")]
pub mod notifiable;

#[cfg(feature = "stable_uploads")]
pub mod uploads;

#[cfg(feature = "stable_hashmap")]
pub mod hashmap;

pub mod types;

// 持久化相关接口

// 升级后恢复
pub fn restore_after_upgrade<R>(state: &RefCell<R>)
where
    R: candid::CandidType + for<'d> candid::Deserialize<'d>,
{
    let mut state = state.borrow_mut();
    let (stable_state,): (R,) = ic_cdk::storage::stable_restore().unwrap();
    *state = stable_state;
}

// 升级前保存
pub fn store_before_upgrade<S>(state: &RefCell<S>)
where
    S: candid::CandidType + Default,
{
    let stable_state: S = std::mem::take(&mut *state.borrow_mut());
    ic_cdk::storage::stable_save((stable_state,)).unwrap();
}

/*

引入包后, 直接使用如下代码即可拥有可恢复数据对象


// ================= 需要持久化的数据 ================

thread_local! {
    // 存储系统数据
    static STATE: RefCell<State> = RefCell::default();
}

// ==================== 升级时的恢复逻辑 ====================

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    STATE.with(|state| {
        ic_canister_kit::stable::restore_after_upgrade(state);
        state.borrow_mut().upgrade(); // ! 恢复后要进行升级到最新版本
    });
}

// ==================== 升级时的保存逻辑，下次升级执行 ====================

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        state.borrow().must_be_maintaining(); // ! 必须是维护状态, 才可以升级
        ic_canister_kit::stable::store_before_upgrade(state);
    });
}

// ==================== 工具方法 ====================

/// 外界需要系统状态时
#[allow(unused)]
pub fn with_state<F, R>(callback: F) -> R
where
    F: FnOnce(&State) -> R,
{
    STATE.with(|_state| {
        let state = _state.borrow(); // 取得不可变对象
        callback(&state)
    })
}

/// 需要可变系统状态时
#[allow(unused)]
pub fn with_mut_state<F, R>(callback: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|_state| {
        let mut state = _state.borrow_mut(); // 取得可变对象
        callback(&mut state)
    })
}

*/
