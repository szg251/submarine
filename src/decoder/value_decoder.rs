use std::collections::HashMap;

use chrono::{DateTime, Utc};
use scale_value::{Composite, Primitive, Value, ValueDef};
use thiserror::Error;

use crate::decoder::storage::StorageValueDecoderError;

pub trait ValueDecoder<T> {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        Self: Sized,
        T: std::fmt::Debug;
}

#[derive(Debug, Error)]
pub enum ValueDecoderError {
    #[error("Expected value type of {expected}, but got {got} at {span}")]
    UnexpectedValueType {
        span: String,
        expected: String,
        got: String,
    },

    #[error("Unexpected variant name {variant_name} at {span}")]
    UnexpectedVariantName { variant_name: String, span: String },

    #[error("We expected a vector of {expected} items, got {got} at {span}")]
    UnexpectedVectorLength {
        expected: usize,
        got: usize,
        span: String,
    },

    #[error("Couldn't find record field {field_name} at {span}")]
    RecordFieldNotFound { field_name: String, span: String },

    #[error("StoageValue decoder error at {span}: {source}")]
    StorageValueDecodingFailed {
        span: String,
        source: StorageValueDecoderError,
    },

    #[error("Value deserializer error at {span}: {source}")]
    ValueDeserializerError {
        source: scale_value::serde::DeserializerError,
        span: String,
    },

    #[error("Timestamp storage value has an invalid value")]
    TimestampValueInvalid { span: String },
}

impl From<StorageValueDecoderError> for ValueDecoderError {
    fn from(source: StorageValueDecoderError) -> Self {
        ValueDecoderError::StorageValueDecodingFailed {
            source,
            span: String::new(),
        }
    }
}

impl From<scale_value::serde::DeserializerError> for ValueDecoderError {
    fn from(source: scale_value::serde::DeserializerError) -> Self {
        ValueDecoderError::ValueDeserializerError {
            source,
            span: String::new(),
        }
    }
}

pub trait WithErrorSpan {
    fn add_error_span(self, span: &str) -> Self;
}

