use candid::CandidType;
use serde::Deserialize;

use crate::results::MotokoResult;

use super::super::types::NftStorage;
use super::types::{
    ExtBalanceArgs, ExtBalanceResult, ExtTransferArgs, ExtTransferResult, StableTransferArgs,
};

use super::core::ExtCore;

#[derive(CandidType, Deserialize)]
pub enum ExtBatchError {
    Error(String),
}

pub type ExtBalanceBatchArgs = Vec<ExtBalanceArgs>;
pub type ExtTransferBatchArgs = Vec<ExtTransferArgs>;

pub type ExtBalanceBatchResult = MotokoResult<Vec<ExtBalanceResult>, ExtBatchError>;
pub type ExtTransferBatchResult = MotokoResult<Vec<ExtTransferResult>, ExtBatchError>;

// ================ 接口 =================

pub trait ExtBatch {
    fn balance_batch(&self, args: ExtBalanceBatchArgs) -> ExtBalanceBatchResult;
    fn transfer_batch(
        &mut self,
        args: ExtTransferBatchArgs,
    ) -> MotokoResult<Vec<(ExtTransferResult, Option<StableTransferArgs>)>, ExtBatchError>;
}

impl ExtBatch for NftStorage {
    fn balance_batch(&self, args: ExtBalanceBatchArgs) -> ExtBalanceBatchResult {
        let mut results = Vec::new();
        for arg in args {
            results.push(self.balance(arg));
        }
        MotokoResult::Ok(results)
    }

    fn transfer_batch(
        &mut self,
        args: ExtTransferBatchArgs,
    ) -> MotokoResult<Vec<(ExtTransferResult, Option<StableTransferArgs>)>, ExtBatchError> {
        let mut results = Vec::new();
        for arg in args {
            results.push(self.transfer(arg));
        }
        MotokoResult::Ok(results)
    }
}
