use thiserror::Error;
use tracing::subscriber::SetGlobalDefaultError;

use crate::{
    decoder::{
        extrinsic::ExtrinsicDecoderError,
        metadata::MetadataError,
        storage::{StorageKeyEncoderError, StorageValueDecoderError},
        value_decoder::ValueDecoderError,
    },
    node_rpc::{client::NodeRPCError, models::StorageKeyHex},
};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    NodeRPCError(#[from] NodeRPCError),

    #[error(transparent)]
    LoggingError(#[from] SetGlobalDefaultError),

    #[error(transparent)]
    ExtrinsicDecoderError(#[from] ExtrinsicDecoderError),

    #[error(transparent)]
    StorageKeyEncoderError(#[from] StorageKeyEncoderError),

    #[error(transparent)]
    StorageValueDecoderError(#[from] StorageValueDecoderError),

    #[error(
        "Storage value not found in storage entry {storage_entry_name} of pallet {pallet_name}{:?}. Storage key hash {storage_entry_key_hash}",
        storage_entry_keys.as_ref().map(|ref x| format!(" with key(s) ({x})")).unwrap_or(String::new())
    )]
    StorageValueNotFound {
        pallet_name: String,
        storage_entry_name: String,
        storage_entry_keys: Option<String>,
        storage_entry_key_hash: StorageKeyHex,
    },

    #[error(transparent)]
    ValueDecoderError(#[from] ValueDecoderError),

    #[error("Failed to parse RuntimeMetadata: {0}")]
    ParsingRuntimeMetadataFailed(parity_scale_codec::Error),

    #[error("Failed to parse Digest Logs: {0}")]
    ParsingDigestLogsFailed(parity_scale_codec::Error),

    #[error("Couldn't find validator by authority index {0}")]
    ValidatorNotFoundForIndex(u32),

    #[error(transparent)]
    MetadataError(#[from] MetadataError),
}
