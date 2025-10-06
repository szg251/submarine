use std::collections::HashMap;

use scale_value::{Composite, Primitive, Value, ValueDef, Variant};

use crate::decoder::value_decoder::{ValueDecoder, ValueDecoderError, WithErrorSpan};

pub const PALLET_NAME: &str = "System";

pub const SYSTEM_EVENTS_KEY: &str =
    "0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Phase {
    ApplyExtrinsic(u32),
    Finalization,
    Initialization,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EventRecord {
    pub phase: Phase,
    pub event: Event,
    pub topics: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub action: String,
    pub params: String,
}

impl<T> ValueDecoder<T> for EventRecord {
    fn decode(value: Value<T>) -> Result<EventRecord, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        let mut record = HashMap::decode(value)?;

        let phase = record
            .remove("phase")
            .ok_or(ValueDecoderError::RecordFieldNotFound {
                field_name: "phase".to_string(),
                span: String::new(),
            })
            .and_then(Phase::decode)
            .add_error_span("phase")?;

        let event = record
            .remove("event")
            .ok_or(ValueDecoderError::RecordFieldNotFound {
                field_name: "event".to_string(),
                span: String::new(),
            })
            .and_then(Event::decode)
            .add_error_span("event")?;

        let topics = record
            .remove("topics")
            .ok_or(ValueDecoderError::RecordFieldNotFound {
                field_name: "topics".to_string(),
                span: String::new(),
            })
            .and_then(ValueDecoder::decode)
            .map(|vec: Vec<Value<T>>| vec.iter().map(scale_value::stringify::to_string).collect())
            .add_error_span("topics")?;

        Ok(EventRecord {
            phase,
            event,
            topics,
        })
    }
}

impl<T> ValueDecoder<T> for Phase {
    fn decode(value: Value<T>) -> Result<Phase, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        match value.value {
            ValueDef::Variant(Variant { name, values }) => match &name[..] {
                "ApplyExtrinsic" => match values {
                    Composite::Unnamed(mut vec) => {
                        let fst = vec.pop().ok_or(ValueDecoderError::UnexpectedVectorLength {
                            expected: 1,
                            got: vec.len(),
                            span: String::new(),
                        })?;

                        match fst.value {
                            ValueDef::Primitive(Primitive::U128(extrinsic_idx)) => {
                                Ok(Phase::ApplyExtrinsic(extrinsic_idx as u32))
                            }
                            _ => Err(ValueDecoderError::UnexpectedVariantName {
                                variant_name: name,
                                span: String::new(),
                            }),
                        }
                    }
                    other => Err(ValueDecoderError::UnexpectedValueType {
                        span: String::new(),
                        expected: "Composite::Unnamed(_)".to_string(),
                        got: format!("{other:?}"),
                    }),
                }
                .add_error_span("ApplyExtrinsic"),
                "Finalization" => match values {
                    Composite::Unnamed(_) => Ok(Phase::Finalization),
                    other => Err(ValueDecoderError::UnexpectedValueType {
                        span: String::new(),
                        expected: "Composite::Unnamed(_)".to_string(),
                        got: format!("{other:?}"),
                    }),
                }
                .add_error_span("Finalization"),
                "Initialization" => match values {
                    Composite::Unnamed(_) => Ok(Phase::Finalization),
                    other => Err(ValueDecoderError::UnexpectedValueType {
                        span: String::new(),
                        expected: "Composite::Unnamed(_)".to_string(),
                        got: format!("{other:?}"),
                    }),
                }
                .add_error_span("Initialization"),
                _ => Err(ValueDecoderError::UnexpectedVariantName {
                    variant_name: name,
                    span: String::new(),
                }),
            },
            other => Err(ValueDecoderError::UnexpectedValueType {
                span: String::new(),
                expected: "ValueDef::Variant(Variant { .. }) ".to_string(),
                got: format!("{other:?}"),
            }),
        }
    }
}

impl<T> ValueDecoder<T> for Event {
    fn decode(value: Value<T>) -> Result<Event, ValueDecoderError>
    where
        T: std::fmt::Debug,
    {
        match value.value {
            ValueDef::Variant(Variant {
                name,
                values: Composite::Unnamed(mut vec),
            }) => {
                let fst = vec.pop().ok_or(ValueDecoderError::UnexpectedVectorLength {
                    expected: 1,
                    got: vec.len(),
                    span: String::new(),
                })?;
                let (action, params) = match fst.value {
                    ValueDef::Variant(Variant { name, values }) => Ok((name, values.to_string())),
                    other => Err(ValueDecoderError::UnexpectedValueType {
                        span: String::new(),
                        expected: "ValueDef::Variant(Variant { .. })".to_string(),
                        got: format!("{other:?}"),
                    }),
                }?;
                Ok(Event {
                    name,
                    action,
                    params,
                })
            }
            other => Err(ValueDecoderError::UnexpectedValueType {
                span: String::new(),
                expected: "ValueDef::Variant(Variant { name, values: Composite::Unnamed(_) })"
                    .to_string(),
                got: format!("{other:?}"),
            }),
        }
    }
}
