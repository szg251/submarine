use chrono::{DateTime, Utc};

use crate::{
    decoder::{
        metadata::AnyRuntimeMetadata,
        storage::{AnyStorageValue, decode_storage_value_any, encode_storage_key_any},
        value_decoder::decode_as_timestamp,
    },
    error::Error,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion, StorageKeyHex, StorageValueBytes},
    },
};

pub async fn fetch_timestamp(
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<DateTime<Utc>, Error> {
    let pallet_name = "Timestamp";
    let storage_entry_name = "Now";

    let timestamp_keys: [u8; 0] = [];
    let timestamp_key = encode_storage_key_any(
        pallet_name,
        storage_entry_name,
        timestamp_keys,
        metadata,
        runtime_version.spec_version,
    )?;

    let timestamp_key_hex = StorageKeyHex::from(timestamp_key.0);

    let StorageValueBytes(storage_bytes) = rpc
        .state_get_storage(&timestamp_key_hex, block_hash)
        .await?
        .ok_or(Error::StorageValueNotFound(timestamp_key_hex.0))?;

    let value = decode_storage_value_any(
        storage_bytes,
        pallet_name,
        storage_entry_name,
        metadata,
        runtime_version.spec_version,
    )?;

    Ok(match value {
        AnyStorageValue::Legacy(value) => decode_as_timestamp(*value)?,
        AnyStorageValue::Modern(value) => decode_as_timestamp(value)?,
    })
}
