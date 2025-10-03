use crate::{
    decoder::{
        metadata::{AnyRuntimeMetadata, MetadataError},
        storage::{AnyStorageValue, decode_storage_value_any, encode_storage_key_any},
        value_decoder::decode_as_bytestring,
    },
    error::Error,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion, StorageKeyHex, StorageValueBytes},
    },
    pallets::ethereum::decoder::{Block, decode_as_block},
};

const PALLET_NAME: &str = "Ethereum";

pub fn verify_pallet_metadata(metadata: AnyRuntimeMetadata<'_>) -> Result<(), MetadataError> {
    let type_registry = metadata.type_registry();

    let storage_types = [
        (
            "CurrentBlock",
            "ethereum::block::Block<ethereum::transaction::LegacyTransaction>",
        ),
        ("BlockHash", "primitive_types::H256"),
    ];

    storage_types
        .iter()
        .try_for_each(|(storage_entry_name, expected_type)| {
            let type_name = metadata
                .pallet_metadata(PALLET_NAME)?
                .storage_entry(storage_entry_name)?
                .type_as_str(type_registry)?;

            if &type_name[..] == *expected_type {
                Ok(())
            } else {
                Err(MetadataError::UnexpectedStorageValueType {
                    expected: expected_type.to_string(),
                    got: type_name,
                    pallet_name: PALLET_NAME.to_string(),
                    storage_entry_name: storage_entry_name.to_string(),
                })
            }
        })
}

pub async fn fetch_block_hash(
    rpc: &NodeRPC,
    block_number: u32,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<Vec<u8>, Error> {
    let storage_entry_name = "BlockHash";

    let current_block_keys: [[u8; 4]; 1] = [block_number.to_le_bytes()]; // CurrentBlock
    let storage_key = encode_storage_key_any(
        PALLET_NAME,
        storage_entry_name,
        current_block_keys,
        metadata,
        runtime_version.spec_version,
    )?;

    let storage_key_hex = StorageKeyHex::from(storage_key.0);

    let StorageValueBytes(storage_bytes) = rpc
        .state_get_storage(&storage_key_hex, block_hash)
        .await?
        .ok_or(Error::StorageValueNotFound(storage_key_hex.0))?;

    let value = decode_storage_value_any(
        storage_bytes,
        PALLET_NAME,
        storage_entry_name,
        metadata,
        runtime_version.spec_version,
    )?;

    Ok(match value {
        AnyStorageValue::Legacy(value) => decode_as_bytestring(*value)?,
        AnyStorageValue::Modern(value) => decode_as_bytestring(value)?,
    })
}

pub async fn fetch_block(
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<Block, Error> {
    let storage_entry_name = "CurrentBlock";

    let current_block_keys: [u8; 0] = []; // CurrentBlock
    let storage_key = encode_storage_key_any(
        PALLET_NAME,
        storage_entry_name,
        current_block_keys,
        metadata,
        runtime_version.spec_version,
    )?;

    let storage_key_hex = StorageKeyHex::from(storage_key.0);

    let StorageValueBytes(storage_bytes) = rpc
        .state_get_storage(&storage_key_hex, block_hash)
        .await?
        .ok_or(Error::StorageValueNotFound(storage_key_hex.0))?;

    let value = decode_storage_value_any(
        storage_bytes,
        PALLET_NAME,
        storage_entry_name,
        metadata,
        runtime_version.spec_version,
    )?;

    Ok(match value {
        AnyStorageValue::Legacy(value) => decode_as_block(*value)?,
        AnyStorageValue::Modern(value) => decode_as_block(value)?,
    })
}
