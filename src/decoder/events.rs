use std::collections::HashMap;

use frame_metadata::RuntimeMetadata;
use scale_value::{Composite, Primitive, Value, ValueDef, Variant};
use thiserror::Error;

use crate::decoder::storage::{
    AnyStorageValue, StorageValueDecoderError, decode_storage_value_any,
};

pub const SYSTEM_EVENTS_KEY: &str =
    "0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7";

#[derive(Debug, Error)]
pub enum EventDecoderError {
    #[error("Value type {expected}, but got {got} at {span}")]
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
}

impl From<StorageValueDecoderError> for EventDecoderError {
    fn from(source: StorageValueDecoderError) -> Self {
        EventDecoderError::StorageValueDecodingFailed {
            source,
            span: String::new(),
        }
    }
}

impl From<scale_value::serde::DeserializerError> for EventDecoderError {
    fn from(source: scale_value::serde::DeserializerError) -> Self {
        EventDecoderError::ValueDeserializerError {
            source,
            span: String::new(),
        }
    }
}

trait WithErrorSpan {
    fn add_error_span(self, span: &str) -> Self;
}

impl<T> WithErrorSpan for Result<T, EventDecoderError> {
    fn add_error_span(self, outer_span: &str) -> Self {
        self.map_err(|mut err| {
            match &mut err {
                EventDecoderError::UnexpectedValueType { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                EventDecoderError::UnexpectedVariantName { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                EventDecoderError::RecordFieldNotFound { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                EventDecoderError::StorageValueDecodingFailed { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                EventDecoderError::ValueDeserializerError { span, .. } => {
                    *span = append_span(span, outer_span);
                }
                EventDecoderError::UnexpectedVectorLength { span, .. } => {
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

/// Decodes any version of System.Events storage value
pub fn decode_events_any(
    events_bytes: impl AsRef<[u8]>,
    metadata: &RuntimeMetadata,
    spec_version: u64,
) -> Result<Vec<EventRecord>, EventDecoderError> {
    let raw_value =
        decode_storage_value_any(events_bytes, "System", "Events", metadata, spec_version)?;

    match raw_value {
        AnyStorageValue::Legacy(value) => parse_events(*value).add_error_span("events"),
        AnyStorageValue::Modern(value) => parse_events(value).add_error_span("events"),
    }
}

fn parse_events<T>(value: Value<T>) -> Result<Vec<EventRecord>, EventDecoderError>
where
    T: std::fmt::Debug,
{
    parse_vec(value)?
        .into_iter()
        .map(parse_event_record)
        .collect::<Result<Vec<_>, _>>()
        .add_error_span("event_record")
}

fn parse_event_record<T>(value: Value<T>) -> Result<EventRecord, EventDecoderError>
where
    T: std::fmt::Debug,
{
    let mut record = parse_record(value)?;

    let phase = record
        .remove("phase")
        .ok_or(EventDecoderError::RecordFieldNotFound {
            field_name: "phase".to_string(),
            span: String::new(),
        })
        .and_then(parse_phase)
        .add_error_span("phase")?;

    let event = record
        .remove("event")
        .ok_or(EventDecoderError::RecordFieldNotFound {
            field_name: "event".to_string(),
            span: String::new(),
        })
        .and_then(parse_event)
        .add_error_span("event")?;

    let topics = record
        .remove("topics")
        .ok_or(EventDecoderError::RecordFieldNotFound {
            field_name: "topics".to_string(),
            span: String::new(),
        })
        .and_then(parse_vec)
        .map(|vec| vec.iter().map(scale_value::stringify::to_string).collect())
        .add_error_span("topics")?;

    Ok(EventRecord {
        phase,
        event,
        topics,
    })
}

fn parse_vec<T>(value: Value<T>) -> Result<Vec<Value<T>>, EventDecoderError>
where
    T: std::fmt::Debug,
{
    match value.value {
        ValueDef::Composite(Composite::Unnamed(vec)) => Ok(vec),
        other => Err(EventDecoderError::UnexpectedValueType {
            expected: "ValueDef::Composite(Composite::Unnamed(_))".to_string(),
            got: format!("{other:?}"),
            span: String::new(),
        }),
    }
}

fn parse_record<T>(value: Value<T>) -> Result<HashMap<String, Value<T>>, EventDecoderError>
where
    T: std::fmt::Debug,
{
    match value.value {
        ValueDef::Composite(Composite::Named(named)) => {
            Ok(named.into_iter().collect::<HashMap<_, _>>())
        }
        other => Err(EventDecoderError::UnexpectedValueType {
            expected: "ValueDef::Composite(Composite::Named(_))".to_string(),
            got: format!("{other:?}"),
            span: String::new(),
        }),
    }
}

fn parse_phase<T>(value: Value<T>) -> Result<Phase, EventDecoderError>
where
    T: std::fmt::Debug,
{
    match value.value {
        ValueDef::Variant(Variant { name, values }) => match &name[..] {
            "ApplyExtrinsic" => match values {
                Composite::Unnamed(mut vec) => {
                    let fst = vec.pop().ok_or(EventDecoderError::UnexpectedVectorLength {
                        expected: 1,
                        got: vec.len(),
                        span: String::new(),
                    })?;

                    match fst.value {
                        ValueDef::Primitive(Primitive::U128(extrinsic_idx)) => {
                            Ok(Phase::ApplyExtrinsic(extrinsic_idx as u32))
                        }
                        _ => Err(EventDecoderError::UnexpectedVariantName {
                            variant_name: name,
                            span: String::new(),
                        }),
                    }
                }
                other => Err(EventDecoderError::UnexpectedValueType {
                    span: String::new(),
                    expected: "Composite::Unnamed(_)".to_string(),
                    got: format!("{other:?}"),
                }),
            }
            .add_error_span("ApplyExtrinsic"),
            "Finalization" => match values {
                Composite::Unnamed(_) => Ok(Phase::Finalization),
                other => Err(EventDecoderError::UnexpectedValueType {
                    span: String::new(),
                    expected: "Composite::Unnamed(_)".to_string(),
                    got: format!("{other:?}"),
                }),
            }
            .add_error_span("Finalization"),
            "Initialization" => match values {
                Composite::Unnamed(_) => Ok(Phase::Finalization),
                other => Err(EventDecoderError::UnexpectedValueType {
                    span: String::new(),
                    expected: "Composite::Unnamed(_)".to_string(),
                    got: format!("{other:?}"),
                }),
            }
            .add_error_span("Initialization"),
            _ => Err(EventDecoderError::UnexpectedVariantName {
                variant_name: name,
                span: String::new(),
            }),
        },
        other => Err(EventDecoderError::UnexpectedValueType {
            span: String::new(),
            expected: "ValueDef::Variant(Variant { .. }) ".to_string(),
            got: format!("{other:?}"),
        }),
    }
}

fn parse_event<T>(value: Value<T>) -> Result<Event, EventDecoderError>
where
    T: std::fmt::Debug,
{
    match value.value {
        ValueDef::Variant(Variant {
            name,
            values: Composite::Unnamed(mut vec),
        }) => {
            let fst = vec.pop().ok_or(EventDecoderError::UnexpectedVectorLength {
                expected: 1,
                got: vec.len(),
                span: String::new(),
            })?;
            let (action, params) = match fst.value {
                ValueDef::Variant(Variant { name, values }) => Ok((name, values.to_string())),
                other => Err(EventDecoderError::UnexpectedValueType {
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
        other => Err(EventDecoderError::UnexpectedValueType {
            span: String::new(),
            expected: "ValueDef::Variant(Variant { name, values: Composite::Unnamed(_) })"
                .to_string(),
            got: format!("{other:?}"),
        }),
    }
}

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
