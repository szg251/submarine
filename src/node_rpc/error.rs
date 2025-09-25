use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeRPCError {
    #[error(transparent)]
    FailedConnection(jsonrpsee::core::client::Error),

    #[error("JSON RPC request failed for method {method} with {source}")]
    RequestFailed {
        method: String,
        source: jsonrpsee::core::client::Error,
    },
}
