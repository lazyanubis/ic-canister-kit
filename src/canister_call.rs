use super::identity::CanisterId;

use super::canister_status::{canister_status, CanisterStatusResult};

use super::cycles::WalletReceiveResult;

pub type CanisterIdRecord = ic_cdk::api::management_canister::main::CanisterIdRecord;
pub type CanisterInstallMode = ic_cdk::api::management_canister::main::CanisterInstallMode;
pub type CreateCanisterArgument = ic_cdk::api::management_canister::main::CreateCanisterArgument;
pub type InstallCodeArgument = ic_cdk::api::management_canister::main::InstallCodeArgument;
pub type UpdateSettingsArgument = ic_cdk::api::management_canister::main::UpdateSettingsArgument;
pub type CanisterSettings = ic_cdk::api::management_canister::main::CanisterSettings;

pub type CallError = (ic_cdk::api::call::RejectionCode, std::string::String);

pub type CanisterWasm = Vec<u8>; // 部署罐子代码
pub type CanisterInitArg = Vec<u8>; // 部署罐子代码初始化参数

#[derive(candid::CandidType, candid::Deserialize, Debug, Clone)]
pub struct CanisterInfo {
    pub canister_id: CanisterId,
    pub status: CanisterStatusResult,
}

impl CanisterInfo {
    pub fn new(canister_id: &str) -> CanisterInfo {
        CanisterInfo {
            canister_id: CanisterId::from_text(canister_id).unwrap(),
            status: CanisterStatusResult {
                status: super::canister_status::CanisterStatus::Running,
                settings: super::canister_status::DefiniteCAnisterSettings {
                    controllers: vec![],
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

// 错误处理

pub fn unwrap_call_result<R: std::fmt::Debug>(
    canister_id: &CanisterId,
    method: &str,
    call_result: Result<(R,), CallError>,
) -> R {
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        panic!(
            "canister: {} call: {} failed: {:?} {}",
            canister_id.to_text(),
            method,
            err.0,
            err.1
        );
    }

    call_result.unwrap().0
}

pub fn unwrap_call_result_with_error<R: std::fmt::Debug>(
    canister_id: &CanisterId,
    method: &str,
    call_result: Result<(R,), CallError>,
) -> Result<R, String> {
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} call: {} failed: {:?} {}",
            canister_id.to_text(),
            method,
            err.0,
            err.1
        ));
    }

    Result::Ok(call_result.unwrap().0)
}

// 部署罐子
pub async fn deploy_canister(
    settings: Option<CanisterSettings>,
    initial_cycles: u128,
    wasm: Option<CanisterWasm>,
    arg: Option<CanisterInitArg>,
) -> Result<CanisterInfo, String> {
    if wasm.is_none() {
        return Result::Err(format!("canister code can not be none"));
    }
    let wasm_module = wasm.unwrap();

    // 1. 创建一个新的罐子
    let record = create_canister(settings, initial_cycles).await?;
    ic_cdk::println!("new canister id: {:?}", record.canister_id.to_text());

    // 2. 安装代码
    install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id: record.canister_id.clone(),
        wasm_module,
        arg: arg.unwrap_or(vec![]),
    })
    .await?;

    // 3. 启动罐子
    start_canister(record.clone()).await?;

    // 4. 查询当前状态
    let result: CanisterStatusResult = canister_status(record).await;

    Result::Ok(CanisterInfo {
        canister_id: record.canister_id,
        status: result,
    })
}

// 包装方法

pub async fn create_canister(
    settings: Option<CanisterSettings>,
    cycles: u128,
) -> Result<CanisterIdRecord, String> {
    let result = ic_cdk::api::management_canister::main::create_canister(
        CreateCanisterArgument {
            settings: settings.clone(),
        },
        cycles,
    )
    .await;
    if result.is_err() {
        let err = result.unwrap_err();
        return Result::Err(format!(
            "create canister {:?} {} failed: {:?} {}",
            settings, cycles, err.0, err.1
        ));
    }
    Ok(result.unwrap().0)
}

