use std::collections::HashMap;

/// Pallet defined storage parsers and verificators for pallet-ethereum
use ethereum::{
    Block, Header, LegacyTransaction, TransactionAction, TransactionV2,
    legacy::TransactionSignature,
};
use ethereum_types::{Bloom, H64};
use scale_decode::ext::primitive_types::{H160, H256, U256};
use scale_value::Value;

use crate::decoder::value_decoder::{ValueDecoder, ValueDecoderError, get_field};

impl<T, Tx> ValueDecoder<T> for Block<Tx>
where
    Tx: ValueDecoder<T>,
{
    fn decode(value: Value<T>) -> Result<Block<Tx>, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        let mut record = HashMap::decode(value)?;

        Ok(Block {
            header: get_field("header", &mut record)?,
            transactions: get_field("transactions", &mut record)?,
            ommers: get_field("ommers", &mut record)?,
        })
    }
}

impl<T> ValueDecoder<T> for Header {
    fn decode(value: Value<T>) -> Result<Header, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        let mut record = HashMap::decode(value)?;

        Ok(Header {
            parent_hash: get_field("parent_hash", &mut record)?,
            ommers_hash: get_field("ommers_hash", &mut record)?,
            beneficiary: get_field("beneficiary", &mut record)?,
            state_root: get_field("state_root", &mut record)?,
            transactions_root: get_field("transactions_root", &mut record)?,
            receipts_root: get_field("receipts_root", &mut record)?,
            logs_bloom: get_field("logs_bloom", &mut record)?,
            difficulty: get_field("difficulty", &mut record)?,
            number: get_field("number", &mut record)?,
            gas_limit: get_field("gas_limit", &mut record)?,
            gas_used: get_field("gas_used", &mut record)?,
            timestamp: get_field("timestamp", &mut record)?,
            extra_data: get_field("extra_data", &mut record)?,
            mix_hash: get_field("mix_hash", &mut record)?,
            nonce: get_field("nonce", &mut record)?,
        })
    }
}

impl<T> ValueDecoder<T> for LegacyTransaction {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        let mut record = HashMap::decode(value)?;

        Ok(LegacyTransaction {
            nonce: get_field("nonce", &mut record)?,
            gas_price: get_field("gas_price", &mut record)?,
            gas_limit: get_field("gas_limit", &mut record)?,
            action: get_field("action", &mut record)?,
            value: get_field("value", &mut record)?,
            input: get_field("input", &mut record)?,
            signature: get_field("signature", &mut record)?,
        })
    }
}

impl<T> ValueDecoder<T> for TransactionAction {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        todo!()
    }
}

impl<T> ValueDecoder<T> for TransactionSignature {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        todo!()
    }
}

impl<T> ValueDecoder<T> for TransactionV2 {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        todo!()
    }
}

impl<T> ValueDecoder<T> for H256 {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        Ok(Self(ValueDecoder::decode(value)?))
    }
}

impl<T> ValueDecoder<T> for H160 {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        Ok(Self(ValueDecoder::decode(value)?))
    }
}

impl<T> ValueDecoder<T> for U256 {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        Ok(Self(ValueDecoder::decode(value)?))
    }
}

impl<T> ValueDecoder<T> for H64 {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        Ok(Self(ValueDecoder::decode(value)?))
    }
}

impl<T> ValueDecoder<T> for Bloom {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        Ok(Self(ValueDecoder::decode(value)?))
    }
}
