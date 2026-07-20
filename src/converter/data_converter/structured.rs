// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Structured JSON and string-map conversion implementations.

use std::collections::HashMap;

#[cfg(feature = "json")]
use serde::Deserializer;

use super::DataConverter;
#[cfg(feature = "json")]
use super::internal::StringMapVisitor;
#[cfg(feature = "json")]
use super::string_source::normalize;
#[cfg(feature = "json")]
use crate::converter::{ConversionLimit, DataFormat, InvalidValueReason};
use crate::converter::{DataConversionError, DataConversionOptions, DataConversionTarget};
use crate::datatype::DataType;

#[cfg(feature = "json")]
impl DataConversionTarget for serde_json::Value {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Json(value) => Ok(value.as_ref().clone()),
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::Json)?;
                let maximum = options.structured().max_text_bytes();
                if value.len() > maximum {
                    return Err(DataConversionError::limit_exceeded(
                        source.data_type(),
                        DataType::Json,
                        ConversionLimit::StructuredTextBytes { maximum },
                    ));
                }
                match serde_json::from_str(value) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(source.invalid(
                        DataType::Json,
                        InvalidValueReason::Deserialization {
                            format: DataFormat::Json,
                        },
                    )),
                }
            }
            DataConverter::StringMap(value) => Ok(serde_json::Value::Object(
                value
                    .iter()
                    .map(|(key, value)| (key.clone(), serde_json::Value::String(value.clone())))
                    .collect(),
            )),
            DataConverter::Unset(_) => Err(source.missing(DataType::Json)),
            _ => Err(source.unsupported(DataType::Json)),
        }
    }
}

/// Deserializes a string map through the duplicate-aware visitor.
///
/// # Parameters
///
/// * `value` - JSON text containing exactly one object.
///
/// # Returns
///
/// A map owning every unique string key and string value.
///
/// # Errors
///
/// Returns [`serde_json::Error`] for syntax errors, trailing data, duplicate
/// keys, or non-string values.
#[cfg(feature = "json")]
fn deserialize_string_map(value: &str) -> Result<HashMap<String, String>, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let result = deserializer.deserialize_map(StringMapVisitor)?;
    deserializer.end()?;
    Ok(result)
}

impl DataConversionTarget for HashMap<String, String> {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        #[cfg(not(feature = "json"))]
        let _ = options;
        match source {
            DataConverter::StringMap(value) => Ok(value.as_ref().clone()),
            #[cfg(feature = "json")]
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::StringMap)?;
                let maximum = options.structured().max_text_bytes();
                if value.len() > maximum {
                    return Err(DataConversionError::limit_exceeded(
                        source.data_type(),
                        DataType::StringMap,
                        ConversionLimit::StructuredTextBytes { maximum },
                    ));
                }
                match deserialize_string_map(value) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(source.invalid(
                        DataType::StringMap,
                        InvalidValueReason::Deserialization {
                            format: DataFormat::Json,
                        },
                    )),
                }
            }
            DataConverter::Unset(_) => Err(source.missing(DataType::StringMap)),
            _ => Err(source.unsupported(DataType::StringMap)),
        }
    }
}
