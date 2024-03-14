use candid::CandidType;
use ic_canister_kit::types::Permission;

mod common;
pub use common::*;

mod business;
pub use business::*;

// 本罐子需要的权限转换
pub trait ParsePermission {
    fn parse_permission<'a>(&self, name: &'a str) -> Result<Permission, ParsePermissionError<'a>>;
}
#[derive(Debug)]
pub struct ParsePermissionError<'a>(&'a str);
impl Display for ParsePermissionError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParsePermissionError: {}", self.0)
    }
}
impl std::error::Error for ParsePermissionError<'_> {}

// ==================== 更新版本需要修改下面代码 ====================

mod v000;
mod v001;

use serde::Deserialize;
// ! 此处应该是最新的版本
// *     👇👇 UPGRADE WARNING: 必须是当前代码的版本
pub use v001::types::*;

#[derive(candid::CandidType, serde::Deserialize, Debug)]
pub enum State {
    V0(v000::types::InnerState),
    V1(v001::types::InnerState),
    // * 👆👆 UPGRADE WARNING: 引入新版本
}

use State::*;

// 升级版本
impl Upgrade for State {
    fn upgrade(&mut self) {
        loop {
            // ! 此处应该是最新的版本
            // *             👇👇 UPGRADE WARNING: 必须是当前代码的版本
            if matches!(self, V1(_)) {
                break; // !  👆👆 UPGRADE WARNING: 升级版本一定要注意修改
            }
            // 进行升级操作, 不断地升到下一版本
            match self {
                V0(s) => *self = V1(std::mem::take(s).into()), // -> V1
                V1(_) => break,                                // do nothing
            }
        }
    }

    fn version(&self) -> u32 {
        match self {
            V0(_) => 0, // ? 版本号
            V1(_) => 1, // ? 版本号
        }
    }
}

impl State {
    pub fn get(&self) -> &dyn Business {
        match self {
            V0(s) => s, // * 获取不可变对象
            V1(s) => s, // * 获取不可变对象
        }
    }
    pub fn get_mut(&mut self) -> &mut dyn Business {
        match self {
            V0(s) => s, // * 获取可变对象
            V1(s) => s, // * 获取可变对象
        }
    }
}

// ==================== 初始化 ====================

// 罐子初始化需要的参数
#[derive(Debug, Deserialize, CandidType)]
pub struct CanisterInitialArg {
    schedule: Option<DurationNanos>,
}
impl CanisterInitialArg {
    pub fn none() -> Self {
        CanisterInitialArg { schedule: None }
    }
}

// 初始化
impl Initial<CanisterInitialArg> for State {
    fn init(&mut self, arg: CanisterInitialArg) {
        self.upgrade(); // 再判断升级一次也没关系
        match self {
            V0(s) => s.init(arg), // * 初始化
            V1(s) => s.init(arg), // * 初始化
        }
    }
}
