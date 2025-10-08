use scale_value::Value;

use crate::decoder::value_decoder::{ValueDecoder, ValueDecoderError};

#[derive(Debug)]
pub struct ValidatorId(pub [u8; 32]);

impl<T> ValueDecoder<T> for ValidatorId {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug,
    {
        Ok(Self(ValueDecoder::decode(value)?))
    }
}
