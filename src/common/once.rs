use std::{borrow::BorrowMut, cell::RefCell};

thread_local! {
    static CALL_ONCE: RefCell<bool> = const { RefCell::new(false) };
}

/// 拦截对象
pub struct CallOnceGuard;

impl Drop for CallOnceGuard {
    fn drop(&mut self) {
        CALL_ONCE.with_borrow_mut(|o| *o.borrow_mut() = false)
    }
}

/// 调用一次
#[inline]
pub fn call_once_guard() -> CallOnceGuard {
    if CALL_ONCE.with(|o| *o.borrow()) {
        ic_cdk::trap("Too many request."); // ! 中止执行
    }

    CALL_ONCE.with_borrow_mut(|o| *o.borrow_mut() = true);

    CallOnceGuard
}
