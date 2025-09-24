use jsonrpsee::{
    core::{DeserializeOwned, client::ClientT, traits::ToRpcParams},
    rpc_params,
    ws_client::{WsClient, WsClientBuilder},
};

pub use super::error::NodeRPCError;
use super::models::{
    BlockHash, BlockHeader, BlockNumber, ChainMetadataBytes, RuntimeVersion, SignedBlock,
    StorageKey, StorageValueBytes,
};

pub struct NodeRPC {
    client: WsClient,
}

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
            .map_err(NodeRPCError::RequestFailed)
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

    pub async fn chain_get_header(
        &self,
        header_hash: BlockHash,
    ) -> Result<BlockHeader, NodeRPCError> {
        self.request("chain_getHeader", rpc_params![header_hash])
            .await
    }

    pub async fn chain_get_last_header(&self) -> Result<BlockHeader, NodeRPCError> {
        self.request("chain_getHeader", rpc_params![]).await
    }

    pub async fn chain_get_finalized_head(&self) -> Result<BlockHash, NodeRPCError> {
        self.request("chain_getFinalizedHead", rpc_params![]).await
    }

    pub async fn chain_get_block_hash(
        &self,
        block_number: &BlockNumber,
    ) -> Result<BlockHash, NodeRPCError> {
        self.request("chain_getBlockHash", rpc_params![block_number])
            .await
    }

    pub async fn chain_get_block(
        &self,
        block_hash: &BlockHash,
    ) -> Result<SignedBlock, NodeRPCError> {
        self.request("chain_getBlock", rpc_params![block_hash])
            .await
    }

    pub async fn state_get_metadata(
        &self,
        block_hash: &BlockHash,
    ) -> Result<ChainMetadataBytes, NodeRPCError> {
        self.request("state_getMetadata", rpc_params![block_hash])
            .await
    }

    pub async fn state_get_runtime_version(
        &self,
        block_hash: &BlockHash,
    ) -> Result<RuntimeVersion, NodeRPCError> {
        self.request("state_getRuntimeVersion", rpc_params![block_hash])
            .await
    }

    /// Get all storage keys of a block paginated
    pub async fn state_get_keys_paged(
        &self,
        block_hash: &BlockHash,
        per_page: u16,
        last_key: Option<StorageKey>,
    ) -> Result<Vec<StorageKey>, NodeRPCError> {
        self.request(
            "state_getKeysPaged",
            rpc_params!["", per_page, last_key, block_hash],
        )
        .await
    }

    pub async fn state_get_storage(
        &self,
        StorageKey(storage_key): &StorageKey,
        BlockHash(block_hash): &BlockHash,
    ) -> Result<StorageValueBytes, NodeRPCError> {
        self.request("state_getStorage", rpc_params![storage_key, block_hash])
            .await
    }
}
