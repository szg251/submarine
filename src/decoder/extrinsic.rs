use frame_decode::{
    extrinsics::{Extrinsic, decode_extrinsic},
    helpers::type_registry_from_metadata,
};
use frame_metadata::RuntimeMetadata;
use scale_info_legacy::LookupName;
use thiserror::Error;

use crate::decoder::metadata::AnyRuntimeMetadata;

#[derive(Debug)]
pub enum AnyExtrinsic<'info> {
    Legacy(Box<Extrinsic<'info, LookupName>>),
    Modern(Extrinsic<'info, u32>),
}

impl<'info> AnyExtrinsic<'info> {
    pub fn pallet_name(&self) -> &str {
        match self {
            AnyExtrinsic::Legacy(ext) => ext.pallet_name(),
            AnyExtrinsic::Modern(ext) => ext.pallet_name(),
        }
    }

    pub fn call_name(&self) -> &str {
        match self {
            AnyExtrinsic::Legacy(ext) => ext.call_name(),
            AnyExtrinsic::Modern(ext) => ext.call_name(),
        }
    }

    // pub fn hash(&self) -> Option<&str> {
    //     match self {
    //         AnyExtrinsic::Legacy(_) => None,
    //         // AnyExtrinsic::Modern(ext) => ext.
    //     }
    // }
}

#[derive(Debug, Error)]
pub enum ExtrinsicDecoderError {
    #[error(transparent)]
    ScaleDecoderFailed(#[from] frame_decode::extrinsics::ExtrinsicDecodeError),

    #[error(transparent)]
    CantBuldTypeRegistry(#[from] scale_info_legacy::lookup_name::ParseError),

    #[error("This metadata version is unsupported: {version}")]
    UnsupportedMetadataVersion { version: u32 },
}

/// Decodes any version of extrinsic
pub fn decode_extrinsic_any<'info>(
    ext: impl AsRef<[u8]>,
    metadata: AnyRuntimeMetadata<'info>,
    spec_version: u64,
) -> Result<AnyExtrinsic<'info>, ExtrinsicDecoderError> {
    let historic_types = frame_decode::legacy_types::polkadot::relay_chain();
    let ext = &mut ext.as_ref();

    match metadata.0 {
        RuntimeMetadata::V8(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyExtrinsic::Legacy(Box::new(decode_extrinsic(
                ext,
                metadata,
                &historic_types_for_spec,
            )?)))
        }
        RuntimeMetadata::V9(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyExtrinsic::Legacy(Box::new(decode_extrinsic(
                ext,
                metadata,
                &historic_types_for_spec,
            )?)))
        }
        RuntimeMetadata::V10(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyExtrinsic::Legacy(Box::new(decode_extrinsic(
                ext,
                metadata,
                &historic_types_for_spec,
            )?)))
        }
        RuntimeMetadata::V11(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyExtrinsic::Legacy(Box::new(decode_extrinsic(
                ext,
                metadata,
                &historic_types_for_spec,
            )?)))
        }
        RuntimeMetadata::V12(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyExtrinsic::Legacy(Box::new(decode_extrinsic(
                ext,
                metadata,
                &historic_types_for_spec,
            )?)))
        }
        RuntimeMetadata::V13(metadata) => {
            let mut historic_types_for_spec = historic_types.for_spec_version(spec_version);
            let types_from_metadata = type_registry_from_metadata(metadata)?;
            historic_types_for_spec.prepend(types_from_metadata);

            Ok(AnyExtrinsic::Legacy(Box::new(decode_extrinsic(
                ext,
                metadata,
                &historic_types_for_spec,
            )?)))
        }
        RuntimeMetadata::V14(metadata) => Ok(AnyExtrinsic::Modern(decode_extrinsic(
            ext,
            metadata,
            &metadata.types,
        )?)),
        RuntimeMetadata::V15(metadata) => Ok(AnyExtrinsic::Modern(decode_extrinsic(
            ext,
            metadata,
            &metadata.types,
        )?)),
        RuntimeMetadata::V16(metadata) => Ok(AnyExtrinsic::Modern(decode_extrinsic(
            ext,
            metadata,
            &metadata.types,
        )?)),
        _ => Err(ExtrinsicDecoderError::UnsupportedMetadataVersion {
            version: metadata.version(),
        }),
    }
}
