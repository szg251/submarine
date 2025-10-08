use ethereum::{Block, LegacyTransaction, TransactionV2};
use ethereum_types::H256;
use scale_decode::ext::primitive_types::U256;

use crate::{
    decoder::metadata::{AnyRuntimeMetadata, MetadataError},
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
    let storage_entry_name = "BlockHash";
    let type_registry = metadata.type_registry();

    let (key_types, value_type) = metadata
        .pallet_metadata(PALLET_NAME)?
        .storage_entry(storage_entry_name)?
        .types_as_str(type_registry)?;

    if key_types[..] != ["primitive_types::U256"] {
        Err(Error::MetadataError(
            MetadataError::UnexpectedStorageValueType {
                expected: "primitive_types::U256".to_string(),
                got: format!("{key_types:?}"),
                pallet_name: PALLET_NAME.to_string(),
                storage_entry_name: storage_entry_name.to_string(),
            },
        ))?;
    }

    if value_type != "primitive_types::H256" {
        Err(Error::MetadataError(
            MetadataError::UnexpectedStorageValueType {
                expected: "primitive_types::H256".to_string(),
                got: value_type.to_string(),
                pallet_name: PALLET_NAME.to_string(),
                storage_entry_name: storage_entry_name.to_string(),
            },
        ))?;
    }

    let block_number = U256::from(block_number).0;
    let H256(bytes) = fetch(
        PALLET_NAME,
        storage_entry_name,
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
) -> Result<Block<TransactionV2>, Error> {
    let storage_entry_name = "CurrentBlock";
    let type_registry = metadata.type_registry();

    let (_key_types, value_type) = metadata
        .pallet_metadata(PALLET_NAME)?
        .storage_entry(storage_entry_name)?
        .types_as_str(type_registry)?;

    match &value_type[..] {
        "ethereum::block::Block<ethereum::transaction::LegacyTransaction>" => {
            let legacy_block: Block<LegacyTransaction> = fetch(
                PALLET_NAME,
                storage_entry_name,
                (),
                rpc,
                block_hash,
                metadata,
                runtime_version,
            )
            .await?;

            Ok(legacy_block.into())
        }
        "ethereum::block::Block<ethereum::transaction::TransactionV2>" => {
            fetch(
                PALLET_NAME,
                storage_entry_name,
                (),
                rpc,
                block_hash,
                metadata,
                runtime_version,
            )
            .await
        }
        other => Err(Error::MetadataError(
            MetadataError::UnexpectedStorageValueType {
                expected: "ethereum::block::Block<ethereum::transaction::LegacyTransaction>"
                    .to_string(),
                got: other.to_string(),
                pallet_name: PALLET_NAME.to_string(),
                storage_entry_name: storage_entry_name.to_string(),
            },
        )),
    }
}
