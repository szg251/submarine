use crate::{
    decoder::{
        metadata::AnyRuntimeMetadata,
        storage::{AnyStorageValue, decode_storage_value_any},
        value_decoder::WithErrorSpan,
    },
    error::Error,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion, StorageKeyHex, StorageValueBytes},
    },
    pallets::system::decoder::{EventRecord, decode_as_events},
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
        .ok_or(Error::StorageValueNotFound(system_events_key_hex.0))?;

    let raw_value = decode_storage_value_any(
        events_bytes,
        "System",
        "Events",
        metadata,
        runtime_version.spec_version,
    )?;

    Ok(match raw_value {
        AnyStorageValue::Legacy(value) => decode_as_events(*value).add_error_span("events")?,
        AnyStorageValue::Modern(value) => decode_as_events(value).add_error_span("events")?,
    })
}