impl<T> WithErrorSpan for Result<T, ValueDecoderError> {
    fn add_error_span(self, outer_span: &str) -> Self {
        self.map_err(|mut err| {
            match &mut err {
                ValueDecoderError::UnexpectedValueType { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                ValueDecoderError::UnexpectedVariantName { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                ValueDecoderError::RecordFieldNotFound { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                ValueDecoderError::StorageValueDecodingFailed { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                ValueDecoderError::ValueDeserializerError { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                ValueDecoderError::UnexpectedVectorLength { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                ValueDecoderError::TimestampValueInvalid { span, .. } => {
                    *span = append_span(span, outer_span);
                }
            };
            err
        })
    }
}

fn append_span(inner_span: &str, outer_span: &str) -> String {
    if !inner_span.is_empty() {
        format!("{outer_span}.{inner_span}")
    } else {
        outer_span.to_string()
    }
}

pub fn get_field<T, R>(
    field_name: &str,
    record: &mut HashMap<String, Value<T>>,
) -> Result<R, ValueDecoderError>
where
    R: ValueDecoder<T>,
    T: std::fmt::Debug,
{
    record
        .remove(field_name)
        .ok_or(ValueDecoderError::RecordFieldNotFound {
            field_name: field_name.to_string(),
            span: String::new(),
        })
        .and_then(R::decode)
        .add_error_span(field_name)
}

/// Decode a `ValueDef::Composite(Composite::Unnamed(vec))` Value with exactly one element
pub fn decode_singleton<T, R>(value: Value<T>) -> Result<R, ValueDecoderError>
where
    T: std::fmt::Debug,
    R: ValueDecoder<T>,
{
    match value.value {
        ValueDef::Composite(Composite::Unnamed(mut vec)) => {
            let fst = if vec.len() == 1 {
                vec.pop().unwrap()
            } else {
                Err(ValueDecoderError::UnexpectedVectorLength {
                    expected: 1,
                    got: vec.len(),
                    span: String::new(),
                })?
            };

            Ok(R::decode(fst)?)
        }
        other => Err(ValueDecoderError::UnexpectedValueType {
            span: String::new(),
            expected: "ValueDef::Composite(Composite::Unnamed(_))".to_string(),
            got: format!("{other:?}"),
        }),
    }
}

impl<T, R> ValueDecoder<T> for Vec<R>
where
    R: ValueDecoder<T>,
{
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        match value.value {
            ValueDef::Composite(Composite::Unnamed(vec)) => {
                vec.into_iter().map(ValueDecoder::decode).collect()
            }
            other => Err(ValueDecoderError::UnexpectedValueType {
                expected: "ValueDef::Composite(Composite::Unnamed(_))".to_string(),
                got: format!("{other:?}"),
                span: String::new(),
            }),
        }
    }
}

impl<T> ValueDecoder<T> for HashMap<String, Value<T>> {
    fn decode(value: Value<T>) -> Result<HashMap<String, Value<T>>, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        match value.value {
            ValueDef::Composite(Composite::Named(named)) => {
                Ok(named.into_iter().collect::<HashMap<_, _>>())
            }
            other => Err(ValueDecoderError::UnexpectedValueType {
                expected: "ValueDef::Composite(Composite::Named(_))".to_string(),
                got: format!("{other:?}"),
                span: String::new(),
            }),
        }
    }
}

/// Parse a `ValueDef::Composite(Composite::Unnamed(_))` to a `[u8]`
impl<T, const N: usize, R> ValueDecoder<T> for [R; N]
where
    R: std::marker::Copy + std::default::Default + ValueDecoder<T>,
{
    fn decode(value: Value<T>) -> Result<[R; N], ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        match value.value {
            ValueDef::Composite(Composite::Unnamed(vec)) => {
                if vec.len() != N {
                    Err(ValueDecoderError::UnexpectedVectorLength {
                        expected: N,
                        got: vec.len(),
                        span: String::new(),
                    })
                } else {
                    let mut result = [Default::default(); N];
                    vec.into_iter().enumerate().try_for_each(|(i, value)| {
                        result[i] = ValueDecoder::decode(value)?;
                        Ok::<(), ValueDecoderError>(())
                    })?;
                    Ok(result)
                }
            }
            other => Err(ValueDecoderError::UnexpectedValueType {
                expected: "ValueDef::Composite(Composite::Unnamed(_))".to_string(),
                got: format!("{other:?}"),
                span: String::new(),
            }),
        }
    }
}

impl<T> ValueDecoder<T> for DateTime<Utc> {
    fn decode(value: Value<T>) -> Result<DateTime<Utc>, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        match value.value {
            ValueDef::Primitive(Primitive::U128(uint)) => DateTime::from_timestamp_millis(
                uint as i64,
            )
            .ok_or(ValueDecoderError::TimestampValueInvalid {
                span: String::new(),
            }),
            other => Err(ValueDecoderError::UnexpectedValueType {
                expected: "ValueDef::Primitive(Primitive::U128(_))".to_string(),
                got: format!("{other:?}"),
                span: String::new(),
            }),
        }
    }
}

impl<T> ValueDecoder<T> for u8 {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        match value.value {
            ValueDef::Primitive(Primitive::U128(uint)) => Ok(uint as u8),
            other => Err(ValueDecoderError::UnexpectedValueType {
                expected: "ValueDef::Primitive(Primitive::U128(_))".to_string(),
                got: format!("{other:?}"),
                span: String::new(),
            }),
        }
    }
}

impl<T> ValueDecoder<T> for u64 {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        match value.value {
            ValueDef::Primitive(Primitive::U128(uint)) => Ok(uint as u64),
            other => Err(ValueDecoderError::UnexpectedValueType {
                expected: "ValueDef::Primitive(Primitive::U128(_))".to_string(),
                got: format!("{other:?}"),
                span: String::new(),
            }),
        }
    }
}

impl<T> ValueDecoder<T> for Value<T> {
    fn decode(value: Value<T>) -> Result<Self, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        Ok(value)
    }
}
