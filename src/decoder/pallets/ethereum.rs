/// Compatible with pallet-ethereum
use chrono::{DateTime, Utc};
use frame_metadata::{RuntimeMetadata, decode_different::DecodeDifferent};
use scale_value::Value;
use tracing::debug;

use crate::{
    decoder::{
        pallets::utils::{PalletMetadataError, UnresolvableTypeError},
        storage::{AnyStorageValue, decode_storage_value_any, encode_storage_key_any},
        value_parser::{
            ValueDecoderError, WithErrorSpan, parse_bytestring, parse_record, parse_timestamp,
            parse_vec,
        },
    },
    error::Error,
    node_rpc::{
        client::NodeRPC,
        models::{BlockHashHex, RuntimeVersion, StorageKeyHex, StorageValueBytes},
    },
};

const PALLET_NAME: &str = "Ethereum";

pub fn verify_pallet_metadata(metadata: &RuntimeMetadata) -> Result<bool, PalletMetadataError> {
    match metadata {
        // RuntimeMetadata::V8(metadata) => {
        //     let pallet_metadata = try_find(match_decode_different(&metadata.modules)?, |module| {
        //         Ok(match_decode_different(&module.name)? == PALLET_NAME)
        //     })?
        //     .ok_or(PalletMetadataError::MetadataNotFound(
        //         PALLET_NAME.to_string(),
        //     ))?;

        //     let storage = pallet_metadata
        //         .storage
        //         .as_ref()
        //         .ok_or(PalletMetadataError::MetadataNotFound("storage".to_string()))?;
        //     let storage = match_decode_different(&storage)?;
        //     let entries = match_decode_different(&storage.entries)?;

        //     let entry_metadata = try_find(entries, |entry| {
        //         Ok(match_decode_different(&entry.name)? == "CurrentBlock")
        //     })?
        //     .ok_or(PalletMetadataError::MetadataNotFound(
        //         "CurrentBlock".to_string(),
        //     ))?;

        //     debug!(?entry_metadata.ty);

        //     Ok(true)
        // }
        // RuntimeMetadata::V9(metadata) => match_decode_different(&metadata.modules)?
        // .iter()
        // .map(|module| match_decode_different(&module.name).cloned())
        // .collect(),
        // RuntimeMetadata::V10(metadata) => match_decode_different(&metadata.modules)?
        // .iter()
        // .map(|module| match_decode_different(&module.name).cloned())
        // .collect(),
        // RuntimeMetadata::V11(metadata) => match_decode_different(&metadata.modules)?
        // .iter()
        // .map(|module| match_decode_different(&module.name).cloned())
        // .collect(),
        // RuntimeMetadata::V12(metadata) => match_decode_different(&metadata.modules)?
        // .iter()
        // .map(|module| match_decode_different(&module.name).cloned())
        // .collect(),
        // RuntimeMetadata::V13(metadata) => match_decode_different(&metadata.modules)?
        // .iter()
        // .map(|module| match_decode_different(&module.name).cloned())
        // .collect(),
        RuntimeMetadata::V14(metadata) => {
            let pallet_metadata = metadata
                .pallets
                .iter()
                .find(|pallet| pallet.name == PALLET_NAME)
                .ok_or(PalletMetadataError::MetadataNotFound(
                    PALLET_NAME.to_string(),
                ))?;

            let storage = pallet_metadata
                .storage
                .as_ref()
                .ok_or(PalletMetadataError::MetadataNotFound("storage".to_string()))?;

            let entry_metadata = storage
                .entries
                .iter()
                .find(|entry| entry.name == "CurrentBlock")
                .ok_or(PalletMetadataError::MetadataNotFound(
                    "CurrentBlock".to_string(),
                ))?;

            let ty = match entry_metadata.ty {
                frame_metadata::v14::StorageEntryType::Plain(type_id) => Ok(type_id),
                ref other => Err(UnresolvableTypeError::UnexpectedTypeVariant {
                    expected: "StorageEntryType::Plain".to_string(),
                    got: format!("{other:?}"),
                }),
            }?;

            let ty = metadata
                .types
                .resolve(ty.id)
                .ok_or(UnresolvableTypeError::TypeIdNotFound(ty.id))?;

            let param_id = ty
                .type_params
                .first()
                .ok_or(UnresolvableTypeError::TypeParamExpected {
                    expected: 1,
                    got: 0,
                })?
                .ty
                .ok_or(UnresolvableTypeError::TypeParamExpected {
                    expected: 1,
                    got: 0,
                })?
                .id;

            let param_ty = metadata
                .types
                .resolve(param_id)
                .ok_or(UnresolvableTypeError::TypeIdNotFound(param_id))?;

            let segments_ok = ty.path.segments == vec!["ethereum", "block", "Block"]
                && param_ty.path.segments == vec!["ethereum", "transaction", "LegacyTransaction"];

            Ok(segments_ok)
        }
        // RuntimeMetadata::V15(metadata) => Ok(metadata
        // .pallets
        // .iter()
        // .map(|pallet| pallet.name.clone())
        // .collect()),
        // RuntimeMetadata::V16(metadata) => Ok(metadata
        // .pallets
        // .iter()
        // .map(|pallet| pallet.name.clone())
        // .collect()),
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

fn try_find<'a, T, E>(
    vec: &'a Vec<T>,
    predicate: impl Fn(&T) -> Result<bool, E>,
) -> Result<Option<&'a T>, E> {
    let mut result = None;

    for item in vec {
        if predicate(item)? {
            result = Some(item);
            break;
        }
    }

    Ok(result)
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<String>,
}

