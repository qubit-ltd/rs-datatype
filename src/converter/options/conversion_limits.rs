// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Value and operation limits applied by data conversion.
// qubit-style: allow multiple-public-types

use std::sync::LazyLock;

use serde::Deserialize;
use serde::Serialize;

use super::CollectionConversionLimits;
use super::CollectionConversionLimitsBuilder;
use super::ConversionOperationLimits;
use super::ConversionOperationLimitsBuilder;
use super::DurationConversionLimits;
use super::DurationConversionLimitsBuilder;
use super::NumericConversionLimits;
use super::NumericConversionLimitsBuilder;
use super::StructuredConversionLimits;
use super::StructuredConversionLimitsBuilder;

/// Aggregates immutable value limits and cumulative operation limits.
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

    /// Returns a shared reference to the default limits.
    ///
    /// # Returns
    ///
    /// A process-wide lazily initialized default value.
    #[must_use = "the default conversion limits should be inspected"]
    #[inline(always)]
    pub fn default_ref() -> &'static Self {
        static DEFAULT: LazyLock<ConversionLimits> =
            LazyLock::new(ConversionLimits::default);
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

/// Builder for [`ConversionLimits`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionLimitsBuilder {
    limits: ConversionLimits,
}

impl From<NumericConversionLimitsBuilder> for NumericConversionLimits {
    fn from(builder: NumericConversionLimitsBuilder) -> Self {
        builder.build()
    }
}

impl From<CollectionConversionLimitsBuilder> for CollectionConversionLimits {
    fn from(builder: CollectionConversionLimitsBuilder) -> Self {
        builder.build()
    }
}

impl From<DurationConversionLimitsBuilder> for DurationConversionLimits {
    fn from(builder: DurationConversionLimitsBuilder) -> Self {
        builder.build()
    }
}

impl From<StructuredConversionLimitsBuilder> for StructuredConversionLimits {
    fn from(builder: StructuredConversionLimitsBuilder) -> Self {
        builder.build()
    }
}

impl From<ConversionOperationLimitsBuilder> for ConversionOperationLimits {
    fn from(builder: ConversionOperationLimitsBuilder) -> Self {
        builder.build()
    }
}

impl ConversionLimitsBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            limits: ConversionLimits::default(),
        }
    }

    /// Configures numeric value limits.
    #[inline(always)]
    #[must_use]
    pub fn numeric_limits<T>(mut self, limits: T) -> Self
    where
        T: Into<NumericConversionLimits>,
    {
        self.limits.numeric = limits.into();
        self
    }

    /// Configures scalar collection value limits.
    #[inline(always)]
    #[must_use]
    pub fn collection_limits(
        mut self,
        limits: impl Into<CollectionConversionLimits>,
    ) -> Self {
        self.limits.collection = limits.into();
        self
    }

    /// Configures duration value limits.
    #[inline(always)]
    #[must_use]
    pub fn duration_limits(
        mut self,
        limits: impl Into<DurationConversionLimits>,
    ) -> Self {
        self.limits.duration = limits.into();
        self
    }

    /// Configures structured value limits.
    #[inline(always)]
    #[must_use]
    pub fn structured_limits(
        mut self,
        limits: impl Into<StructuredConversionLimits>,
    ) -> Self {
        self.limits.structured = limits.into();
        self
    }

    /// Configures cumulative operation limits.
    #[inline(always)]
    #[must_use]
    pub fn operation_limits(
        mut self,
        limits: impl Into<ConversionOperationLimits>,
    ) -> Self {
        self.limits.operation = limits.into();
        self
    }

    /// Builds the configured conversion limits.
    #[inline]
    #[must_use]
    pub fn build(self) -> ConversionLimits {
        self.limits
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
