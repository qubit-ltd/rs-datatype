/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Data Conversion Trait
//!
//! Defines the target-type conversion trait implemented by `DataConverter`.
//!

use super::data_conversion_options::DataConversionOptions;
use super::data_conversion_result::DataConversionResult;

/// Trait implemented by `DataConverter` for each supported target type.
pub trait DataConvertTo<T> {
    /// Converts the source value to `T`.
    ///
    /// # Parameters
    ///
    /// * `options` - Conversion options used for parsing source content.
    ///
    /// # Returns
    ///
    /// Returns the converted target value.
    ///
    /// # Errors
    ///
    /// Returns a [`super::DataConversionError`] when the conversion is
    /// unsupported, the source value is empty, or the source content is invalid
    /// for `T`.
    fn convert(&self, options: &DataConversionOptions) -> DataConversionResult<T>;
}
