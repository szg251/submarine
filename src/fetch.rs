use frame_decode::storage::IntoStorageKeys;
use scale_info_legacy::LookupName;

use crate::{
    decoder::{
        metadata::AnyRuntimeMetadata,
        storage::{AnyStorageValue, decode_storage_value_any, encode_storage_key_any},
        value_decoder::{ValueDecoder, WithErrorSpan},
    },
    error::Error,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion, StorageKeyHex, StorageValueBytes},
    },
};

pub async fn fetch<T, K>(
    pallet_name: &str,
    storage_entry_name: &str,
    storage_keys: K,
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<T, Error>
where
    T: ValueDecoder<LookupName> + ValueDecoder<u32>,
    K: IntoStorageKeys + std::fmt::Debug,
{
    let storage_keys_str = format!("{storage_keys:?}");
    let full_name_str =
        format!("pallet stoge: {pallet_name}:{storage_entry_name}:{storage_keys_str}");
    let key = encode_storage_key_any(
        pallet_name,
        storage_entry_name,
        storage_keys,
        metadata,
        runtime_version.spec_version,
    )?;

    let key_hex = StorageKeyHex::from(key.0);

    let StorageValueBytes(storage_bytes) = rpc
        .state_get_storage(&key_hex, block_hash)
        .await?
        .ok_or(Error::StorageValueNotFound {
            pallet_name: pallet_name.to_string(),
            storage_entry_name: storage_entry_name.to_string(),
            storage_entry_keys: Some(storage_keys_str),
            storage_entry_key_hash: key_hex,
        })?;

    let value = decode_storage_value_any(
        storage_bytes,
        pallet_name,
        storage_entry_name,
        metadata,
        runtime_version.spec_version,
    )?;

    Ok(match value {
        AnyStorageValue::Legacy(value) => {
            ValueDecoder::decode(*value).add_error_span(&full_name_str)?
        }
        AnyStorageValue::Modern(value) => {
            ValueDecoder::decode(value).add_error_span(&full_name_str)?
        }
    })
}