pub async fn update_settings(arg: UpdateSettingsArgument) -> Result<(), String> {
    let call_result = ic_cdk::api::management_canister::main::update_settings(arg.clone()).await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} update_settings: {:?} failed: {:?} {}",
            arg.canister_id.to_text(),
            arg.settings,
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

pub async fn install_code(arg: InstallCodeArgument) -> Result<(), String> {
    let call_result = ic_cdk::api::management_canister::main::install_code(arg.clone()).await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} install_code: {:?} failed: {:?} {}",
            arg.canister_id.to_text(),
            arg.mode,
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

pub async fn uninstall_code(arg: CanisterIdRecord) -> Result<(), String> {
    let call_result = ic_cdk::api::management_canister::main::uninstall_code(arg.clone()).await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} uninstall_code failed: {:?} {}",
            arg.canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

pub async fn start_canister(arg: CanisterIdRecord) -> Result<(), String> {
    let call_result = ic_cdk::api::management_canister::main::start_canister(arg.clone()).await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} start_canister failed: {:?} {}",
            arg.canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

pub async fn stop_canister(arg: CanisterIdRecord) -> Result<(), String> {
    let call_result = ic_cdk::api::management_canister::main::stop_canister(arg.clone()).await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} stop_canister failed: {:?} {}",
            arg.canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

pub async fn delete_canister(arg: CanisterIdRecord) -> Result<(), String> {
    let call_result = ic_cdk::api::management_canister::main::delete_canister(arg.clone()).await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} delete_canister failed: {:?} {}",
            arg.canister_id.to_text(),
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

pub async fn deposit_cycles(arg: CanisterIdRecord, cycles: u128) -> Result<(), String> {
    let call_result =
        ic_cdk::api::management_canister::main::deposit_cycles(arg.clone(), cycles).await;
    if call_result.is_err() {
        let err = call_result.unwrap_err();
        return Result::Err(format!(
            "canister: {} deposit_cycles {} failed: {:?} {}",
            arg.canister_id.to_text(),
            cycles,
            err.0,
            err.1
        ));
    }
    Result::Ok(())
}

pub async fn do_canister_upgrade(
    canister_id: CanisterId,
    wasm: CanisterWasm,
    arg: Option<CanisterInitArg>,
) -> Result<(), String> {
    install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade,
        canister_id,
        wasm_module: wasm,
        arg: arg.unwrap_or(vec![]),
    })
    .await
}

pub async fn do_canister_reinstall(
    canister_id: CanisterId,
    wasm: CanisterWasm,
    arg: Option<CanisterInitArg>,
) -> Result<(), String> {
    install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Reinstall,
        canister_id,
        wasm_module: wasm,
        arg: arg.unwrap_or(vec![]),
    })
    .await
}

pub async fn call_wallet_balance(canister_id: &CanisterId) -> candid::Nat {
    let call_result: Result<(candid::Nat,), CallError> =
        ic_cdk::api::call::call(canister_id.clone(), "wallet_balance", ()).await;
    unwrap_call_result(canister_id, "wallet_balance", call_result)
}

pub async fn call_wallet_receive(canister_id: &CanisterId, cycles: u64) -> WalletReceiveResult {
    let call_result: Result<(WalletReceiveResult,), CallError> =
        ic_cdk::api::call::call_with_payment(canister_id.clone(), "wallet_receive", (), cycles)
            .await;
    unwrap_call_result(canister_id, "wallet_receive", call_result)
}

pub async fn call_canister_status(canister_id: &CanisterId) -> CanisterStatusResult {
    let call_result: Result<(CanisterStatusResult,), CallError> =
        ic_cdk::call(canister_id.clone(), "canister_status", ()).await;
    unwrap_call_result(canister_id, "canister_status", call_result)
}

pub async fn call_canister<
    T: candid::utils::ArgumentEncoder,
    R: for<'a> candid::utils::ArgumentDecoder<'a>,
>(
    canister_id: CanisterId,
    method: &str,
    args: T,
) -> Result<R, CallError> {
    ic_cdk::call(canister_id.clone(), &method, args).await
}
