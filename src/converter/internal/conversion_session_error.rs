// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Converts rendering and JSON codec failures into conversion errors.

use qubit_budget::BudgetedStringError;
#[cfg(feature = "json")]
use qubit_json::decode::JsonDecodeError;
#[cfg(feature = "json")]
use qubit_json::decode::JsonDecodeErrorKind;
#[cfg(feature = "json")]
use qubit_json::encode::JsonEncodeError;

use super::super::conversion_resource::ConversionResource;
use super::super::error::DataConversionError;
#[cfg(feature = "json")]
use super::super::error::JsonConversionDiagnostic;
#[cfg(feature = "json")]
use super::super::error::JsonConversionErrorCategory;
use crate::datatype::DataType;

/// Preserves budget, quantity, or rendering details in a conversion error.
///
/// # Parameters
///
/// * `from` - Runtime source type retained in the error.
/// * `to` - Requested target type retained in the error.
/// * `error` - Rendering or budget failure to convert.
///
/// # Returns
///
/// A conversion error preserving the failure details supported by the public
/// error model.
pub(in crate::converter) fn map_string_write_error(
    from: DataType,
    to: DataType,
    error: BudgetedStringError<ConversionResource, DataConversionError>,
) -> DataConversionError {
    match error {
        BudgetedStringError::Budget(error) => DataConversionError::limit_exceeded(from, to, error.into()),
        BudgetedStringError::Quantity { resource, source } => DataConversionError::quantity(from, to, resource, source),
        BudgetedStringError::Render(error) => error,
        BudgetedStringError::InvalidUtf8(_)
        | BudgetedStringError::LengthOverflow
        | BudgetedStringError::Allocation(_) => {
            DataConversionError::invalid(from, to, super::super::error::InvalidValueReason::OutOfRange)
        }
    }
}

/// Converts a JSON decoder failure into a conversion error.
///
/// # Parameters
///
/// * `from` - Runtime source type retained in the error.
/// * `to` - Requested target type retained in the error.
/// * `error` - JSON decoding failure to convert.
///
/// # Returns
///
/// A measured-limit error when the decoder exhausted a budget, otherwise a
/// JSON deserialization invalid-value error.
#[cfg(feature = "json")]
pub(in crate::converter) fn map_json_decode_error(
    from: DataType,
    to: DataType,
    error: JsonDecodeError<ConversionResource, u64>,
) -> DataConversionError {
    if let Some(budget) = error.budget_error().cloned() {
        return DataConversionError::measured_limit(from, to, budget);
    }
    let converted = DataConversionError::invalid(
        from,
        to,
        super::super::error::InvalidValueReason::Deserialization {
            format: super::super::error::DataFormat::Json,
        },
    );
    match json_decode_diagnostic(&error) {
        Some(diagnostic) => converted.with_json_diagnostic(diagnostic),
        None => converted,
    }
}

/// Returns a diagnostic for JSON syntax and deserialization failures.
///
/// # Parameters
///
/// * `error` - Decoder failure with a stable category and source location.
///
/// # Returns
///
/// `Some` contains the JSON error category and line/column for decoder errors;
/// `None` means the failure is a budget error represented separately.
#[cfg(feature = "json")]
fn json_decode_diagnostic(error: &JsonDecodeError<ConversionResource, u64>) -> Option<JsonConversionDiagnostic> {
    let category = match error.kind() {
        JsonDecodeErrorKind::Read => JsonConversionErrorCategory::Read,
        JsonDecodeErrorKind::EmptyInput => JsonConversionErrorCategory::EmptyInput,
        JsonDecodeErrorKind::InvalidUtf8 => JsonConversionErrorCategory::InvalidUtf8,
        JsonDecodeErrorKind::InvalidJson => JsonConversionErrorCategory::InvalidJson,
        JsonDecodeErrorKind::UnexpectedTopLevel => JsonConversionErrorCategory::UnexpectedTopLevel,
        JsonDecodeErrorKind::Deserialize => JsonConversionErrorCategory::Deserialize,
        JsonDecodeErrorKind::Budget => return None,
    };
    Some(JsonConversionDiagnostic::new(category, error.line(), error.column()))
}

/// Converts a JSON encoder failure into a conversion error.
///
/// # Parameters
///
/// * `from` - Runtime source type retained in the error.
/// * `to` - Requested target type retained in the error.
/// * `error` - JSON encoding failure to convert.
///
/// # Returns
///
/// A measured-limit error for budget failures, or a JSON serialization
/// invalid-value error for encoding failures.
#[cfg(feature = "json")]
pub(in crate::converter) fn map_json_encode_error(
    from: DataType,
    to: DataType,
    error: JsonEncodeError<ConversionResource, u64>,
) -> DataConversionError {
    if let Some(error) = error.into_budget_error() {
        DataConversionError::measured_limit(from, to, error)
    } else {
        DataConversionError::invalid(
            from,
            to,
            super::super::error::InvalidValueReason::Serialization {
                format: super::super::error::DataFormat::Json,
            },
        )
    }
}
