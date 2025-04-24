use ic_cdk::bitcoin_canister::{
    GetBalanceRequest, GetCurrentFeePercentilesRequest, GetUtxosRequest, SendTransactionRequest,
};

pub use ic_cdk::bitcoin_canister::{
    Address as BitcoinAddress, BlockHash, GetUtxosResponse, MillisatoshiPerByte,
    Network as BitcoinNetwork, Satoshi, Utxo, UtxosFilter,
};

use crate::types::CanisterCallError;

/// 查询余额
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
pub async fn bitcoin_get_balance(
    network: BitcoinNetwork,
    address: BitcoinAddress,
    min_confirmations: Option<u32>,
) -> super::types::CanisterCallResult<Satoshi> {
    ic_cdk::bitcoin_canister::bitcoin_get_balance(&GetBalanceRequest {
        network,
        address,
        min_confirmations,
    })
    .await
    .map_err(|err| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#bitcoin_get_balance".to_string(),
        message: err.to_string(),
    })
}

/// 查询网络费用
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_current_fee_percentiles
pub async fn bitcoin_get_current_fee_percentiles(
    network: BitcoinNetwork,
) -> super::types::CanisterCallResult<Vec<MillisatoshiPerByte>> {
    ic_cdk::bitcoin_canister::bitcoin_get_current_fee_percentiles(
        &GetCurrentFeePercentilesRequest { network },
    )
    .await
    .map_err(|err| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#bitcoin_get_current_fee_percentiles".to_string(),
        message: err.to_string(),
    })
}

/// 查询 UTXO
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
pub async fn bitcoin_get_utxos(
    network: BitcoinNetwork,
    address: BitcoinAddress,
    filter: Option<UtxosFilter>,
) -> super::types::CanisterCallResult<GetUtxosResponse> {
    ic_cdk::bitcoin_canister::bitcoin_get_utxos(&GetUtxosRequest {
        address,
        network,
        filter,
    })
    .await
    .map_err(|err| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#bitcoin_get_utxos".to_string(),
        message: err.to_string(),
    })
}

/// 发送交易
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
pub async fn bitcoin_send_transaction(
    network: BitcoinNetwork,
    transaction: Vec<u8>,
) -> super::types::CanisterCallResult<()> {
    ic_cdk::bitcoin_canister::bitcoin_send_transaction(&SendTransactionRequest {
        transaction,
        network,
    })
    .await
    .map_err(|err| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#bitcoin_send_transaction".to_string(),
        message: err.to_string(),
    })
}
