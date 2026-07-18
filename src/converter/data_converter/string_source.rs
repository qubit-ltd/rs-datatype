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
};
use crate::datatype::DataType;

/// Normalizes a textual source and attaches target context to policy errors.
///
/// # Parameters
///
/// * `value` - Textual source to normalize.
/// * `options` - String normalization policies.
/// * `to` - Target type attached to normalization errors.
///
/// # Returns
///
/// A slice borrowing `value`, possibly without surrounding whitespace.
///
/// # Errors
///
/// Returns the corresponding [`DataConversionError`] for missing or rejected
/// blank input.
#[inline(always)]
pub(super) fn normalize<'a>(
    value: &'a str,
    options: &DataConversionOptions,
    to: DataType,
) -> Result<&'a str, DataConversionError> {
    options
        .string()
        .normalize(value)
        .map_err(|error| error.into_data_conversion_error(to))
}
