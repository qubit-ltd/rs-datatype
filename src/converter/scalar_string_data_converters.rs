// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Scalar String Data Conversion
//!
//! Provides conversion of a single scalar string into collection values.

use super::data_conversion_target::DataConversionTarget;
use super::data_converter::DataConverter;
use super::error::{
    DataConversionError,
    DataListConversionError,
    InvalidValueReason,
    StringNormalizationError,
};
use super::options::DataConversionOptions;
use crate::datatype::{
    DataType,
    DataTypeOf,
};

/// Maps string normalization outcomes to the requested element type.
///
/// The error does not include the source text. Missing and rejected blanks are
/// distinguished so callers can apply stable policy-specific handling.
fn normalization_error<T: DataTypeOf>(
    error: StringNormalizationError,
) -> DataConversionError {
    match error {
        StringNormalizationError::Missing => DataConversionError::Missing {
            from: DataType::String,
            to: T::DATA_TYPE,
        },
        StringNormalizationError::BlankRejected => {
            DataConversionError::InvalidValue {
                from: DataType::String,
                to: T::DATA_TYPE,
                reason: InvalidValueReason::BlankRejected,
            }
        }
    }
}

/// Converts a scalar string as a configurable collection source.
///
/// This type applies [`DataConversionOptions::collection`] when converting a
/// scalar string to a vector or first value. It keeps scalar strings such as
/// `"1,2,3"` distinct from already-materialized string collections such as
/// `["1", "2", "3"]`.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct ScalarStringDataConverters<'a> {
    /// The scalar string source.
    source: &'a str,
}

impl<'a> ScalarStringDataConverters<'a> {
    /// Creates a scalar string converter.
    ///
    /// # Parameters
    ///
    /// * `source` - Scalar string source.
    ///
    /// # Returns
    ///
    /// Returns a converter that can split the scalar source when requested by
    /// collection options.
    #[inline]
    pub const fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Converts the scalar string to a vector using default options.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target element type.
    ///
    /// # Returns
    ///
    /// Returns converted values.
    ///
    /// # Errors
    ///
    /// Returns [`DataListConversionError`] when the scalar string cannot be
    /// normalized, split, or converted to the requested element type.
    pub fn to_vec<T>(self) -> Result<Vec<T>, DataListConversionError>
    where
        T: DataTypeOf,
        T: DataConversionTarget,
    {
        self.to_vec_with(DataConversionOptions::default_ref())
    }

    /// Converts the scalar string to a vector using options.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target element type.
    ///
    /// # Parameters
    ///
    /// * `options` - Conversion options used for scalar string normalization,
    ///   splitting, and item conversion.
    ///
    /// # Returns
    ///
    /// Returns converted values.
    ///
    /// # Errors
    ///
    /// Returns [`DataListConversionError`] when the scalar string cannot be
    /// normalized, split, or converted to the requested element type.
    pub fn to_vec_with<'b, T>(
        self,
        options: &'b DataConversionOptions,
    ) -> Result<Vec<T>, DataListConversionError>
    where
        'a: 'b,
        T: DataTypeOf,
        T: DataConversionTarget,
    {
        let text = match options.string.normalize(self.source) {
            Ok(text) => text,
            Err(error) => {
                return Err(DataListConversionError {
                    source_index: 0,
                    source: normalization_error::<T>(error),
                });
            }
        };
        let items = options.collection.scalar_items(text);
        let (capacity, _) = items.size_hint();
        let mut converted = Vec::with_capacity(capacity);
        for item in items {
            let item = item.map_err(|error| DataListConversionError {
                source_index: error.source_index,
                source: DataConversionError::InvalidValue {
                    from: DataType::String,
                    to: T::DATA_TYPE,
                    reason: InvalidValueReason::BlankRejected,
                },
            })?;
            let value = match DataConverter::from(item.value).to_with(options) {
                Ok(value) => value,
                Err(source) => {
                    return Err(DataListConversionError {
                        source_index: item.source_index,
                        source,
                    });
                }
            };
            converted.push(value);
        }
        Ok(converted)
    }

    /// Converts the first scalar string item using default options.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target type.
    ///
    /// # Returns
    ///
    /// Returns the converted first item.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError::Missing`] when normalization treats the
    /// scalar as missing, [`DataConversionError::EmptyCollection`] when
    /// splitting yields no retained item, or the underlying conversion error.
    pub fn to_first<T>(self) -> Result<T, DataConversionError>
    where
        T: DataTypeOf,
        T: DataConversionTarget,
    {
        self.to_first_with(DataConversionOptions::default_ref())
    }

    /// Converts the first scalar string item using options.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target type.
    ///
    /// # Parameters
    ///
    /// * `options` - Conversion options used for parsing.
    ///
    /// # Returns
    ///
    /// Returns the converted first item.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError::Missing`] when normalization treats the
    /// scalar as missing, [`DataConversionError::EmptyCollection`] when
    /// splitting yields no retained item, or the underlying conversion error.
    pub fn to_first_with<'b, T>(
        self,
        options: &'b DataConversionOptions,
    ) -> Result<T, DataConversionError>
    where
        'a: 'b,
        T: DataTypeOf,
        T: DataConversionTarget,
    {
        let text = options
            .string
            .normalize(self.source)
            .map_err(normalization_error::<T>)?;
        let first = options
            .collection
            .scalar_items(text)
            .next()
            .ok_or(DataConversionError::EmptyCollection { to: T::DATA_TYPE })?
            .map_err(|_| DataConversionError::InvalidValue {
                from: DataType::String,
                to: T::DATA_TYPE,
                reason: InvalidValueReason::BlankRejected,
            })?;
        DataConverter::from(first.value).to_with::<T>(options)
    }
}

impl<'a> From<&'a str> for ScalarStringDataConverters<'a> {
    /// Creates a scalar string converter from a string slice.
    #[inline]
    fn from(source: &'a str) -> Self {
        Self::new(source)
    }
}
