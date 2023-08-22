use std::{borrow::BorrowMut, cell::RefMut};

#[cfg(feature = "stable_initial")]
pub mod initial;

#[cfg(feature = "stable_permissions")]
pub mod permissions;

#[cfg(feature = "stable_logs")]
pub mod logs;

pub mod types;

// 持久化相关接口

// 可保存和恢复
pub trait Stable<S, R>
where
    S: candid::utils::ArgumentEncoder,
    R: for<'de> candid::utils::ArgumentDecoder<'de>,
{
    fn store(&mut self) -> S;
    fn restore(&mut self, restore: R);
}

// 升级前保存
pub fn pre_upgrade<S, R>(stable: &mut RefMut<dyn Stable<S, R>>)
where
    S: candid::utils::ArgumentEncoder,
    R: for<'de> candid::utils::ArgumentDecoder<'de>,
{
    let s = stable.borrow_mut().store();
    ic_cdk::storage::stable_save(s).unwrap();
}

// 升级后恢复
pub fn post_upgrade<S, R>(stable: &mut RefMut<dyn Stable<S, R>>)
where
    S: candid::utils::ArgumentEncoder,
    R: for<'de> candid::utils::ArgumentDecoder<'de>,
{
    let d: R = ic_cdk::storage::stable_restore().unwrap();
    stable.restore(d);
}
