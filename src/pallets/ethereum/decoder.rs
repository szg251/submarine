/// Pallet defined storage parsers and verificators for pallet-ethereum
use chrono::{DateTime, Utc};
use scale_value::Value;

use crate::decoder::value_decoder::{
    ValueDecoderError, WithErrorSpan, decode_as_bytestring, decode_as_record, decode_as_timestamp,
    decode_as_vec,
};

const PALLET_NAME: &str = "Ethereum";

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

pub fn decode_as_block<T>(value: Value<T>) -> Result<Block, ValueDecoderError>
where
    T: std::fmt::Debug,
{
    let mut record = decode_as_record(value)?;

    let header = record
        .remove("header")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "header".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_block_header)
        .add_error_span("header")?;

    let transactions = record
        .remove("transactions")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "transactions".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_vec)
        .map(|vec| vec.iter().map(|tx| tx.to_string()).collect())
        .add_error_span("transactions")?;

    Ok(Block {
        header,
        transactions,
    })
}

fn decode_as_block_header<T>(value: Value<T>) -> Result<BlockHeader, ValueDecoderError>
where
    T: std::fmt::Debug,
{
    let mut record = decode_as_record(value)?;

    let parent_hash = record
        .remove("parent_hash")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "parent_hash".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
        .add_error_span("parent_hash")?;

    let ommers_hash = record
        .remove("ommers_hash")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "ommers_hash".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
        .add_error_span("ommers_hash")?;

    let beneficiary = record
        .remove("beneficiary")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "beneficiary".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
        .add_error_span("beneficiary")?;

    let transactions_root = record
        .remove("transactions_root")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "transactions_root".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
        .add_error_span("transactions_root")?;

    let receipts_root = record
        .remove("receipts_root")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "receipts_root".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
        .add_error_span("receipts_root")?;

    let logs_bloom = record
        .remove("logs_bloom")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "logs_bloom".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
        .add_error_span("logs_bloom")?;

    let difficulty = record
        .remove("difficulty")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "difficulty".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
        .add_error_span("difficulty")?;

    let number = record
        .remove("number")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "number".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
        .add_error_span("number")?;

    let gas_limit = record
        .remove("gas_limit")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "gas_limit".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
        .add_error_span("gas_limit")?;

    let gas_used = record
        .remove("gas_used")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "gas_used".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
        .add_error_span("gas_used")?;

    let timestamp = record
        .remove("timestamp")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "timestamp".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_timestamp)
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
        .and_then(decode_as_bytestring)
        .add_error_span("mix_hash")?;

    let nonce = record
        .remove("nonce")
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: "nonce".to_string(),
            span: String::new(),
        })
        .and_then(decode_as_bytestring)
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
