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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionLimitsBuilder {
    /// Aggregate limits being configured.
    pub(super) limits: ConversionLimits,
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
    pub fn numeric_limits(mut self, limits: NumericConversionLimits) -> Self {
        self.limits.numeric = limits;
        self
    }

    /// Configures scalar collection value limits.
    #[inline(always)]
    #[must_use]
    pub fn collection_limits(mut self, limits: CollectionConversionLimits) -> Self {
        self.limits.collection = limits;
        self
    }

    /// Configures duration value limits.
    #[inline(always)]
    #[must_use]
    pub fn duration_limits(mut self, limits: DurationConversionLimits) -> Self {
        self.limits.duration = limits;
        self
    }

    /// Configures structured value limits.
    #[inline(always)]
    #[must_use]
    pub fn structured_limits(mut self, limits: StructuredConversionLimits) -> Self {
        self.limits.structured = limits;
        self
    }

    /// Configures cumulative operation limits.
    #[inline(always)]
    #[must_use]
    pub fn operation_limits(mut self, limits: ConversionOperationLimits) -> Self {
        self.limits.operation = limits;
        self
    }

    /// Builds the configured conversion limits.
    #[inline]
    #[must_use]
    pub fn build(self) -> ConversionLimits {
        self.limits
    }
}
