//! Canister Wasm 分块存储管理。

use super::types::{ClearChunkStoreArgs, StoredChunksArgs, UploadChunkArgs};
use crate::identity::CanisterId;

pub use super::types::ChunkHash;

/// 管理 Canister 允许上传的单个 Wasm 分块最大字节数（1 MiB）。
pub const MAX_CHUNK_SIZE_IN_BYTES: usize = 1024 * 1024;

/// 待上传的 Canister Wasm 分块。
pub type CanisterCodeChunk = Vec<u8>;

/// 向指定 Canister 的 chunk store 上传一个 Wasm 分块，并返回该分块的 SHA-256 hash。
///
/// 分块不得超过 [`MAX_CHUNK_SIZE_IN_BYTES`]。调用者必须是目标 Canister 自身或其 controller。
/// <https://docs.internetcomputer.org/references/management-canister/#upload_chunk>
pub async fn upload_chunk(
    canister_id: CanisterId,
    chunk: CanisterCodeChunk,
) -> super::types::CanisterCallResult<ChunkHash> {
    ic_cdk_management_canister::upload_chunk(&UploadChunkArgs { canister_id, chunk })
        .await
        .map_err(|err| super::types::CanisterCallError::new(canister_id, "ic#upload_chunk", err))
}

/// 清空指定 Canister 的 chunk store。
///
/// 调用者必须是目标 Canister 自身或其 controller。
/// <https://docs.internetcomputer.org/references/management-canister/#clear_chunk_store>
pub async fn clear_chunk_store(canister_id: CanisterId) -> super::types::CanisterCallResult<()> {
    let call_result = ic_cdk_management_canister::clear_chunk_store(&ClearChunkStoreArgs { canister_id }).await;
    super::wrap_call_result(canister_id, "ic#clear_chunk_store", call_result)
}

/// 查询指定 Canister 的 chunk store 中已存储的所有分块 hash。
///
/// 调用者必须是目标 Canister 自身或其 controller。返回顺序不代表 Wasm 的拼接顺序。
/// <https://docs.internetcomputer.org/references/management-canister/#stored_chunks>
pub async fn stored_chunks(canister_id: CanisterId) -> super::types::CanisterCallResult<Vec<ChunkHash>> {
    ic_cdk_management_canister::stored_chunks(&StoredChunksArgs { canister_id })
        .await
        .map_err(|err| super::types::CanisterCallError::new(canister_id, "ic#stored_chunks", err))
}
