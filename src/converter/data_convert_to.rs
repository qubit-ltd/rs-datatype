// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Conversion Trait
//!
//! Defines the target-type conversion trait implemented by `DataConverter`.

use super::error::DataConversionError;
use super::options::DataConversionOptions;

/// Defines conversion from a [`super::DataConverter`] to target type `T`.
///
/// The crate implements this trait once per supported target family. Most
/// callers should use [`super::DataConverter::to`] or
/// [`super::DataConverter::to_with`]; this trait is primarily useful in generic
/// bounds and when adding an in-crate target implementation. It intentionally
/// keeps the runtime [`crate::DataType`] independent from converter objects.
///
/// # Type Parameters
///
/// * `T` - Concrete Rust target type returned after conversion.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{DataConvertTo, DataConverter};
///
/// fn parse<'a, T>(source: &DataConverter<'a>) -> T
/// where
///     DataConverter<'a>: DataConvertTo<T>,
/// {
///     source.to().expect("example input should convert")
/// }
///
/// assert_eq!(parse::<u16>(&DataConverter::from("8080")), 8080);
/// ```
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
    fn convert(
        &self,
        options: &DataConversionOptions,
    ) -> Result<T, DataConversionError>;
}
