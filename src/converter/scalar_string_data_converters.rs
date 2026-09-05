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

use super::conversion_session::ConversionSession;
use super::data_conversion_target::DataConversionTarget;
use super::error::DataConversionError;
use super::error::DataListConversionError;
use super::options::ConversionLimits;
use super::options::ConversionPolicy;

/// Converts a scalar string as a configurable collection source.
///
/// This type applies [`ConversionPolicy::collection`] when converting a
/// scalar string to a vector or first value. It keeps scalar strings such as
/// `"1,2,3"` distinct from already-materialized string collections such as
/// `["1", "2", "3"]`.
///
/// # Type Parameters
///
/// * `'a` - Lifetime of the borrowed scalar string source.
///
/// # Examples
///
/// ```
/// use qubit_datatype::CollectionConversionPolicy;
/// use qubit_datatype::ConversionLimits;
/// use qubit_datatype::ConversionPolicy;
/// use qubit_datatype::ScalarStringDataConverters;
///
/// let collection = CollectionConversionPolicy::builder()
///     .split_scalar_strings(true)
///     .delimiters([','])
///     .build();
/// let policy = ConversionPolicy::builder().collection_policy(collection).build();
/// let limits = ConversionLimits::default();
/// let values = ScalarStringDataConverters::from("1,2,3")
///     .to_vec_with::<u32>(&policy, &limits)
///     .unwrap();
/// assert_eq!(values, vec![1, 2, 3]);
/// ```
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn to_vec<T>(self) -> Result<Vec<T>, DataListConversionError>
    where
        T: DataConversionTarget,
    {
        self.to_vec_with(ConversionPolicy::default_ref(), ConversionLimits::default_ref())
    }

    /// Converts the scalar string to a vector using the specified policy and
    /// limits.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target element type.
    ///
    /// # Parameters
    ///
    /// * `policy` - Semantic conversion policy used for scalar string
    ///   normalization, splitting, and item conversion.
    /// * `limits` - Value-level and operation-level resource limits.
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
        policy: &'b ConversionPolicy,
        limits: &'b ConversionLimits,
    ) -> Result<Vec<T>, DataListConversionError>
    where
        'a: 'b,
        T: DataConversionTarget,
    {
        let mut session = ConversionSession::new(policy, limits);
        self.to_vec_in(&mut session)
    }

    /// Converts the scalar string using an existing conversion session.
    pub fn to_vec_in<T>(self, session: &mut ConversionSession<'_>) -> Result<Vec<T>, DataListConversionError>
    where
        T: DataConversionTarget,
    {
        let mut source = session
            .admit_scalar_string_source(self.source)
            .map_err(|source| DataListConversionError::new(0, source))?;
        let mut converted = Vec::new();
        while let Some(item) = source.next_item_for(T::DATA_TYPE) {
            let admitted = item?;
            let source_index = admitted.source_index();
            let value = admitted
                .convert::<T>()
                .map_err(|error| DataListConversionError::new(source_index, error))?;
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
    /// Returns the converted first item. Later scalar items are neither split,
    /// converted, nor validated.
    ///
    /// # Errors
    ///
    /// Returns a missing-value [`DataConversionError`] when normalization
    /// treats the scalar as missing, an empty-collection error when
    /// splitting yields no retained item, or the underlying conversion error.
    #[inline(always)]
    pub fn to_first<T>(self) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        self.to_first_with(ConversionPolicy::default_ref(), ConversionLimits::default_ref())
    }

    /// Converts the first scalar string item using an explicit policy and
    /// limits.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Target type.
    ///
    /// # Parameters
    ///
    /// * `policy` - Semantic conversion policy used for parsing.
    /// * `limits` - Value-level and operation-level resource limits.
    ///
    /// # Returns
    ///
    /// Returns the converted first item. Later scalar items are neither split,
    /// converted, nor validated.
    ///
    /// # Errors
    ///
    /// Returns a missing-value [`DataConversionError`] when normalization
    /// treats the scalar as missing, an empty-collection error when
    /// splitting yields no retained item, or the underlying conversion error.
    #[inline]
    pub fn to_first_with<'b, T>(
        self,
        policy: &'b ConversionPolicy,
        limits: &'b ConversionLimits,
    ) -> Result<T, DataConversionError>
    where
        'a: 'b,
        T: DataConversionTarget,
    {
        let mut session = ConversionSession::new(policy, limits);
        self.to_first_in(&mut session)
    }

    /// Converts the first scalar item using an existing conversion session.
    #[inline]
    pub fn to_first_in<T>(self, session: &mut ConversionSession<'_>) -> Result<T, DataConversionError>
    where
        T: DataConversionTarget,
    {
        let mut source = session.admit_scalar_string_source(self.source)?;
        let first = source
            .next_item_for(T::DATA_TYPE)
            .ok_or(DataConversionError::empty_collection(T::DATA_TYPE))?
            .map_err(DataListConversionError::into_conversion_error)?;
        first.convert::<T>()
    }
}

impl<'a> From<&'a str> for ScalarStringDataConverters<'a> {
    /// Creates a scalar string converter from a string slice.
    ///
    /// # Parameters
    ///
    /// * `source` - Scalar string to borrow.
    ///
    /// # Returns
    ///
    /// A converter borrowing the source for its lifetime.
    #[inline(always)]
    fn from(source: &'a str) -> Self {
        Self::new(source)
    }
}
