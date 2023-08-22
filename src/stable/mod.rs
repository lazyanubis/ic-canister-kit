use std::{borrow::BorrowMut, cell::RefMut};

// 持久化相关接口

// 可保存
pub trait Storable<S>
where
    S: candid::utils::ArgumentEncoder,
{
    fn store(&mut self) -> S;
}
// 可恢复
pub trait Restorable<R>
where
    R: for<'de> candid::utils::ArgumentDecoder<'de>,
{
    fn restore(&mut self, restore: R);
}
// 可保存和恢复
pub trait Stable<S, R>: Storable<S> + Restorable<R>
where
    S: candid::utils::ArgumentEncoder,
    R: for<'de> candid::utils::ArgumentDecoder<'de>,
{
}

// 升级前保存
pub fn pre_upgrade<S>(stable: &mut RefMut<dyn Storable<S>>)
where
    S: candid::utils::ArgumentEncoder,
{
    let s = stable.borrow_mut().store();
    ic_cdk::storage::stable_save(s).unwrap();
}

// 升级后恢复
pub fn post_upgrade<R>(stable: &mut RefMut<dyn Restorable<R>>)
where
    R: for<'de> candid::utils::ArgumentDecoder<'de>,
{
    let d: R = ic_cdk::storage::stable_restore().unwrap();
    stable.restore(d);
}
