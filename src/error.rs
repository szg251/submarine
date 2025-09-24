use thiserror::Error;
use tracing::subscriber::SetGlobalDefaultError;

use crate::{
    decoder::{extrinsic::ExtrinsicDecoderError, storage::StorageValueDecoderError},
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
    StorageKVDecoderError(#[from] StorageValueDecoderError),

    #[error("Failed to read metadata file: {0}")]
    ReadingMetadataFileFailed(std::io::Error),

    #[error("Failed to parse metadata file: {0}")]
    ParsingMetadataFileFailed(serde_yaml::Error),

    #[error("Failed to parse RuntimeMetadata: {0}")]
    ParsingRuntimeMetadataFailed(parity_scale_codec::Error),
}