#[derive(Debug)]
pub struct BlockHeader {
    pub parent_hash: Vec<u8>,
    pub ommers_hash: Vec<u8>,
    pub beneficiary: Vec<u8>,
    pub transactions_root: Vec<u8>,
    pub receipts_root: Vec<u8>,
    pub logs_bloom: Vec<u8>,
    pub difficulty: Vec<u8>,
    pub number: Vec<u8>,
    pub gas_limit: Vec<u8>,
    pub gas_used: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub extra_data: String,
    pub mix_hash: Vec<u8>,
    pub nonce: Vec<u8>,
}

pub async fn fetch_block_hash(
    rpc: &NodeRPC,
    block_number: u32,
    block_hash: &BlockHashHex,
    metadata: &RuntimeMetadata,
    runtime_version: &RuntimeVersion,
) -> Result<Vec<u8>, Error> {
    let storage_entry_name = "BlockHash";

    let current_block_keys: [[u8; 4]; 1] = [block_number.to_le_bytes()]; // CurrentBlock
    let storage_key = encode_storage_key_any(
        PALLET_NAME,
        storage_entry_name,
        current_block_keys,
        metadata,
        runtime_version.spec_version,
    )?;

    let storage_key_hex = StorageKeyHex::from(storage_key.0);

    let StorageValueBytes(storage_bytes) = rpc
        .state_get_storage(&storage_key_hex, block_hash)
        .await?
        .ok_or(Error::StorageValueNotFound(storage_key_hex.0))?;

    let value = decode_storage_value_any(
        storage_bytes,
        PALLET_NAME,
        storage_entry_name,
        metadata,
        runtime_version.spec_version,
    )?;

    Ok(match value {
        AnyStorageValue::Legacy(value) => parse_bytestring(*value)?,
        AnyStorageValue::Modern(value) => parse_bytestring(value)?,
    })
}

pub async fn fetch_block(
    rpc: &NodeRPC,
    block_hash: &BlockHashHex,
    metadata: &RuntimeMetadata,
    runtime_version: &RuntimeVersion,
) -> Result<Block, Error> {
    let storage_entry_name = "CurrentBlock";

    let current_block_keys: [u8; 0] = []; // CurrentBlock
    let storage_key = encode_storage_key_any(
        PALLET_NAME,
        storage_entry_name,
        current_block_keys,
        metadata,
        runtime_version.spec_version,
    )?;

    let storage_key_hex = StorageKeyHex::from(storage_key.0);

    let StorageValueBytes(storage_bytes) = rpc
        .state_get_storage(&storage_key_hex, block_hash)
        .await?
        .ok_or(Error::StorageValueNotFound(storage_key_hex.0))?;

    let value = decode_storage_value_any(
        storage_bytes,
        PALLET_NAME,
        storage_entry_name,
        metadata,
        runtime_version.spec_version,
    )?;

    Ok(match value {
        AnyStorageValue::Legacy(value) => parse_block(*value)?,
        AnyStorageValue::Modern(value) => parse_block(value)?,
    })
}

fn parse_block<T>(value: Value<T>) -> Result<Block, ValueDecoderError>
where
    T: std::fmt::Debug,
{
    let mut record = parse_record(value)?;

    let header = record
        .remove("header")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "header".to_string(),
            span: String::new(),
        })
        .and_then(parse_block_header)
        .add_error_span("header")?;

    let transactions = record
        .remove("transactions")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "transactions".to_string(),
            span: String::new(),
        })
        .and_then(parse_vec)
        .map(|vec| vec.iter().map(|tx| tx.to_string()).collect())
        .add_error_span("transactions")?;

    Ok(Block {
        header,
        transactions,
    })
}
fn parse_block_header<T>(value: Value<T>) -> Result<BlockHeader, ValueDecoderError>
where
    T: std::fmt::Debug,
{
    let mut record = parse_record(value)?;

    let parent_hash = record
        .remove("parent_hash")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "parent_hash".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("parent_hash")?;

    let ommers_hash = record
        .remove("ommers_hash")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "ommers_hash".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("ommers_hash")?;

    let beneficiary = record
        .remove("beneficiary")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "beneficiary".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("beneficiary")?;

    let transactions_root = record
        .remove("transactions_root")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "transactions_root".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("transactions_root")?;

    let receipts_root = record
        .remove("receipts_root")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "receipts_root".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("receipts_root")?;

    let logs_bloom = record
        .remove("logs_bloom")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "logs_bloom".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("logs_bloom")?;

    let difficulty = record
        .remove("difficulty")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "difficulty".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("difficulty")?;

    let number = record
        .remove("number")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "number".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("number")?;

    let gas_limit = record
        .remove("gas_limit")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "gas_limit".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("gas_limit")?;

    let gas_used = record
        .remove("gas_used")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "gas_used".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("gas_used")?;

    let timestamp = record
        .remove("timestamp")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "timestamp".to_string(),
            span: String::new(),
        })
        .and_then(parse_timestamp)
        .add_error_span("timestamp")?;

    let extra_data = record
        .remove("extra_data")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "extra_data".to_string(),
            span: String::new(),
        })
        .map(|value| value.to_string())
        .add_error_span("extra_data")?;

    let mix_hash = record
        .remove("mix_hash")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "mix_hash".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("mix_hash")?;

    let nonce = record
        .remove("nonce")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "nonce".to_string(),
            span: String::new(),
        })
        .and_then(parse_bytestring)
        .add_error_span("nonce")?;

    Ok(BlockHeader {
        parent_hash,
        ommers_hash,
        beneficiary,
        transactions_root,
        receipts_root,
        logs_bloom,
        difficulty,
        number,
        gas_limit,
        gas_used,
        timestamp,
        extra_data,
        mix_hash,
        nonce,
    })
}
