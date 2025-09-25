use frame_metadata::RuntimeMetadata;
use scale_info_legacy::ChainTypeRegistry;

use crate::decoder::storage::{
    AnyStorageValue, StorageValueDecoderError, decode_storage_value_any,
};

pub const SYSTEM_EVENTS_KEY: &str =
    "0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7";

/// Decodes any version of System.Events storage value
pub fn decode_events_any(
    events_bytes: impl AsRef<[u8]>,
    historic_types: &ChainTypeRegistry,
    metadata: &RuntimeMetadata,
    spec_version: u64,
) -> Result<AnyStorageValue, StorageValueDecoderError> {
    decode_storage_value_any(
        events_bytes,
        "System",
        "Events",
        historic_types,
        metadata,
        spec_version,
    )
}
