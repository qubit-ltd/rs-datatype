// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Structured JSON and string-map conversion implementations.

use std::collections::HashMap;

use qubit_budget::MeasuredBudgetError;
use qubit_budget::json::JsonMeasurement;
#[cfg(feature = "json")]
use serde_json::Value;

use super::DataConverter;
#[cfg(feature = "json")]
use super::internal::StringMapVisitor;
#[cfg(feature = "json")]
use super::internal::sorted_string_map_entries;
#[cfg(feature = "json")]
use super::string_source::normalize;
use crate::converter::BuiltinDataConversionTarget;
use crate::converter::ConversionContext;
use crate::converter::ConversionResource;
use crate::converter::ConversionSession;
use crate::converter::DataConversionError;
#[cfg(any(feature = "json", feature = "url"))]
use crate::converter::StructuredConversionLimits;
use crate::datatype::DataType;

/// Accounts one string map and its payload against a conversion session.
pub(super) fn account_string_map_structure(
    value: &HashMap<String, String>,
    depth: usize,
    session: &mut ConversionSession<'_>,
) -> Result<(), MeasuredBudgetError<ConversionResource, u64>> {
    let mut transaction = session.structured_transaction();
    transaction.try_admit(JsonMeasurement::Object {
        depth,
        entries: value.len(),
    })?;
    let value_depth = depth.saturating_add(1);
    for (key, value) in value {
        transaction.try_admit(JsonMeasurement::Key { bytes: key.len() })?;
        transaction.try_admit(JsonMeasurement::String {
            depth: value_depth,
            bytes: value.len(),
        })?;
    }
    transaction.commit()
}

/// Accounts a materialized JSON value through rs-budget's canonical traversal.
#[cfg(feature = "json")]
pub(super) fn account_json_structure(
    value: &Value,
    source: DataType,
    target: DataType,
    session: &mut ConversionSession<'_>,
) -> Result<(), DataConversionError> {
    session.account_json_value(source, target, value)
}

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
    limits: &StructuredConversionLimits,
) -> Result<(), DataConversionError> {
    limits.text().check(value).map_err(|error| match error {
        MeasuredBudgetError::Budget(error) => DataConversionError::limit_exceeded(from, to, error),
        MeasuredBudgetError::Quantity { resource, source } => DataConversionError::quantity(from, to, resource, source),
    })
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
impl BuiltinDataConversionTarget for Value {
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
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        let policy = context.policy();
        let limits = context.limits();
        match source {
            DataConverter::Json(value) => Ok(value.as_ref().clone()),
            DataConverter::String(value) => {
                let value = normalize(value, policy, DataType::Json)?;
                check_structured_text_limit(value, source.data_type(), DataType::Json, limits.structured())?;
                let decoded: Value = context.decode_json(value.as_bytes())?;
                Ok(decoded)
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
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::Json(value) => Ok(value.into_owned()),
            DataConverter::StringMap(value) => Ok(string_map_into_json(value.into_owned())),
            source => Self::convert_from(&source, context),
        }
    }
}

impl BuiltinDataConversionTarget for HashMap<String, String> {
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
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        #[cfg(feature = "json")]
        let policy = context.policy();
        #[cfg(feature = "json")]
        let limits = context.limits();
        #[cfg(not(feature = "json"))]
        let _ = context;
        match source {
            DataConverter::StringMap(value) => Ok(value.as_ref().clone()),
            #[cfg(feature = "json")]
            DataConverter::String(value) => {
                let value = normalize(value, policy, DataType::StringMap)?;
                check_structured_text_limit(value, source.data_type(), DataType::StringMap, limits.structured())?;
                let decoded = context.decode_json_seed(StringMapVisitor, value.as_bytes())?;
                Ok(decoded)
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
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        match source {
            DataConverter::StringMap(value) => Ok(value.into_owned()),
            source => Self::convert_from(&source, context),
        }
    }
}
