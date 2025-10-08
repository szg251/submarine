use jsonrpsee::{
    core::{DeserializeOwned, client::ClientT, traits::ToRpcParams},
    rpc_params,
    ws_client::{WsClient, WsClientBuilder},
};

use crate::node_rpc::models::{SyncState, SystemProperties};

pub use super::error::NodeRPCError;
use super::models::{
    BlockHashHex, BlockHeader, BlockNumberHex, ChainMetadataBytes, RuntimeVersion, SignedBlock,
    StorageKeyHex, StorageValueBytes,
};

pub struct NodeRPC {
    client: WsClient,
}

#[allow(dead_code)]
impl NodeRPC {
    pub async fn new(url: &str) -> Result<Self, NodeRPCError> {
        let client = WsClientBuilder::default()
            .build(url)
            .await
            .map_err(NodeRPCError::FailedConnection)?;

        Ok(Self { client })
    }

    async fn request<P, R>(&self, method: &str, params: P) -> Result<R, NodeRPCError>
    where
        P: ToRpcParams + Send,
        R: DeserializeOwned,
    {
        self.client
            .request(method, params)
            .await
            .map_err(|source| NodeRPCError::RequestFailed {
                method: method.to_string(),
                source,
            })
    }

    pub async fn system_chain(&self) -> Result<String, NodeRPCError> {
        self.request("system_chain", rpc_params![]).await
    }

    pub async fn system_name(&self) -> Result<String, NodeRPCError> {
        self.request("system_name", rpc_params![]).await
    }

    pub async fn system_version(&self) -> Result<String, NodeRPCError> {
        self.request("system_version", rpc_params![]).await
    }

    pub async fn system_sync_state(&self) -> Result<SyncState, NodeRPCError> {
        self.request("system_syncState", rpc_params![]).await
    }

    pub async fn system_properties(&self) -> Result<SystemProperties, NodeRPCError> {
        self.request("system_properties", rpc_params![]).await
    }

    pub async fn chain_get_header(
        &self,
        header_hash: BlockHashHex,
    ) -> Result<BlockHeader, NodeRPCError> {
        self.request("chain_getHeader", rpc_params![header_hash])
            .await
    }

    pub async fn chain_get_finalized_head(&self) -> Result<BlockHashHex, NodeRPCError> {
        self.request("chain_getFinalizedHead", rpc_params![]).await
    }

    pub async fn chain_get_block_hash(
        &self,
        block_number: &BlockNumberHex,
    ) -> Result<BlockHashHex, NodeRPCError> {
        self.request("chain_getBlockHash", rpc_params![block_number])
            .await
    }

    pub async fn chain_get_block(
        &self,
        block_hash: &BlockHashHex,
    ) -> Result<SignedBlock, NodeRPCError> {
        self.request("chain_getBlock", rpc_params![block_hash])
            .await
    }

    pub async fn state_get_metadata(
        &self,
        block_hash: &BlockHashHex,
    ) -> Result<ChainMetadataBytes, NodeRPCError> {
        self.request("state_getMetadata", rpc_params![block_hash])
            .await
    }

    pub async fn state_get_runtime_version(
        &self,
        block_hash: &BlockHashHex,
    ) -> Result<RuntimeVersion, NodeRPCError> {
        self.request("state_getRuntimeVersion", rpc_params![block_hash])
            .await
    }

    /// Get all storage keys of a block paginated
    pub async fn state_get_keys_paged(
        &self,
        block_hash: &BlockHashHex,
        per_page: u16,
        last_key: Option<StorageKeyHex>,
    ) -> Result<Vec<StorageKeyHex>, NodeRPCError> {
        self.request(
            "state_getKeysPaged",
            rpc_params!["", per_page, last_key, block_hash],
        )
        .await
    }

    pub async fn state_get_storage(
        &self,
        storage_key: &StorageKeyHex,
        block_hash: &BlockHashHex,
    ) -> Result<Option<StorageValueBytes>, NodeRPCError> {
        self.request("state_getStorage", rpc_params![storage_key, block_hash])
            .await
    }
}
