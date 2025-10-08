use crate::{
    decoder::metadata::AnyRuntimeMetadata,
    error::Error,
    fetch::fetch,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion},
    },
    pallets::session::decoder::ValidatorId,
};

pub const PALLET_NAME: &str = "Session";

pub const STORAGE_TYPES: [(&str, (&[&str], &str)); 1] =
    [("Validators", (&[], "Vec<T::ValidatorId>"))];

pub async fn fetch_validators(
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<Vec<ValidatorId>, Error> {
    fetch(
        PALLET_NAME,
        "Validators",
        (),
        rpc,
        block_hash,
        metadata,
        runtime_version,
    )
    .await
}
