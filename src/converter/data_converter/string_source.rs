// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Shared normalization for textual converter sources.

use crate::converter::{
    DataConversionError,
    DataConversionOptions,
    InvalidValueReason,
    StringNormalizationError,
};
use crate::datatype::DataType;

/// Normalizes a textual source and attaches target context to policy errors.
///
/// `value` is normalized with the string group in `options`; `to` is attached
/// to any resulting error. The returned slice borrows `value` and may exclude
/// surrounding whitespace. Missing and rejected blank outcomes become the
/// corresponding [`DataConversionError`] variants.
pub(super) fn normalize<'a>(
    value: &'a str,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<&'a str, DataConversionError> {
    options
        .string
        .normalize(value)
        .map_err(|error| match error {
            StringNormalizationError::Missing => {
                DataConversionError::missing(DataType::String, to)
            }
            StringNormalizationError::BlankRejected => {
                DataConversionError::invalid(
                    DataType::String,
                    to,
                    InvalidValueReason::BlankRejected,
                )
            }
        })
}
