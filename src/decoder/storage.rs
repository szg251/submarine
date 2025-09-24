use frame_decode::{helpers::type_registry_from_metadata, storage::decode_storage_value};
use frame_metadata::RuntimeMetadata;
use scale_info_legacy::{ChainTypeRegistry, LookupName};
use subxt::{dynamic::Value, ext::scale_value::scale::ValueVisitor};
use thiserror::Error;

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

/// Decodes any version of storage value
pub fn decode_storage_value_any(
    historic_types: &ChainTypeRegistry,
    pallet_name: &str,
    storage_entry_name: &str,
    value: impl AsRef<[u8]>,
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
