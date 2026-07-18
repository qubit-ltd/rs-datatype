// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Conversion Error
//!
//! Defines errors returned by reusable data conversions.

use crate::datatype::DataType;

use super::data_conversion_error_kind::DataConversionErrorKind;
use super::internal::DataConversionErrorInner;
use super::invalid_value_reason::InvalidValueReason;

/// Describes why a source value could not be converted.
///
/// The error exposes stable classification and context through accessors while
/// keeping its variant representation private. Source values are deliberately
/// not retained or formatted, which makes these errors safe to surface for
/// secrets such as environment variables.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{
///     DataConversionErrorKind, DataConverter, InvalidValueReason,
/// };
///
/// let error = DataConverter::from("not-a-number").to::<u32>().unwrap_err();
/// assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
/// assert!(matches!(
///     error.reason(),
///     Some(InvalidValueReason::InvalidSyntax { .. }),
/// ));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{inner}")]
pub struct DataConversionError {
    /// Variant-specific conversion error details.
    inner: DataConversionErrorInner,
}

impl DataConversionError {
    /// Creates an error for a missing source value.
    ///
    /// # Parameters
    ///
    /// * `from` - Declared source data type.
    /// * `to` - Requested target data type.
    ///
    /// # Returns
    ///
    /// A missing-value conversion error.
    #[inline(always)]
    pub const fn missing(from: DataType, to: DataType) -> Self {
        Self {
            inner: DataConversionErrorInner::Missing { from, to },
        }
    }

    /// Creates an error for a first-value conversion from an empty collection.
    ///
    /// # Parameters
    ///
    /// * `to` - Requested target data type.
    ///
    /// # Returns
    ///
    /// An empty-collection conversion error.
    #[inline(always)]
    pub const fn empty_collection(to: DataType) -> Self {
        Self {
            inner: DataConversionErrorInner::EmptyCollection { to },
        }
    }

    /// Creates an error for an unsupported source and target type pair.
    ///
    /// # Parameters
    ///
    /// * `from` - Declared source data type.
    /// * `to` - Requested target data type.
    ///
    /// # Returns
    ///
    /// An unsupported-conversion error.
    #[inline(always)]
    pub const fn unsupported(from: DataType, to: DataType) -> Self {
        Self {
            inner: DataConversionErrorInner::Unsupported { from, to },
        }
    }

    /// Creates an error for an invalid value in a supported conversion.
    ///
    /// # Parameters
    ///
    /// * `from` - Declared source data type.
    /// * `to` - Requested target data type.
    /// * `reason` - Stable, value-free rejection reason.
    ///
    /// # Returns
    ///
    /// An invalid-value conversion error.
    #[inline(always)]
    pub const fn invalid(
        from: DataType,
        to: DataType,
        reason: InvalidValueReason,
    ) -> Self {
        Self {
            inner: DataConversionErrorInner::InvalidValue { from, to, reason },
        }
    }

    /// Returns the stable classification of this error.
    ///
    /// # Returns
    ///
    /// The conversion error kind.
    #[inline(always)]
    pub const fn kind(&self) -> DataConversionErrorKind {
        match &self.inner {
            DataConversionErrorInner::Missing { .. } => {
                DataConversionErrorKind::Missing
            }
            DataConversionErrorInner::EmptyCollection { .. } => {
                DataConversionErrorKind::EmptyCollection
            }
            DataConversionErrorInner::Unsupported { .. } => {
                DataConversionErrorKind::Unsupported
            }
            DataConversionErrorInner::InvalidValue { .. } => {
                DataConversionErrorKind::InvalidValue
            }
        }
    }

    /// Reports whether this error represents a missing source value.
    ///
    /// # Returns
    ///
    /// `true` only for a missing-value error; otherwise, `false`.
    #[inline(always)]
    pub const fn is_missing(&self) -> bool {
        matches!(&self.inner, DataConversionErrorInner::Missing { .. })
    }

    /// Returns the declared source data type when one exists.
    ///
    /// # Returns
    ///
    /// `Some` with the source type for value-based errors, or `None` for an
    /// empty-collection error because no source value exists.
    #[inline(always)]
    pub const fn from_type(&self) -> Option<DataType> {
        match &self.inner {
            DataConversionErrorInner::Missing { from, .. }
            | DataConversionErrorInner::Unsupported { from, .. }
            | DataConversionErrorInner::InvalidValue { from, .. } => {
                Some(*from)
            }
            DataConversionErrorInner::EmptyCollection { .. } => None,
        }
    }

    /// Returns the requested target data type.
    ///
    /// # Returns
    ///
    /// The target type recorded by this error.
    #[inline(always)]
    pub const fn to_type(&self) -> DataType {
        match &self.inner {
            DataConversionErrorInner::Missing { to, .. }
            | DataConversionErrorInner::Unsupported { to, .. }
            | DataConversionErrorInner::InvalidValue { to, .. }
            | DataConversionErrorInner::EmptyCollection { to } => *to,
        }
    }

    /// Returns the invalid-value reason when one exists.
    ///
    /// # Returns
    ///
    /// `Some` with the rejection reason for an invalid-value error, or `None`
    /// for all other error kinds.
    #[inline(always)]
    pub const fn reason(&self) -> Option<&InvalidValueReason> {
        match &self.inner {
            DataConversionErrorInner::InvalidValue { reason, .. } => {
                Some(reason)
            }
            _ => None,
        }
    }
}
