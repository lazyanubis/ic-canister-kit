use std::cell::RefCell;

use crate::functions::types::RecordId;

// 持久化相关接口
// ! 如果通过其他方式，比如 ic-stable-structures，使用了持久化内存，则不能够使用下面传统的方式进行升级持久化

/// 升级后恢复
pub fn restore_after_upgrade<R>(state: &RefCell<R>) -> Option<RecordId>
where
    R: candid::CandidType + for<'d> serde::Deserialize<'d>,
{
    let mut state = state.borrow_mut();
    #[allow(clippy::unwrap_used)] // ? checked
    let (stable_state, record_id): (R, Option<RecordId>) =
        ic_cdk::storage::stable_restore().unwrap();
    *state = stable_state;
    record_id
}

/// 升级前保存
pub fn store_before_upgrade<S>(state: &RefCell<S>, record_id: Option<RecordId>)
where
    S: candid::CandidType + Default,
{
    let stable_state: S = std::mem::take(&mut *state.borrow_mut());
    #[allow(clippy::unwrap_used)] // ? checked
    ic_cdk::storage::stable_save((stable_state, record_id)).unwrap();
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
        let record_id = ic_canister_kit::stable::restore_after_upgrade(state);
        state.borrow_mut().upgrade(); // ! 恢复后要进行升级到最新版本
        let schedule = state.borrow().schedule_find();
        state.borrow_mut().init(CanisterInitialArg { schedule }); // ! 升级到最新版本后, 需要执行初始化操作
        state.borrow_mut().schedule_reload(); // * 重置定时任务
        let version = state.borrow().version();
        if let Some(record_id) = record_id {
            state
                .borrow_mut()
                .record_update(record_id, format!("Next version: {}", version));
        }
    });
}

// ==================== 升级时的保存逻辑，下次升级执行 ====================

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        #[allow(clippy::unwrap_used)] // ? checked
        state.borrow().pause_must_be_paused().unwrap(); // ! 必须是维护状态, 才可以升级
        state.borrow_mut().schedule_stop(); // * 停止定时任务
        ic_canister_kit::stable::store_before_upgrade(state);
        let record_id = state.borrow_mut().record_push(
            caller,
            RecordTopics::Upgrade.topic(),
            format!("Upgrade by {}", caller.to_text()),
        );
        ic_canister_kit::stable::store_before_upgrade(state, Some(record_id));
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
pub fn with_mut_state_without_record<F, R>(callback: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|state| {
        let mut state = state.borrow_mut(); // 取得可变对象
        callback(&mut state)
    })
}

/// 需要可变系统状态时 // ! 变更操作一定要记录
#[allow(unused)]
pub fn with_mut_state<F, R>(callback: F, caller: CallerId, topic: RecordTopic, content: String) -> R
where
    F: FnOnce(&mut State) -> (Option<String>, R),
{
    STATE.with(|state| {
        let mut state = state.borrow_mut(); // 取得可变对象
        let record_id = state.record_push(caller, topic, content);
        let (done, result) = callback(&mut state);
        state.record_update(record_id, done.unwrap_or_default());
        result
    })
}

*/
