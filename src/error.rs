use thiserror::Error;
use tracing::subscriber::SetGlobalDefaultError;

use crate::{
    decoder::{
        extrinsic::ExtrinsicDecoderError,
        metadata::MetadataError,
        storage::{StorageKeyEncoderError, StorageValueDecoderError},
        value_decoder::ValueDecoderError,
    },
    node_rpc::client::NodeRPCError,
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

    #[error("Storage value not found for key {0}")]
    StorageValueNotFound(String),

    #[error(transparent)]
    ValueDecoderError(#[from] ValueDecoderError),

    #[error("Failed to parse RuntimeMetadata: {0}")]
    ParsingRuntimeMetadataFailed(parity_scale_codec::Error),

    #[error(transparent)]
    MetadataError(#[from] MetadataError),
}
