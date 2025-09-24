use frame_metadata::RuntimeMetadata;
use scale_info_legacy::ChainTypeRegistry;

use crate::decoder::storage::{
    AnyStorageValue, StorageValueDecoderError, decode_storage_value_any,
};

pub const SYSTEM_EVENT_KEY: &str =
    "0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7";

/// Decodes any version of storage value
pub fn decode_events_any(
    historic_types: &ChainTypeRegistry,
    events_bytes: impl AsRef<[u8]>,
    metadata: &RuntimeMetadata,
    spec_version: u64,
) -> Result<AnyStorageValue, StorageValueDecoderError> {
    decode_storage_value_any(
        historic_types,
        "System",
        "Events",
        events_bytes,
        metadata,
        spec_version,
    )
}
