use ethereum::{Block, LegacyTransaction};
use ethereum_types::H256;
use scale_decode::ext::primitive_types::U256;

use crate::{
    decoder::metadata::AnyRuntimeMetadata,
    error::Error,
    fetch::fetch,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion},
    },
};

pub const PALLET_NAME: &str = "Ethereum";

pub const STORAGE_TYPES: [(&str, (&[&str], &str)); 2] = [
    (
        "BlockHash",
        (&["primitive_types::U256"], "primitive_types::H256"),
    ),
    (
        "CurrentBlock",
        (
            &[],
            "ethereum::block::Block<ethereum::transaction::LegacyTransaction>",
        ),
    ),
];

pub async fn fetch_block_hash(
    rpc: &NodeRPC,
    block_number: u32,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<Vec<u8>, Error> {
    let block_number = U256::from(block_number).0;
    let H256(bytes) = fetch(
        PALLET_NAME,
        "BlockHash",
        [block_number],
        rpc,
        block_hash,
        metadata,
        runtime_version,
    )
    .await?;
    Ok(Vec::from(bytes))
}

pub async fn fetch_block(
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<Block<LegacyTransaction>, Error> {
    fetch(
        PALLET_NAME,
        "CurrentBlock",
        (),
        rpc,
        block_hash,
        metadata,
        runtime_version,
    )
    .await
}
