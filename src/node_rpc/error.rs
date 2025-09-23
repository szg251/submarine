use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeRPCError {
    #[error(transparent)]
    FailedConnection(jsonrpsee::core::client::Error),

    #[error(transparent)]
    RequestFailed(jsonrpsee::core::client::Error),

    #[error(transparent)]
    HexDeserializationFailed(hex::FromHexError),

    #[error(transparent)]
    ScaleDeserializationFailed(parity_scale_codec::Error),

    #[error(transparent)]
    ChainMetadataDeserializationFailed(ChainMetadataDeserializationError),
}

#[derive(Debug, Error)]
pub enum ChainMetadataDeserializationError {
    #[error("Expected a bytestring of length 5 but got {length}")]
    BytestringTooShort { length: usize },
}
