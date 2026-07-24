// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data List Conversion Error
//!
//! Defines errors returned by reusable batch data conversions.

use super::data_conversion_error::DataConversionError;

/// Reports the first failed item in an ordered batch conversion.
///
/// [`Self::source_index`] refers to the original, zero-based source position,
/// including positions skipped by collection policies.
/// [`Self::conversion_error`] returns the complete single-value conversion
/// error, which is also exposed as the standard error source.
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Data conversion failed at source index {source_index}: {source}")]
pub struct DataListConversionError {
    /// Zero-based index in the original source, before any items were skipped.
    source_index: usize,
    /// Original single-value conversion error.
    #[source]
    source: DataConversionError,
}

impl DataListConversionError {
    /// Creates an error for a failed item in an ordered batch conversion.
    ///
    /// # Parameters
    ///
    /// * `source_index` - Original zero-based source position.
    /// * `source` - Single-value conversion error produced for the failed item.
    ///
    /// # Returns
    ///
    /// A list conversion error containing the failed position and cause.
    #[inline(always)]
    pub const fn new(source_index: usize, source: DataConversionError) -> Self {
        Self {
            source_index,
            source,
        }
    }

    /// Returns the original zero-based source position of the failed item.
    ///
    /// # Returns
    ///
    /// The position in the original source, before any items were skipped.
    #[must_use]
    #[inline(always)]
    pub const fn source_index(&self) -> usize {
        self.source_index
    }

    /// Returns the single-value conversion error produced for the failed item.
    ///
    /// # Returns
    ///
    /// A shared reference to the underlying conversion error.
    ///
    /// ```compile_fail
    /// #![deny(unused_must_use)]
    /// use qubit_datatype::{
    ///     DataConversionError,
    ///     DataListConversionError,
    ///     DataType,
    /// };
    ///
    /// let error = DataListConversionError::new(
    ///     0,
    ///     DataConversionError::missing(DataType::String, DataType::Bool),
    /// );
    /// error.conversion_error();
    /// ```
    #[must_use = "the conversion error should be inspected"]
    #[inline(always)]
    pub const fn conversion_error(&self) -> &DataConversionError {
        &self.source
    }

    /// Consumes this error and returns its single-value conversion error.
    ///
    /// # Returns
    ///
    /// The owned underlying conversion error.
    #[inline(always)]
    pub fn into_conversion_error(self) -> DataConversionError {
        self.source
    }

    /// Consumes this error and returns its source index and conversion error.
    ///
    /// # Returns
    ///
    /// The original zero-based source position and owned conversion error.
    #[inline(always)]
    pub fn into_parts(self) -> (usize, DataConversionError) {
        (self.source_index, self.source)
    }
}
