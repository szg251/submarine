use scale_value::{Composite, Value, ValueDef};

use crate::decoder::value_decoder::{ValueDecoder, ValueDecoderError, decode_singleton};

#[derive(Debug)]
pub struct ValidatorId(pub [u8; 32]);

#[derive(Debug)]
pub struct LegacyValidatorId(pub [u8; 32]);

impl From<LegacyValidatorId> for ValidatorId {
    fn from(value: LegacyValidatorId) -> Self {
        ValidatorId(value.0)
    }
}

impl<T> ValueDecoder<T> for ValidatorId {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        Ok(Self(ValueDecoder::decode(value)?))
    }
}

impl<T> ValueDecoder<T> for LegacyValidatorId {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        Ok(Self(decode_singleton(value)?))
    }
}
