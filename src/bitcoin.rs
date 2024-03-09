use ic_cdk::api::management_canister::bitcoin::{
    GetBalanceRequest, GetCurrentFeePercentilesRequest, GetUtxosRequest, SendTransactionRequest,
};

pub use ic_cdk::api::management_canister::bitcoin::{
    BitcoinAddress, BitcoinNetwork, BlockHash, GetUtxosResponse, MillisatoshiPerByte, Satoshi,
    Utxo, UtxoFilter,
};

use crate::{canister::fetch_tuple0, types::CanisterCallError};

/// 查询余额
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
pub async fn bitcoin_get_balance(
    network: BitcoinNetwork,
    address: BitcoinAddress,
    min_confirmations: Option<u32>,
) -> super::types::CanisterCallResult<Satoshi> {
    ic_cdk::api::management_canister::bitcoin::bitcoin_get_balance(GetBalanceRequest {
        network,
        address,
        min_confirmations,
    })
    .await
    .map(fetch_tuple0)
    .map_err(|(rejection_code, message)| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#bitcoin_get_balance".to_string(),
        rejection_code,
        message,
    })
}

/// 查询网络费用
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_current_fee_percentiles
pub async fn bitcoin_get_current_fee_percentiles(
    network: BitcoinNetwork,
) -> super::types::CanisterCallResult<Vec<MillisatoshiPerByte>> {
    ic_cdk::api::management_canister::bitcoin::bitcoin_get_current_fee_percentiles(
        GetCurrentFeePercentilesRequest { network },
    )
    .await
    .map(fetch_tuple0)
    .map_err(|(rejection_code, message)| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#bitcoin_get_current_fee_percentiles".to_string(),
        rejection_code,
        message,
    })
}

/// 查询 UTXO
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
pub async fn bitcoin_get_utxos(
    network: BitcoinNetwork,
    address: BitcoinAddress,
    filter: Option<UtxoFilter>,
) -> super::types::CanisterCallResult<GetUtxosResponse> {
    ic_cdk::api::management_canister::bitcoin::bitcoin_get_utxos(GetUtxosRequest {
        address,
        network,
        filter,
    })
    .await
    .map(fetch_tuple0)
    .map_err(|(rejection_code, message)| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#bitcoin_get_utxos".to_string(),
        rejection_code,
        message,
    })
}

/// 发送交易
/// https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
pub async fn bitcoin_send_transaction(
    network: BitcoinNetwork,
    transaction: Vec<u8>,
) -> super::types::CanisterCallResult<()> {
    ic_cdk::api::management_canister::bitcoin::bitcoin_send_transaction(SendTransactionRequest {
        transaction,
        network,
    })
    .await
    .map_err(|(rejection_code, message)| CanisterCallError {
        canister_id: crate::identity::CanisterId::anonymous(),
        method: "ic#bitcoin_send_transaction".to_string(),
        rejection_code,
        message,
    })
}
