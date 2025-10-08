use crate::{
    decoder::metadata::{AnyRuntimeMetadata, MetadataError},
    error::Error,
    fetch::fetch,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion},
    },
    pallets::session::decoder::{LegacyValidatorId, ValidatorId},
};

pub const PALLET_NAME: &str = "Session";

pub async fn fetch_validators(
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: AnyRuntimeMetadata<'_>,
    runtime_version: &RuntimeVersion,
) -> Result<Vec<ValidatorId>, Error> {
    let storage_entry_name = "Validators";
    let type_registry = metadata.type_registry();

    let (_key_types, value_type) = metadata
        .pallet_metadata(PALLET_NAME)?
        .storage_entry(storage_entry_name)?
        .types_as_str(type_registry)?;

    match &value_type[..] {
        // Legacy version
        "Vec<T::ValidatorId>" => Ok(fetch(
            PALLET_NAME,
            storage_entry_name,
            (),
            rpc,
            block_hash,
            metadata,
            runtime_version,
        )
        .await?),
        // Modern version (builtin)
        "" => {
            let legacy_validator_id: Vec<LegacyValidatorId> = fetch(
                PALLET_NAME,
                storage_entry_name,
                (),
                rpc,
                block_hash,
                metadata,
                runtime_version,
            )
            .await?;
            Ok(legacy_validator_id
                .into_iter()
                .map(ValidatorId::from)
                .collect())
        }
        other => Err(Error::MetadataError(
            MetadataError::UnexpectedStorageValueType {
                expected: "`Vec<T::ValidatorId>` or ``".to_string(),
                got: other.to_string(),
                pallet_name: PALLET_NAME.to_string(),
                storage_entry_name: storage_entry_name.to_string(),
            },
        )),
    }
}
