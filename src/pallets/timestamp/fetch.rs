use chrono::{DateTime, Utc};

use crate::{
    decoder::metadata::AnyRuntimeMetadata,
    error::Error,
    fetch::fetch,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion},
    },
};

pub async fn fetch_timestamp(
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<DateTime<Utc>, Error> {
    fetch(
        "Timestamp",
        "Now",
        (),
        rpc,
        block_hash,
        metadata,
        runtime_version,
    )
    .await
}
