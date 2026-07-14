// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Reusable Batch Data Conversion
//!
//! Provides `DataConverters`, a lightweight iterator adapter used to convert
//! batches of common runtime values with the single-value [`DataConverter`]
//! rules.

use super::data_convert_to::DataConvertTo;
use super::data_converter::DataConverter;
use super::error::{
    DataConversionError,
    DataListConversionError,
};
use super::options::DataConversionOptions;
use crate::datatype::DataTypeOf;

/// A lightweight adapter for converting batches of source values.
///
/// `DataConverters` stores an iterator and converts each item through
/// [`DataConverter`]. Borrowed inputs such as `&Vec<T>` and `&[T]` are
/// converted by reference and do not clone the source collection.
///
/// # Examples
///
/// ```
/// use qubit_datatype::converter::{
///     DataConverters,
///     DataListConversionError,
/// };
///
/// fn parse_ports(
///     values: &[String],
/// ) -> Result<Vec<u16>, DataListConversionError> {
///     DataConverters::from(values).to_vec()
/// }
///
/// let values = vec![String::from("8080"), String::from("9090")];
/// let ports = parse_ports(&values).expect("all port values should convert");
///
/// assert_eq!(ports, vec![8080, 9090]);
/// assert_eq!(values, vec![String::from("8080"), String::from("9090")]);
/// ```
#[derive(Debug, Clone)]
pub struct DataConverters<I> {
    /// The iterator of source values.
    sources: I,
}

impl<I> DataConverters<I>
where
    I: Iterator,
{
    /// Creates a batch converter from an iterator.
    ///
    /// # Parameters
    ///
    /// * `sources` - Iterator that yields values convertible to
    ///   [`DataConverter`].
    ///
    /// # Returns
    ///
    /// Returns a batch converter that consumes the iterator when conversion is
    /// requested.
    #[inline]
    pub fn from_iterator(sources: I) -> Self {
        Self { sources }
    }
}

impl<I> DataConverters<I>
where
    I: Iterator,
{
    /// Converts every source item to the requested target type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target element type.
    ///
    /// # Returns
    ///
    /// Returns converted values in source order. Empty sources return an empty
    /// vector.
    ///
    /// # Errors
    ///
    /// Returns [`DataListConversionError`] with the zero-based failing index
    /// and the original [`DataConversionError`] when any element fails
    /// conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_datatype::converter::DataConverters;
    ///
    /// let values = ["1", "0", "true", "FALSE"];
    /// let flags: Vec<bool> = DataConverters::from_iterator(values.iter().copied())
    ///     .to_vec()
    ///     .expect("all flag values should convert");
    ///
    /// assert_eq!(flags, vec![true, false, true, false]);
    /// ```
    pub fn to_vec<'a, T>(self) -> Result<Vec<T>, DataListConversionError>
    where
        I::Item: Into<DataConverter<'a>>,
        DataConverter<'a>: DataConvertTo<T>,
    {
        self.to_vec_with(DataConversionOptions::default_ref())
    }

    /// Converts every source item to the requested target type using options.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target element type.
    ///
    /// # Parameters
    ///
    /// * `options` - Conversion options used for each element.
    ///
    /// # Returns
    ///
    /// Returns converted values in source order. Empty sources return an empty
    /// vector.
    ///
    /// # Errors
    ///
    /// Returns [`DataListConversionError`] with the zero-based failing index
    /// and the original [`DataConversionError`] when any element fails
    /// conversion.
    pub fn to_vec_with<'a, T>(
        self,
        options: &DataConversionOptions,
    ) -> Result<Vec<T>, DataListConversionError>
    where
        I::Item: Into<DataConverter<'a>>,
        DataConverter<'a>: DataConvertTo<T>,
    {
        let sources = self.sources;
        let (capacity, _) = sources.size_hint();
        let mut converted = Vec::with_capacity(capacity);
        for (index, source) in sources.enumerate() {
            let value = match source.into().to_with::<T>(options) {
                Ok(value) => value,
                Err(source) => {
                    return Err(DataListConversionError {
                        source_index: index,
                        source,
                    });
                }
            };
            converted.push(value);
        }
        Ok(converted)
    }

    /// Converts the first source item to the requested target type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target type.
    ///
    /// # Returns
    ///
    /// Returns the converted first value.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError::Missing`] when the source iterator is
    /// empty. Because an empty generic iterator has no source value whose type
    /// can be inferred, that error uses the requested target type for both its
    /// `from` and `to` fields. Returns the original single-value conversion
    /// error when the first element cannot be converted.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_datatype::converter::DataConverters;
    ///
    /// let values = vec![String::from("42"), String::from("100")];
    /// let first: i32 = DataConverters::from(&values)
    ///     .to_first()
    ///     .expect("first value should convert to i32");
    ///
    /// assert_eq!(first, 42);
    /// ```
    pub fn to_first<'a, T>(self) -> Result<T, DataConversionError>
    where
        T: DataTypeOf,
        I::Item: Into<DataConverter<'a>>,
        DataConverter<'a>: DataConvertTo<T>,
    {
        self.to_first_with(DataConversionOptions::default_ref())
    }

    /// Converts the first source item to the requested target type using
    /// options.
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
    /// Returns the converted first value.
    ///
    /// # Errors
    ///
    /// Returns [`DataConversionError::Missing`] when the source iterator is
    /// empty. Because an empty generic iterator has no source value whose type
    /// can be inferred, that error uses the requested target type for both its
    /// `from` and `to` fields. Returns the original conversion error when the
    /// first element cannot be converted.
    pub fn to_first_with<'a, T>(
        self,
        options: &DataConversionOptions,
    ) -> Result<T, DataConversionError>
    where
        T: DataTypeOf,
        I::Item: Into<DataConverter<'a>>,
        DataConverter<'a>: DataConvertTo<T>,
    {
        let mut sources = self.sources;
        match sources.next() {
            Some(source) => source.into().to_with::<T>(options),
            None => Err(DataConversionError::Missing {
                // An empty generic collection has no source value whose type
                // could be inferred, so the requested type is used for both
                // sides of the missing-value relation.
                from: T::DATA_TYPE,
                to: T::DATA_TYPE,
            }),
        }
    }
}

impl<I> DataConverters<I>
where
    I: ExactSizeIterator,
{
    /// Returns the number of source items remaining in this converter.
    ///
    /// # Returns
    ///
    /// Returns the exact number of items that can still be converted.
    #[inline]
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Returns whether this converter has no source items remaining.
    ///
    /// # Returns
    ///
    /// Returns `true` when [`Self::len`] is zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.sources.len() == 0
    }
}

impl<'a, V> From<&'a [V]> for DataConverters<std::slice::Iter<'a, V>> {
    /// Creates a batch converter from a borrowed slice.
    #[inline]
    fn from(values: &'a [V]) -> Self {
        Self::from_iterator(values.iter())
    }
}

impl<'a, V> From<&'a Vec<V>> for DataConverters<std::slice::Iter<'a, V>> {
    /// Creates a batch converter from a borrowed vector.
    #[inline]
    fn from(values: &'a Vec<V>) -> Self {
        Self::from(values.as_slice())
    }
}

impl<V> From<Vec<V>> for DataConverters<std::vec::IntoIter<V>> {
    /// Creates a batch converter from an owned vector.
    #[inline]
    fn from(values: Vec<V>) -> Self {
        Self::from_iterator(values.into_iter())
    }
}
