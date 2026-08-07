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
use super::internal::CanonicalStringMap;
#[cfg(feature = "json")]
use super::internal::StringMapVisitor;
#[cfg(feature = "json")]
use super::internal::sorted_string_map_entries;
#[cfg(feature = "json")]
use super::string_source::normalize;
#[cfg(feature = "json")]
use crate::converter::ConversionLimit;
use crate::converter::DataConversionError;
use crate::converter::DataConversionOptions;
use crate::converter::DataConversionTarget;
#[cfg(feature = "json")]
use crate::converter::DataFormat;
#[cfg(feature = "json")]
use crate::converter::InvalidValueReason;
use crate::datatype::DataType;

/// Converts a borrowed string map to a JSON object with canonical key order.
#[cfg(feature = "json")]
pub(super) fn string_map_to_json(
    value: &HashMap<String, String>,
) -> serde_json::Value {
    serde_json::Value::Object(
        sorted_string_map_entries(value)
            .into_iter()
            .map(|(key, value)| {
                (key.clone(), serde_json::Value::String(value.clone()))
            })
            .collect(),
    )
}

/// Converts an owned string map to a JSON object with canonical key order.
#[cfg(feature = "json")]
fn string_map_into_json(value: HashMap<String, String>) -> serde_json::Value {
    let mut entries = value.into_iter().collect::<Vec<_>>();
    // A borrowed sort key cannot outlive this closure, so compare directly.
    #[allow(clippy::unnecessary_sort_by)]
    entries.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    serde_json::Value::Object(
        entries
            .into_iter()
            .map(|(key, value)| (key, serde_json::Value::String(value)))
            .collect(),
    )
}

/// Serializes a string map as canonical JSON text without cloning its entries.
///
/// # Errors
///
/// Returns a JSON serialization error if the serializer rejects the output.
#[cfg(feature = "json")]
pub(super) fn string_map_to_json_text(
    value: &HashMap<String, String>,
) -> Result<String, serde_json::Error> {
    serde_json::to_string(&CanonicalStringMap { value })
}

#[cfg(feature = "json")]
impl DataConversionTarget for serde_json::Value {
    /// Converts a borrowed runtime value to JSON.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - String normalization and structured text-limit policies.
    ///
    /// # Returns
    ///
    /// The converted JSON value.
    ///
    /// # Errors
    ///
    /// Returns missing, unsupported, JSON syntax, normalization, or structured
    /// text-limit errors.
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
            DataConverter::StringMap(value) => Ok(string_map_to_json(value)),
            DataConverter::Unset(_) => Err(source.missing(DataType::Json)),
            _ => Err(source.unsupported(DataType::Json)),
        }
    }

    /// Converts a runtime value to JSON, consuming it when possible.
    ///
    /// # Parameters
    ///
    /// * `source` - Runtime value to consume.
    /// * `options` - String normalization and structured text-limit policies.
    ///
    /// # Returns
    ///
    /// The converted JSON value; owned JSON and StringMap sources reuse their
    /// transferable storage.
    ///
    /// # Errors
    ///
    /// Returns the same conversion errors as [`Self::convert_from`].
    #[inline(always)]
    fn convert_owned(
        source: DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Json(value) => Ok(value.into_owned()),
            DataConverter::StringMap(value) => {
                Ok(string_map_into_json(value.into_owned()))
            }
            source => Self::convert_from(&source, options),
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
#[inline(always)]
fn deserialize_string_map(
    value: &str,
) -> Result<HashMap<String, String>, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let result = deserializer.deserialize_map(StringMapVisitor)?;
    deserializer.end()?;
    Ok(result)
}

impl DataConversionTarget for HashMap<String, String> {
    /// Converts a borrowed runtime value to a string map.
    ///
    /// # Parameters
    ///
    /// * `source` - Borrowed runtime value to convert.
    /// * `options` - String normalization and structured text-limit policies.
    ///
    /// # Returns
    ///
    /// A map whose keys and values are strings.
    ///
    /// # Errors
    ///
    /// Returns missing, unsupported, JSON shape, duplicate-key, normalization,
    /// or structured text-limit errors.
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

    /// Converts a runtime value to a string map, consuming it when possible.
    ///
    /// # Parameters
    ///
    /// * `source` - Runtime value to consume.
    /// * `options` - String normalization and structured text-limit policies.
    ///
    /// # Returns
    ///
    /// The converted map; an owned string map reuses its storage.
    ///
    /// # Errors
    ///
    /// Returns the same conversion errors as [`Self::convert_from`].
    #[inline(always)]
    fn convert_owned(
        source: DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::StringMap(value) => Ok(value.into_owned()),
            source => Self::convert_from(&source, options),
        }
    }
}
