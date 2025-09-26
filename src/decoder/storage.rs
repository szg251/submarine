use frame_decode::{
    helpers::type_registry_from_metadata,
    storage::{IntoStorageKeys, decode_storage_value, encode_storage_key},
};
use frame_metadata::RuntimeMetadata;
use scale_info_legacy::{ChainTypeRegistry, LookupName};
use scale_value::{Value, scale::ValueVisitor};
use thiserror::Error;

#[derive(Debug)]
pub struct StorageKey(pub Vec<u8>);

#[derive(Debug, Error)]
pub enum StorageKeyEncoderError {
    #[error(transparent)]
    ScaleEncoderFailed(#[from] frame_decode::storage::StorageKeyEncodeError),

    #[error(transparent)]
    CantBuldTypeRegistry(#[from] scale_info_legacy::lookup_name::ParseError),

    #[error("This metadata version is unsupported: {version}")]
    UnsupportedMetadataVersion { version: u32 },
}

#[derive(Debug)]
pub enum AnyStorageValue {
    Legacy(Box<Value<LookupName>>),
    Modern(Value<u32>),
}

#[derive(Debug, Error)]
pub enum StorageValueDecoderError {
    #[error(transparent)]
    ModernScaleDecoderFailed(#[from] frame_decode::storage::StorageValueDecodeError<u32>),

    #[error(transparent)]
    LegacyScaleDecoderFailed(Box<frame_decode::storage::StorageValueDecodeError<LookupName>>),

    #[error(transparent)]
    CantBuldTypeRegistry(#[from] scale_info_legacy::lookup_name::ParseError),

    #[error("This metadata version is unsupported: {version}")]
    UnsupportedMetadataVersion { version: u32 },
}

impl From<frame_decode::storage::StorageValueDecodeError<LookupName>> for StorageValueDecoderError {
    fn from(value: frame_decode::storage::StorageValueDecodeError<LookupName>) -> Self {
        StorageValueDecoderError::LegacyScaleDecoderFailed(Box::new(value))
    }
}

/// Encode any version of storage key
pub fn encode_storage_key_any(
    pallet_name: &str,
    storage_entry_name: &str,
    keys: impl IntoStorageKeys,
    historic_types: &ChainTypeRegistry,
    metadata: &RuntimeMetadata,
    spec_version: u64,
) -> Result<StorageKey, StorageKeyEncoderError> {
    // `ToTypeRegistry` is not exposed by `frame-decode` so we have to match on metadata use
    match metadata {
        RuntimeMetadata::V8(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(StorageKey(encode_storage_key(
                pallet_name,
                storage_entry_name,
                keys,
                metadata,
                &historic_types_for_spec,
            )?))
        }
        RuntimeMetadata::V9(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(StorageKey(encode_storage_key(
                pallet_name,
                storage_entry_name,
                keys,
                metadata,
                &historic_types_for_spec,
            )?))
        }
        RuntimeMetadata::V10(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(StorageKey(encode_storage_key(
                pallet_name,
                storage_entry_name,
                keys,
                metadata,
                &historic_types_for_spec,
            )?))
        }
        RuntimeMetadata::V11(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(StorageKey(encode_storage_key(
                pallet_name,
                storage_entry_name,
                keys,
                metadata,
                &historic_types_for_spec,
            )?))
        }
        RuntimeMetadata::V12(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(StorageKey(encode_storage_key(
                pallet_name,
                storage_entry_name,
                keys,
                metadata,
                &historic_types_for_spec,
            )?))
        }
        RuntimeMetadata::V13(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(StorageKey(encode_storage_key(
                pallet_name,
                storage_entry_name,
                keys,
                metadata,
                &historic_types_for_spec,
            )?))
        }
        RuntimeMetadata::V14(metadata) => Ok(StorageKey(encode_storage_key(
            pallet_name,
            storage_entry_name,
            keys,
            metadata,
            &metadata.types,
        )?)),
        RuntimeMetadata::V15(metadata) => Ok(StorageKey(encode_storage_key(
            pallet_name,
            storage_entry_name,
            keys,
            metadata,
            &metadata.types,
        )?)),
        RuntimeMetadata::V16(metadata) => Ok(StorageKey(encode_storage_key(
            pallet_name,
            storage_entry_name,
            keys,
            metadata,
            &metadata.types,
        )?)),
        _ => Err(StorageKeyEncoderError::UnsupportedMetadataVersion {
            version: metadata.version(),
        }),
    }
}

/// Decodes any version of storage value
pub fn decode_storage_value_any(
    value: impl AsRef<[u8]>,
    pallet_name: &str,
    storage_entry_name: &str,
    historic_types: &ChainTypeRegistry,
    metadata: &RuntimeMetadata,
    spec_version: u64,
) -> Result<AnyStorageValue, StorageValueDecoderError> {
    let value = &mut value.as_ref();

    match metadata {
        RuntimeMetadata::V8(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyStorageValue::Legacy(Box::new(decode_storage_value(
                pallet_name,
                storage_entry_name,
                value,
                metadata,
                &historic_types_for_spec,
                ValueVisitor::new(),
            )?)))
        }
        RuntimeMetadata::V9(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyStorageValue::Legacy(Box::new(decode_storage_value(
                pallet_name,
                storage_entry_name,
                value,
                metadata,
                &historic_types_for_spec,
                ValueVisitor::new(),
            )?)))
        }
        RuntimeMetadata::V10(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyStorageValue::Legacy(Box::new(decode_storage_value(
                pallet_name,
                storage_entry_name,
                value,
                metadata,
                &historic_types_for_spec,
                ValueVisitor::new(),
            )?)))
        }
        RuntimeMetadata::V11(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            let keys: [u8; 0] = [];
            let key =
                encode_storage_key("System", "Events", keys, metadata, &historic_types_for_spec)
                    .unwrap();

            println!("{:?}", hex::encode(key));

            Ok(AnyStorageValue::Legacy(Box::new(decode_storage_value(
                pallet_name,
                storage_entry_name,
                value,
                metadata,
                &historic_types_for_spec,
                ValueVisitor::new(),
            )?)))
        }
        RuntimeMetadata::V12(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyStorageValue::Legacy(Box::new(decode_storage_value(
                pallet_name,
                storage_entry_name,
                value,
                metadata,
                &historic_types_for_spec,
                ValueVisitor::new(),
            )?)))
        }
        RuntimeMetadata::V13(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyStorageValue::Legacy(Box::new(decode_storage_value(
                pallet_name,
                storage_entry_name,
                value,
                metadata,
                &historic_types_for_spec,
                ValueVisitor::new(),
            )?)))
        }
        RuntimeMetadata::V14(metadata) => Ok(AnyStorageValue::Modern(decode_storage_value(
            pallet_name,
            storage_entry_name,
            value,
            metadata,
            &metadata.types,
            ValueVisitor::new(),
        )?)),
        RuntimeMetadata::V15(metadata) => Ok(AnyStorageValue::Modern(decode_storage_value(
            pallet_name,
            storage_entry_name,
            value,
            metadata,
            &metadata.types,
            ValueVisitor::new(),
        )?)),
        RuntimeMetadata::V16(metadata) => Ok(AnyStorageValue::Modern(decode_storage_value(
            pallet_name,
            storage_entry_name,
            value,
            metadata,
            &metadata.types,
            ValueVisitor::new(),
        )?)),
        _ => Err(StorageValueDecoderError::UnsupportedMetadataVersion {
            version: metadata.version(),
        }),
    }
}
