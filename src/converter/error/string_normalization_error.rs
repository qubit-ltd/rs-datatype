// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # String Normalization Error
//!
//! Defines policy outcomes produced before target-specific parsing.

use crate::datatype::DataType;

use super::data_conversion_error::DataConversionError;
use super::invalid_value_reason::InvalidValueReason;

/// Error returned while normalizing a string source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum StringNormalizationError {
    /// The blank source is treated as a missing value.
    #[error("missing string value")]
    Missing,
    /// The blank source is explicitly rejected.
    #[error("blank string rejected")]
    BlankRejected,
}

impl StringNormalizationError {
    /// Reports whether normalization treated the source as missing.
    ///
    /// # Returns
    ///
    /// `true` only for [`Self::Missing`]; otherwise, `false`.
    #[inline(always)]
    pub const fn is_missing(&self) -> bool {
        matches!(self, Self::Missing)
    }

    /// Converts this outcome into a target-aware data conversion error.
    ///
    /// # Parameters
    ///
    /// * `to` - Requested target data type.
    ///
    /// # Returns
    ///
    /// A missing-value or rejected-blank conversion error whose source type is
    /// [`DataType::String`] and whose target type is `to`.
    #[inline(always)]
    pub const fn into_data_conversion_error(self, to: DataType) -> DataConversionError {
        match self {
            Self::Missing => DataConversionError::missing(DataType::String, to),
            Self::BlankRejected => DataConversionError::invalid(
                DataType::String,
                to,
                InvalidValueReason::BlankRejected,
            ),
        }
    }
}
