use scale_decode::ext::primitive_types::U256;

use crate::{
    decoder::metadata::{AnyRuntimeMetadata, MetadataError},
    error::Error,
    fetch::fetch,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion},
    },
    pallets::ethereum::decoder::Block,
};

const PALLET_NAME: &str = "Ethereum";

pub fn verify_pallet_metadata(metadata: AnyRuntimeMetadata<'_>) -> Result<(), MetadataError> {
    let type_registry = metadata.type_registry();

    let storage_types = [
        (
            "BlockHash",
            vec!["primitive_types::U256"],
            "primitive_types::H256",
        ),
        (
            "CurrentBlock",
            vec![],
            "ethereum::block::Block<ethereum::transaction::LegacyTransaction>",
        ),
    ];

    storage_types.iter().try_for_each(
        |(storage_entry_name, expected_key_types, expected_value_type)| {
            let (key_types, value_type) = metadata
                .pallet_metadata(PALLET_NAME)?
                .storage_entry(storage_entry_name)?
                .types_as_str(type_registry)?;

            if &value_type[..] != *expected_value_type {
                Err(MetadataError::UnexpectedStorageValueType {
                    expected: expected_value_type.to_string(),
                    got: value_type,
                    pallet_name: PALLET_NAME.to_string(),
                    storage_entry_name: storage_entry_name.to_string(),
                })?
            };

            key_types.iter().enumerate().try_for_each(|(i, key_type)| {
                let expected_key_type = expected_key_types.get(i).copied();
                if Some(&key_type[..]) != expected_key_type {
                    Err(MetadataError::UnexpectedStorageKeyType {
                        expected: format!("{expected_key_type:?}"),
                        got: key_type.to_string(),
                        pallet_name: PALLET_NAME.to_string(),
                        storage_entry_name: storage_entry_name.to_string(),
                    })
                } else {
                    Ok(())
                }
            })
        },
    )
}

pub async fn fetch_block_hash(
    rpc: &NodeRPC,
    block_number: u32,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<Vec<u8>, Error> {
    let block_number = U256::from(block_number).0;
    fetch(
        PALLET_NAME,
        "BlockHash",
        [block_number],
        rpc,
        block_hash,
        metadata,
        runtime_version,
    )
    .await
}

pub async fn fetch_block(
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<Block, Error> {
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
