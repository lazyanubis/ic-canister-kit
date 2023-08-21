use ic_cdk::api::management_canister::main::{
    CanisterSettings, CanisterStatusType, DefiniteCanisterSettings,
};

use super::codes::install_code;
use super::codes::{CanisterCodeWasm, CanisterInitArg};
use super::create::create_canister;
use super::status::{canister_status, start_canister, CanisterStatusResult};
use crate::identity::{CanisterId, UserId};

/// 和 罐子 的 部署 相关

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct CanisterInfo {
    pub canister_id: CanisterId,
    pub status: CanisterStatusResult,
}

impl CanisterInfo {
    pub fn new(canister_id: &str, controllers: Vec<UserId>) -> CanisterInfo {
        CanisterInfo {
            canister_id: CanisterId::from_text(canister_id).unwrap(),
            status: CanisterStatusResult {
                status: CanisterStatusType::Running,
                settings: DefiniteCanisterSettings {
                    controllers,
                    compute_allocation: candid::Nat::from(0),
                    memory_allocation: candid::Nat::from(0),
                    freezing_threshold: candid::Nat::from(0),
                },
                module_hash: None,
                memory_size: candid::Nat::from(0),
                cycles: candid::Nat::from(1000000) * 2000000,
                idle_cycles_burned_per_day: candid::Nat::from(0),
            },
        }
    }
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct CanisterInfoShow {
    pub canister_id: String,
    pub status: CanisterStatusResult,
}

impl From<CanisterInfo> for CanisterInfoShow {
    fn from(value: CanisterInfo) -> Self {
        CanisterInfoShow {
            canister_id: value.canister_id.to_text(),
            status: value.status,
        }
    }
}

// 部署罐子
pub async fn deploy_canister(
    settings: Option<CanisterSettings>,
    initial_cycles: u128,
    wasm_module: Option<CanisterCodeWasm>,
    arg: Option<CanisterInitArg>,
) -> Result<CanisterInfo, String> {
    if wasm_module.is_none() {
        return Result::Err(format!("canister code can not be none"));
    }
    let wasm_module = wasm_module.unwrap();

    // 1. 创建一个新的罐子
    let canister_id = create_canister(settings, initial_cycles).await?;
    ic_cdk::println!("new canister id: {:?}", canister_id.to_text());

    // 2. 安装代码
    install_code(canister_id.clone(), wasm_module, arg.unwrap_or(vec![])).await?;

    // 3. 启动罐子
    start_canister(canister_id.clone()).await?;

    // 4. 查询当前状态
    let result: CanisterStatusResult = canister_status(canister_id).await;

    Result::Ok(CanisterInfo {
        canister_id,
        status: result,
    })
}
