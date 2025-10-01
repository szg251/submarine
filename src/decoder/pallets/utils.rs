use std::collections::HashSet;

use frame_metadata::{RuntimeMetadata, decode_different::DecodeDifferent};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PalletMetadataError {
    #[error("Decoded data unavailable in type DecodeDifferent")]
    DecodedDataUnavailable,

    #[error("Could not found pallet metadata")]
    MetadataNotFound(String),

    #[error("This metadata version is unsupported: {version}")]
    UnsupportedMetadataVersion { version: u32 },

    #[error("Couldn't resolve metadata type: {0}")]
    UnresolvableType(#[from] UnresolvableTypeError),
}

#[derive(Debug, Error)]
pub enum UnresolvableTypeError {
    #[error("Expected a type variant of {expected} but got {got}")]
    UnexpectedTypeVariant { expected: String, got: String },

    #[error("Expected to have {expected} type params but got {got}")]
    TypeParamExpected { expected: usize, got: usize },

    #[error("Type resolver cannot find type id {0}")]
    TypeIdNotFound(u32),
}

pub fn get_pallets(metadata: &RuntimeMetadata) -> Result<HashSet<String>, PalletMetadataError> {
    match metadata {
        RuntimeMetadata::V8(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V9(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V10(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V11(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V12(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V13(metadata) => match_decode_different(&metadata.modules)?
            .iter()
            .map(|module| match_decode_different(&module.name).cloned())
            .collect(),
        RuntimeMetadata::V14(metadata) => Ok(metadata
            .pallets
            .iter()
            .map(|pallet| pallet.name.clone())
            .collect()),
        RuntimeMetadata::V15(metadata) => Ok(metadata
            .pallets
            .iter()
            .map(|pallet| pallet.name.clone())
            .collect()),
        RuntimeMetadata::V16(metadata) => Ok(metadata
            .pallets
            .iter()
            .map(|pallet| pallet.name.clone())
            .collect()),
        _ => Err(PalletMetadataError::UnsupportedMetadataVersion {
            version: metadata.version(),
        }),
    }
}

fn match_decode_different<B, O>(
    decode_different: &DecodeDifferent<B, O>,
) -> Result<&O, PalletMetadataError> {
    match decode_different {
        DecodeDifferent::Encode(_) => Err(PalletMetadataError::DecodedDataUnavailable),
        DecodeDifferent::Decoded(decoded) => Ok(decoded),
    }
}
