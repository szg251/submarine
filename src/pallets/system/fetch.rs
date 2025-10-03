use crate::{
    decoder::{
        metadata::AnyRuntimeMetadata,
        storage::{AnyStorageValue, decode_storage_value_any},
        value_decoder::{ValueDecoder, WithErrorSpan},
    },
    error::Error,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion, StorageKeyHex, StorageValueBytes},
    },
    pallets::system::decoder::EventRecord,
};

pub const PALLET_NAME: &str = "System";

// Hard coded key for System: Events
pub const SYSTEM_EVENTS_KEY: &str =
    "0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7";

pub async fn fetch_events(
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<Vec<EventRecord>, Error> {
    let system_events_key_hex = StorageKeyHex(SYSTEM_EVENTS_KEY.to_string());

    let StorageValueBytes(events_bytes) = rpc
        .state_get_storage(&system_events_key_hex, block_hash)
        .await?
        .ok_or(Error::StorageValueNotFound {
            pallet_name: PALLET_NAME.to_string(),
            storage_entry_name: "Events".to_string(),
            storage_entry_keys: None,
            storage_entry_key_hash: system_events_key_hex,
        })?;

    let raw_value = decode_storage_value_any(
        events_bytes,
        PALLET_NAME,
        "Events",
        metadata,
        runtime_version.spec_version,
    )?;

    Ok(match raw_value {
        AnyStorageValue::Legacy(value) => {
            <Vec<EventRecord>>::decode(*value).add_error_span("events")?
        }
        AnyStorageValue::Modern(value) => {
            <Vec<EventRecord>>::decode(value).add_error_span("events")?
        }
    })
}
