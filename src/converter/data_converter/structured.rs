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
use qubit_budget::JsonSerdeError;
#[cfg(feature = "json")]
use qubit_budget::from_slice_seed_with_budget;
#[cfg(feature = "json")]
use qubit_budget::from_slice_with_budget;
#[cfg(feature = "json")]
use serde_json::Value;

use super::DataConverter;
#[cfg(feature = "json")]
use super::internal::StringMapVisitor;
#[cfg(feature = "json")]
use super::internal::sorted_string_map_entries;
#[cfg(feature = "json")]
use super::string_source::normalize;
#[cfg(feature = "json")]
use crate::converter::ConversionResource;
#[cfg(feature = "json")]
use crate::converter::ConversionSession;
use crate::converter::DataConversionError;
use crate::converter::DataConversionOptions;
use crate::converter::DataConversionTarget;
#[cfg(feature = "json")]
use crate::converter::DataFormat;
#[cfg(feature = "json")]
use crate::converter::InvalidValueReason;
use crate::datatype::DataType;

/// Enforces the configured normalized structured-text byte limit.
///
/// # Parameters
///
/// * `value` - Normalized text whose byte length is checked.
/// * `from` - Source data type retained in a limit error.
/// * `to` - Target data type retained in a limit error.
/// * `options` - Structured conversion limits.
///
/// # Returns
///
/// `Ok(())` when `value` is within the configured limit.
///
/// # Errors
///
/// Returns a structured-text budget error when `value` exceeds the configured
/// maximum.
#[inline]
#[cfg(any(feature = "json", feature = "url"))]
pub(super) fn check_structured_text_limit(
    value: &str,
    from: DataType,
    to: DataType,
    options: &DataConversionOptions,
) -> Result<(), DataConversionError> {
    options
        .structured()
        .max_text_bytes_limit()
        .check(value.len())
        .map_err(|error| DataConversionError::limit_exceeded(from, to, error))
}

/// Converts a borrowed string map to a JSON object with canonical key order.
#[cfg(feature = "json")]
pub(super) fn string_map_to_json(value: &HashMap<String, String>) -> Value {
    Value::Object(
        sorted_string_map_entries(value)
            .into_iter()
            .map(|(key, value)| (key.clone(), Value::String(value.clone())))
            .collect(),
    )
}

/// Converts an owned string map to a JSON object with canonical key order.
#[cfg(feature = "json")]
fn string_map_into_json(value: HashMap<String, String>) -> Value {
    let mut entries = value.into_iter().collect::<Vec<_>>();
    // A borrowed sort key cannot outlive this closure, so compare directly.
    #[allow(clippy::unnecessary_sort_by)]
    entries.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    Value::Object(
        entries
            .into_iter()
            .map(|(key, value)| (key, Value::String(value)))
            .collect(),
    )
}

#[cfg(feature = "json")]
impl DataConversionTarget for Value {
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
        session: &mut ConversionSession<'_>,
    ) -> Result<Self, DataConversionError> {
        let options = session.options();
        match source {
            DataConverter::Json(value) => Ok(value.as_ref().clone()),
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::Json)?;
                check_structured_text_limit(value, source.data_type(), DataType::Json, options)?;
                from_slice_with_budget(value.as_bytes(), session.json_budget_mut())
                    .map_err(|error| map_json_decode_error(source, DataType::Json, error))
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
        session: &mut ConversionSession<'_>,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Json(value) => Ok(value.into_owned()),
            DataConverter::StringMap(value) => Ok(string_map_into_json(value.into_owned())),
            source => Self::convert_from(&source, session),
        }
    }
}

/// Maps a budget-aware JSON decoding error to the public conversion error.
///
/// # Parameters
///
/// * `source` - Conversion source that supplies the source data type.
/// * `target` - Requested structured target data type.
/// * `error` - Budget-aware JSON decoding failure to translate.
///
/// # Returns
///
/// Returns the corresponding value-free public conversion error.
#[cfg(feature = "json")]
fn map_json_decode_error(
    source: &DataConverter<'_>,
    target: DataType,
    error: JsonSerdeError<ConversionResource>,
) -> DataConversionError {
    map_json_decode_error_from_type(source.data_type(), target, error)
}

/// Maps a budget-aware JSON decoding error with an explicit source type.
///
/// # Parameters
///
/// * `source` - Runtime source data type retained in the public error.
/// * `target` - Requested structured target data type.
/// * `error` - Budget-aware JSON decoding failure to translate.
///
/// # Returns
///
/// Returns a limit-exceeded error for budget failures and a deserialization
/// error for JSON or I/O failures.
#[cfg(feature = "json")]
fn map_json_decode_error_from_type(
    source: DataType,
    target: DataType,
    error: JsonSerdeError<ConversionResource>,
) -> DataConversionError {
    match error {
        JsonSerdeError::Budget(error) => DataConversionError::limit_exceeded(source, target, error),
        JsonSerdeError::Json(_) | JsonSerdeError::Io(_) => DataConversionError::invalid(
            source,
            target,
            InvalidValueReason::Deserialization {
                format: DataFormat::Json,
            },
        ),
    }
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
        session: &mut ConversionSession<'_>,
    ) -> Result<Self, DataConversionError> {
        #[cfg(feature = "json")]
        let options = session.options();
        #[cfg(not(feature = "json"))]
        let _ = session;
        match source {
            DataConverter::StringMap(value) => Ok(value.as_ref().clone()),
            #[cfg(feature = "json")]
            DataConverter::String(value) => {
                let value = normalize(value, options, DataType::StringMap)?;
                check_structured_text_limit(
                    value,
                    source.data_type(),
                    DataType::StringMap,
                    options,
                )?;
                from_slice_seed_with_budget(
                    value.as_bytes(),
                    StringMapVisitor,
                    session.json_budget_mut(),
                )
                .map_err(|error| map_json_decode_error(source, DataType::StringMap, error))
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
        session: &mut ConversionSession<'_>,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::StringMap(value) => Ok(value.into_owned()),
            source => Self::convert_from(&source, session),
        }
    }
}
