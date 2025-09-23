use thiserror::Error;
use tracing::subscriber::SetGlobalDefaultError;

use crate::{metadata::decoder::MetadataDecoderError, node_rpc::client::NodeRPCError};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    NodeRPCError(#[from] NodeRPCError),

    #[error(transparent)]
    LoggingError(#[from] SetGlobalDefaultError),

    #[error(transparent)]
    MetadataDecoderError(#[from] MetadataDecoderError),
}
