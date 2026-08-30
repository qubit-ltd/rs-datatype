// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Value and operation limits applied by data conversion.
use std::sync::LazyLock;

use serde::Deserialize;
use serde::Serialize;

use super::CollectionConversionLimits;
use super::ConversionOperationLimits;
use super::DurationConversionLimits;
use super::NumericConversionLimits;
use super::StructuredConversionLimits;

mod conversion_limits_builder;

pub use conversion_limits_builder::ConversionLimitsBuilder;

/// Aggregates immutable value limits and cumulative operation limits.
///
/// # Examples
///
/// ```
/// use qubit_datatype::ConversionLimits;
///
/// let limits = ConversionLimits::builder().build();
/// assert!(limits.operation().max_items() > 0);
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ConversionLimits {
    /// Limits applied to one numeric value conversion.
    numeric: NumericConversionLimits,

    /// Limits applied to one scalar collection conversion.
    collection: CollectionConversionLimits,

    /// Limits applied to one duration value conversion.
    duration: DurationConversionLimits,

    /// Limits applied to one structured value conversion.
    structured: StructuredConversionLimits,

    /// Cumulative limits shared by one logical conversion operation.
    operation: ConversionOperationLimits,
}

impl ConversionLimits {
    /// Creates a builder initialized with the default conversion limits.
    #[inline]
    #[must_use]
    pub fn builder() -> ConversionLimitsBuilder {
        ConversionLimitsBuilder::new()
    }

    /// Converts these limits into a builder for explicit customization.
    #[inline]
    #[must_use]
    pub fn into_builder(self) -> ConversionLimitsBuilder {
        ConversionLimitsBuilder { limits: self }
    }

    /// Returns a shared reference to the default limits.
    ///
    /// # Returns
    ///
    /// A process-wide lazily initialized default value.
    #[must_use = "the default conversion limits should be inspected"]
    #[inline(always)]
    pub fn default_ref() -> &'static Self {
        static DEFAULT: LazyLock<ConversionLimits> = LazyLock::new(ConversionLimits::default);
        &DEFAULT
    }

    /// Returns the numeric value limits.
    #[must_use = "numeric conversion limits should be inspected"]
    #[inline(always)]
    pub const fn numeric(&self) -> &NumericConversionLimits {
        &self.numeric
    }

    /// Returns the scalar collection value limits.
    #[must_use = "collection conversion limits should be inspected"]
    #[inline(always)]
    pub const fn collection(&self) -> &CollectionConversionLimits {
        &self.collection
    }

    /// Returns the duration value limits.
    #[must_use = "duration conversion limits should be inspected"]
    #[inline(always)]
    pub const fn duration(&self) -> &DurationConversionLimits {
        &self.duration
    }

    /// Returns the structured value limits.
    #[must_use = "structured conversion limits should be inspected"]
    #[inline(always)]
    pub const fn structured(&self) -> &StructuredConversionLimits {
        &self.structured
    }

    /// Returns the cumulative operation limits.
    #[must_use = "conversion operation limits should be inspected"]
    #[inline(always)]
    pub const fn operation(&self) -> &ConversionOperationLimits {
        &self.operation
    }
}

impl Default for ConversionLimits {
    /// Creates the documented finite conversion limits.
    #[inline(always)]
    fn default() -> Self {
        Self {
            numeric: NumericConversionLimits::default(),
            collection: CollectionConversionLimits::default(),
            duration: DurationConversionLimits::default(),
            structured: StructuredConversionLimits::default(),
            operation: ConversionOperationLimits::default(),
        }
    }
}
