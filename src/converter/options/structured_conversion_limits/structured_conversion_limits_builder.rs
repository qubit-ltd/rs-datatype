// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for structured conversion limits.

use super::StructuredConversionLimits;

/// Builder for [`StructuredConversionLimits`].
///
/// `max_depth` counts the root value as depth one. `max_sequence_items` and
/// `max_map_entries` each limit the direct contents of every individual
/// container. Zero sequence and map maxima accept only empty containers; a
/// zero depth maximum rejects every root value. These limits apply to each
/// converted value, while cumulative structured-node accounting is configured
/// separately through [`ConversionOperationLimits`].
///
/// [`ConversionOperationLimits`]: crate::converter::ConversionOperationLimits
///
/// # Examples
///
/// ```
/// use qubit_datatype::StructuredConversionLimits;
///
/// let limits = StructuredConversionLimits::builder()
///     .max_depth(3)
///     .max_sequence_items(2)
///     .max_map_entries(1)
///     .build();
/// assert_eq!(limits.max_depth(), 3);
/// assert_eq!(limits.max_sequence_items(), 2);
/// assert_eq!(limits.max_map_entries(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructuredConversionLimitsBuilder {
    /// Structured limits being configured.
    pub(super) limits: StructuredConversionLimits,
}

impl StructuredConversionLimitsBuilder {
    /// Creates a builder initialized with the documented defaults.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            limits: StructuredConversionLimits::default(),
        }
    }

    /// Configures the normalized text byte maximum.
    #[inline]
    #[must_use]
    pub fn max_text_bytes(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_text_bytes(maximum),
        }
    }

    /// Configures the maximum depth, counting the root value as one. A zero
    /// maximum rejects every root value.
    #[inline]
    #[must_use]
    pub fn max_depth(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_depth(maximum),
        }
    }

    /// Configures the maximum number of direct items in each sequence. Zero
    /// permits only empty sequences.
    #[inline]
    #[must_use]
    pub fn max_sequence_items(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_sequence_items(maximum),
        }
    }

    /// Configures the maximum number of direct entries in each map. Zero
    /// permits only empty maps.
    #[inline]
    #[must_use]
    pub fn max_map_entries(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_map_entries(maximum),
        }
    }

    /// Builds the configured structured limits.
    #[inline]
    pub fn build(self) -> StructuredConversionLimits {
        self.limits
    }
}
