use std::fmt::Display;

use candid::CandidType;
use serde::{Deserialize, Serialize};

// ================== 罐子调用产生的错误信息 ==================

/// 罐子调用会产生的错误
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CanisterCallError {
    /// 罐子 id
    pub canister_id: crate::identity::CanisterId,

    /// 调用的方法
    pub method: String,

    /// 错误消息
    pub message: String,
}
impl std::fmt::Display for CanisterCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Canister({}) call({}) failed: {}",
            self.canister_id.to_text(),
            self.method,
            self.message
        )
    }
}
impl std::error::Error for CanisterCallError {}

impl CanisterCallError {
    /// 新建
    pub fn new<E: Display>(canister_id: crate::identity::CanisterId, method: impl Into<String>, err: E) -> Self {
        Self {
            canister_id,
            method: method.into(),
            message: err.to_string(),
        }
    }

    /// 管理调用
    pub fn from<E: Display>(method: impl Into<String>, err: E) -> Self {
        Self {
            canister_id: crate::identity::CanisterId::management_canister(),
            method: method.into(),
            message: err.to_string(),
        }
    }
}

/// 罐子调用结果
pub type CanisterCallResult<T> = Result<T, CanisterCallError>;

// ================== Canister cycles 指标 ==================

/// `canister_metrics` 管理接口的参数。
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CanisterMetricsArgs {
    /// 要查询的 Canister。
    pub canister_id: crate::identity::CanisterId,
}

/// Canister 自创建以来按用途累计消耗的 cycles；旧 Canister 从该指标启用时开始累计。
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CyclesConsumed {
    /// 为内存资源累计消耗的 cycles。
    pub memory: candid::Nat,
    /// 为计算资源分配累计消耗的 cycles。
    pub compute_allocation: candid::Nat,
    /// 为 ingress 消息接纳累计消耗的 cycles。
    pub ingress_induction: candid::Nat,
    /// 为执行指令累计消耗的 cycles。
    pub instructions: candid::Nat,
    /// 为请求和响应传输累计消耗的 cycles。
    pub request_and_response_transmission: candid::Nat,
    /// 因 cycles 耗尽而卸载 Canister 累计消耗的 cycles。
    pub uninstall: candid::Nat,
    /// 创建 Canister 累计消耗的 cycles。
    pub canister_creation: candid::Nat,
    /// 为 HTTP outcall 累计消耗的 cycles。
    pub http_outcalls: candid::Nat,
    /// Canister 通过 cycles burn API 主动销毁的 cycles。
    pub burned_cycles: candid::Nat,
}

/// `canister_metrics` 管理接口的返回值。
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CanisterMetricsResult {
    /// 按用途划分的累计 cycles 消耗。
    pub cycles_consumed: CyclesConsumed,
}

// ===================== 常用模块 =====================

pub use super::{
    chunks::CanisterCodeChunk,
    codes::{CanisterCodeHash, CanisterCodeWasm, CanisterInitArg},
};
pub use ic_cdk_management_canister::*;

#[cfg(test)]
mod tests {
    use super::{CanisterMetricsArgs, CanisterMetricsResult, CyclesConsumed};
    use candid::{CandidType, Deserialize, Nat, Principal};

    #[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
    struct ManagementCanisterMetricsArgs {
        canister_id: Principal,
    }

    #[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
    struct ManagementCyclesConsumed {
        memory: Nat,
        compute_allocation: Nat,
        ingress_induction: Nat,
        instructions: Nat,
        request_and_response_transmission: Nat,
        uninstall: Nat,
        canister_creation: Nat,
        http_outcalls: Nat,
        burned_cycles: Nat,
    }

    #[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
    struct ManagementCanisterMetricsResult {
        cycles_consumed: ManagementCyclesConsumed,
    }

    #[test]
    fn canister_metrics_types_match_management_candid_interface() {
        let canister_id = Principal::anonymous();
        let encoded_args =
            candid::encode_one(CanisterMetricsArgs { canister_id }).expect("canister_metrics args should be encodable");
        let decoded_args: ManagementCanisterMetricsArgs =
            candid::decode_one(&encoded_args).expect("canister_metrics args should match the management interface");
        assert_eq!(decoded_args.canister_id, canister_id);

        let encoded_result = candid::encode_one(CanisterMetricsResult {
            cycles_consumed: CyclesConsumed {
                memory: Nat::from(1_u8),
                compute_allocation: Nat::from(2_u8),
                ingress_induction: Nat::from(3_u8),
                instructions: Nat::from(4_u8),
                request_and_response_transmission: Nat::from(5_u8),
                uninstall: Nat::from(6_u8),
                canister_creation: Nat::from(7_u8),
                http_outcalls: Nat::from(8_u8),
                burned_cycles: Nat::from(9_u8),
            },
        })
        .expect("canister_metrics result should be encodable");
        let decoded_result: ManagementCanisterMetricsResult =
            candid::decode_one(&encoded_result).expect("canister_metrics result should match the management interface");

        assert_eq!(decoded_result.cycles_consumed.memory, Nat::from(1_u8));
        assert_eq!(decoded_result.cycles_consumed.burned_cycles, Nat::from(9_u8));
    }
}
