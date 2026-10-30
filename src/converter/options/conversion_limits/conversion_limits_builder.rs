// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for aggregate conversion limits.

use super::ConversionLimits;
use crate::converter::options::CollectionConversionLimits;
use crate::converter::options::ConversionOperationLimits;
use crate::converter::options::DurationConversionLimits;
use crate::converter::options::NumericConversionLimits;
use crate::converter::options::StructuredConversionLimits;

/// Builder for [`ConversionLimits`].
///
/// # Examples
///
/// ```
/// use qubit_datatype::ConversionLimitsBuilder;
///
/// let limits = ConversionLimitsBuilder::new().build();
/// assert!(limits.operation().max_items() > 0);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionLimitsBuilder {
    /// Aggregate limits being configured.
    pub(super) limits: ConversionLimits,
}

impl ConversionLimitsBuilder {
    /// Creates a builder initialized with the documented defaults.
    ///
    /// # Returns
    ///
    /// A builder containing the default aggregate conversion limits.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            limits: ConversionLimits::default(),
        }
    }

    /// Configures numeric value limits.
    ///
    /// # Parameters
    ///
    /// * `limits` - Numeric limits applied to each converted numeric value.
    ///
    /// # Returns
    ///
    /// This builder with the numeric value limits configured as `limits`.
    #[inline]
    #[must_use]
    pub fn numeric_limits(mut self, limits: NumericConversionLimits) -> Self {
        self.limits.numeric = limits;
        self
    }

    /// Configures scalar collection value limits.
    ///
    /// # Parameters
    ///
    /// * `limits` - Limits applied to the contents of each scalar collection.
    ///
    /// # Returns
    ///
    /// This builder with the scalar collection limits configured as `limits`.
    #[inline]
    #[must_use]
    pub fn collection_limits(mut self, limits: CollectionConversionLimits) -> Self {
        self.limits.collection = limits;
        self
    }

    /// Configures duration value limits.
    ///
    /// # Parameters
    ///
    /// * `limits` - Limits applied to each converted duration value.
    ///
    /// # Returns
    ///
    /// This builder with the duration value limits configured as `limits`.
    #[inline]
    #[must_use]
    pub fn duration_limits(mut self, limits: DurationConversionLimits) -> Self {
        self.limits.duration = limits;
        self
    }

    /// Configures structured value limits.
    ///
    /// # Parameters
    ///
    /// * `limits` - Limits applied to each converted structured value.
    ///
    /// # Returns
    ///
    /// This builder with the structured value limits configured as `limits`.
    #[inline]
    #[must_use]
    pub fn structured_limits(mut self, limits: StructuredConversionLimits) -> Self {
        self.limits.structured = limits;
        self
    }

    /// Configures cumulative operation limits.
    ///
    /// # Parameters
    ///
    /// * `limits` - Limits shared across the logical conversion operation.
    ///
    /// # Returns
    ///
    /// This builder with the cumulative operation limits configured as
    /// `limits`.
    #[inline]
    #[must_use]
    pub fn operation_limits(mut self, limits: ConversionOperationLimits) -> Self {
        self.limits.operation = limits;
        self
    }

    /// Builds the configured conversion limits.
    ///
    /// # Returns
    ///
    /// The aggregate conversion limits configured by this builder.
    #[inline]
    pub fn build(self) -> ConversionLimits {
        self.limits
    }
}
