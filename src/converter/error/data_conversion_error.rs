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

use qubit_budget::BudgetError;
use qubit_budget::MeasuredBudgetError;
use qubit_budget::Observation;
use qubit_budget::QuantityConversionError;

use super::super::conversion_resource::ConversionResource;
use super::data_conversion_error_kind::DataConversionErrorKind;
use super::internal::DataConversionErrorInner;
use super::invalid_value_reason::InvalidValueReason;
use crate::datatype::DataType;

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
#[must_use]
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
    pub const fn invalid(from: DataType, to: DataType, reason: InvalidValueReason) -> Self {
        Self {
            inner: DataConversionErrorInner::InvalidValue { from, to, reason },
        }
    }

    /// Creates an error for a conversion resource limit violation.
    ///
    /// # Parameters
    ///
    /// * `from` - Declared source data type.
    /// * `to` - Requested target data type.
    /// * `source` - Complete point or cumulative budget failure.
    ///
    /// # Returns
    ///
    /// A resource-limit conversion error.
    #[inline(always)]
    pub(crate) fn limit_exceeded(from: DataType, to: DataType, source: BudgetError<ConversionResource, u64>) -> Self {
        Self {
            inner: DataConversionErrorInner::LimitExceeded { from, to, source },
        }
    }

    /// Creates an error for an unrepresentable native resource measurement.
    ///
    /// # Parameters
    ///
    /// * `from` - Declared source data type.
    /// * `to` - Requested target data type.
    /// * `resource` - Resource whose measurement could not fit the quantity.
    /// * `source` - Exact native quantity conversion failure.
    ///
    /// # Returns
    ///
    /// A conversion measurement error distinct from a budget-limit violation.
    #[inline(always)]
    pub(crate) fn quantity(
        from: DataType,
        to: DataType,
        resource: ConversionResource,
        source: QuantityConversionError,
    ) -> Self {
        Self {
            inner: DataConversionErrorInner::Quantity {
                from,
                to,
                resource,
                source,
            },
        }
    }

    /// Maps native measurement accounting into a public conversion error.
    #[inline(always)]
    pub(crate) fn measured_limit(
        from: DataType,
        to: DataType,
        error: MeasuredBudgetError<ConversionResource, u64>,
    ) -> Self {
        match error {
            MeasuredBudgetError::Budget(error) => Self::limit_exceeded(from, to, error),
            MeasuredBudgetError::Quantity { resource, source } => Self::quantity(from, to, resource, source),
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
            DataConversionErrorInner::Missing { .. } => DataConversionErrorKind::Missing,
            DataConversionErrorInner::EmptyCollection { .. } => DataConversionErrorKind::EmptyCollection,
            DataConversionErrorInner::Unsupported { .. } => DataConversionErrorKind::Unsupported,
            DataConversionErrorInner::InvalidValue { .. } => DataConversionErrorKind::InvalidValue,
            DataConversionErrorInner::LimitExceeded { .. } => DataConversionErrorKind::LimitExceeded,
            DataConversionErrorInner::Quantity { .. } => DataConversionErrorKind::Quantity,
        }
    }

    /// Reports whether this error represents a missing source value.
    ///
    /// # Returns
    ///
    /// `true` only for a missing-value error; otherwise, `false`.
    ///
    /// ```compile_fail
    /// #![deny(unused_must_use)]
    /// use qubit_datatype::{DataConversionError, DataType};
    ///
    /// let error = DataConversionError::missing(DataType::String, DataType::Bool);
    /// error.is_missing();
    /// ```
    #[must_use]
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
            | DataConversionErrorInner::InvalidValue { from, .. }
            | DataConversionErrorInner::LimitExceeded { from, .. }
            | DataConversionErrorInner::Quantity { from, .. } => Some(*from),
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
            | DataConversionErrorInner::LimitExceeded { to, .. }
            | DataConversionErrorInner::Quantity { to, .. }
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
            DataConversionErrorInner::InvalidValue { reason, .. } => Some(reason),
            _ => None,
        }
    }

    /// Returns the conversion resource associated with an accounting failure.
    ///
    /// # Returns
    ///
    /// `Some` for resource-limit and native-measurement failures, or `None`
    /// for conversion errors unrelated to resource accounting.
    #[must_use]
    #[inline(always)]
    pub const fn resource(&self) -> Option<ConversionResource> {
        match &self.inner {
            DataConversionErrorInner::LimitExceeded { source, .. } => Some(*source.resource()),
            DataConversionErrorInner::Quantity { resource, .. } => Some(*resource),
            _ => None,
        }
    }

    /// Returns the configured limit associated with a resource-limit failure.
    ///
    /// # Returns
    ///
    /// `Some` with the point maximum or cumulative limit for a
    /// resource-limit error, or `None` for every other error kind.
    #[must_use]
    #[inline(always)]
    pub const fn configured_limit(&self) -> Option<u64> {
        match &self.inner {
            DataConversionErrorInner::LimitExceeded { source, .. } => Some(source.configured_limit()),
            _ => None,
        }
    }

    /// Returns the safe lower bound observed by a failed point-limit check.
    ///
    /// # Returns
    ///
    /// `Some` for a point-limit failure, including inexact observations, or
    /// `None` for cumulative and non-budget failures.
    #[must_use]
    #[inline(always)]
    pub const fn observed_lower_bound(&self) -> Option<u64> {
        match &self.inner {
            DataConversionErrorInner::LimitExceeded { source, .. } => source.observed_lower_bound(),
            _ => None,
        }
    }

    /// Reports whether a point-limit observation is exact.
    ///
    /// # Returns
    ///
    /// `Some(true)` for an exact point observation, `Some(false)` for a
    /// lower-bound observation, or `None` when the error is not a point-limit
    /// failure.
    #[must_use]
    #[inline(always)]
    pub const fn observation_is_exact(&self) -> Option<bool> {
        match &self.inner {
            DataConversionErrorInner::LimitExceeded { source, .. } => match source.observation() {
                Some(Observation::Exact(_)) => Some(true),
                Some(Observation::AtLeast(_)) => Some(false),
                None => None,
            },
            _ => None,
        }
    }

    /// Returns cumulative usage before a failed request.
    ///
    /// # Returns
    ///
    /// `Some` for an insufficient cumulative budget, or `None` for point-limit
    /// and non-budget failures.
    #[must_use]
    #[inline(always)]
    pub fn used(&self) -> Option<u64> {
        match &self.inner {
            DataConversionErrorInner::LimitExceeded { source, .. } => source.used(),
            _ => None,
        }
    }

    /// Returns capacity remaining before a failed cumulative request.
    ///
    /// # Returns
    ///
    /// `Some` for an insufficient cumulative budget, or `None` for point-limit
    /// and non-budget failures.
    #[must_use]
    #[inline(always)]
    pub const fn remaining(&self) -> Option<u64> {
        match &self.inner {
            DataConversionErrorInner::LimitExceeded { source, .. } => source.remaining(),
            _ => None,
        }
    }

    /// Returns the quantity rejected by a failed cumulative request.
    ///
    /// # Returns
    ///
    /// `Some` for an insufficient cumulative budget, or `None` for point-limit
    /// and non-budget failures.
    #[must_use]
    #[inline(always)]
    pub const fn requested(&self) -> Option<u64> {
        match &self.inner {
            DataConversionErrorInner::LimitExceeded { source, .. } => source.requested(),
            _ => None,
        }
    }

    /// Returns the resource whose native measurement could not fit `u64`.
    #[must_use]
    #[inline(always)]
    pub const fn quantity_resource(&self) -> Option<ConversionResource> {
        match &self.inner {
            DataConversionErrorInner::Quantity { resource, .. } => Some(*resource),
            _ => None,
        }
    }
}
