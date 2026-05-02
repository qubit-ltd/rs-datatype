/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Scalar String Data Conversion
//!
//! Provides conversion of a single scalar string into collection values.
//!
//! # Author
//!
//! Haixing Hu

use super::data_conversion_error::DataConversionError;
use super::data_conversion_options::DataConversionOptions;
use super::data_conversion_result::DataConversionResult;
use super::data_convert_to::DataConvertTo;
use super::data_converter::DataConverter;
use super::data_converters::DataConverters;
use super::data_list_conversion_error::DataListConversionError;
use super::data_list_conversion_result::DataListConversionResult;

/// Converts a scalar string as a configurable collection source.
///
/// This type applies [`DataConversionOptions::collection`] when converting a
/// scalar string to a vector or first value. It keeps scalar strings such as
/// `"1,2,3"` distinct from already-materialized string collections such as
/// `["1", "2", "3"]`.
///
/// # Author
///
/// Haixing Hu
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
    pub fn to_vec<T>(self) -> DataListConversionResult<Vec<T>>
    where
        DataConverter<'a>: DataConvertTo<T>,
    {
        self.to_vec_with(&DataConversionOptions::default())
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
    pub fn to_vec_with<T>(self, options: &DataConversionOptions) -> DataListConversionResult<Vec<T>>
    where
        DataConverter<'a>: DataConvertTo<T>,
    {
        let text = match DataConverter::from(self.source).to_with::<String>(options) {
            Ok(text) => text,
            Err(source) => {
                return Err(DataListConversionError { index: 0, source });
            }
        };
        let items = options.collection.scalar_items(&text)?;
        DataConverters::from(items).to_vec_with(options)
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
    /// Returns [`DataConversionError::NoValue`] when splitting yields no items,
    /// or the underlying conversion error.
    pub fn to_first<T>(self) -> DataConversionResult<T>
    where
        DataConverter<'a>: DataConvertTo<T>,
    {
        self.to_first_with(&DataConversionOptions::default())
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
    /// Returns [`DataConversionError::NoValue`] when splitting yields no items,
    /// or the underlying conversion error.
    pub fn to_first_with<T>(self, options: &DataConversionOptions) -> DataConversionResult<T>
    where
        DataConverter<'a>: DataConvertTo<T>,
    {
        let text = DataConverter::from(self.source).to_with::<String>(options)?;
        let mut items = match options.collection.scalar_items(&text) {
            Ok(items) => items,
            Err(error) => return Err(error.source),
        };
        let first = items.drain(..).next().ok_or(DataConversionError::NoValue)?;
        DataConverter::from(first).to_with::<T>(options)
    }
}

impl<'a> From<&'a str> for ScalarStringDataConverters<'a> {
    /// Creates a scalar string converter from a string slice.
    #[inline]
    fn from(source: &'a str) -> Self {
        Self::new(source)
    }
}
