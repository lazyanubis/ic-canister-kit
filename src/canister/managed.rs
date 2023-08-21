use ic_cdk::api::management_canister::main::CanisterSettings;

use crate::identity::CanisterId;

use super::{
    codes::{CanisterCodeWasm, CanisterInitArg},
    cycles::call_wallet_receive,
    deploy::{CanisterInfo, CanisterInfoShow},
    status::{call_canister_status, CanisterStatusResult},
};

// ========== 接口所需 ==========

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub enum ManagedCanisterRefreshResult {
    Refreshed(Vec<CanisterInfoShow>),
    NewCanister(CanisterInfoShow),
    DeployFailed(String),
}

// ================= 管理多个罐子 =================

// 记录所管理的 Canister 的状态，选取有用的时候，倒序遍历
pub type ManagedCanisterStates = Vec<CanisterInfo>;

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct ManagedCanisterInitial {
    pub max_canister_count: u32,        // 最大数量限制，0 表示不限制最大数量
    pub max_canister_memory_size: u128, // 最大内存使用，超过这个限制，表示不能再加入新的数据了
    pub initial_canister_cycles: u128,  // 初始分配的 cycles 数量
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct ManagedCanisterMaintained {
    pub min_cycles: u128,     // cycles 低于此值，就要充值
    pub min_days: u16,        // 估算时间低于此值，就要充值
    pub recharge_cycles: u64, // 每次充值数量
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct ManagedCanisterChecking {
    pub check_number: u32, // 每次最大检查数量，0 表示不限制
    pub next: u32,         // 下一个要核查的 canister 序号
}

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct ManagedCanisterConfig {
    pub initial: ManagedCanisterInitial,       // 初始化需要的配置信息
    pub maintained: ManagedCanisterMaintained, // 维护单个罐子需要的信息
    pub checking: ManagedCanisterChecking,     // 每次检查需要的信息
}

impl ManagedCanisterConfig {
    // 限制刷新数量情况下获取最新数据
    pub async fn refresh_status(
        &self,
        canisters: &Vec<CanisterId>,
    ) -> (u32, Vec<(usize, CanisterStatusResult)>) {
        let length = canisters.len() as u32;
        let mut check_number = self.checking.check_number;
        if check_number == 0 || check_number > length {
            check_number = length;
        }
        let mut next = self.checking.next;

        if length == 0 {
            return (next, Vec::new());
        }

        let mut status_list = Vec::new();

        for _ in 0..check_number {
            if next >= length {
                next = 0;
            }
            let canister_id = canisters[next as usize];
            let result = call_canister_status(&canister_id).await;

            self.check_cycles(&canister_id, &result).await;

            status_list.push((next as usize, result));

            next += 1;
        }

        (next, status_list)
    }

    // 检查 cycles 是否满足条件
    async fn check_cycles(&self, canister_id: &CanisterId, result: &CanisterStatusResult) {
        let maintained = &self.maintained;
        if maintained.min_cycles < result.cycles
            && candid::Nat::from(maintained.min_days)
                < result.cycles.clone() / result.idle_cycles_burned_per_day.clone()
        {
            return;
        }
        // 下面要进行充值 cycles
        call_wallet_receive(canister_id, maintained.recharge_cycles).await;
    }

    pub fn available(&self, memory_size: &candid::Nat) -> bool {
        memory_size < &self.initial.max_canister_memory_size
    }

    pub async fn deploy_canister(
        &self,
        count: u32,
        settings: Option<CanisterSettings>,
        wasm_module: Option<CanisterCodeWasm>,
        arg: Option<CanisterInitArg>,
    ) -> Result<CanisterInfo, String> {
        if 0 < self.initial.max_canister_count && self.initial.max_canister_count <= count {
            return Result::Err(format!(
                "max canister count: {}",
                self.initial.max_canister_count
            ));
        }
        let initial_cycles = self.initial.initial_canister_cycles;
        super::deploy::deploy_canister(settings, initial_cycles, wasm_module, arg).await
    }
}
