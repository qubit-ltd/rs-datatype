// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for scalar-string collection conversion limits.

use super::CollectionConversionLimits;

/// Builder for [`CollectionConversionLimits`].
///
/// # Examples
///
/// ```
/// use qubit_datatype::CollectionConversionLimits;
///
/// let limits = CollectionConversionLimits::builder()
///     .max_source_bytes(4_096)
///     .max_items(128)
///     .build();
/// assert_eq!(limits.max_source_bytes(), 4_096);
/// assert_eq!(limits.max_items(), 128);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollectionConversionLimitsBuilder {
    /// Collection limits being configured.
    pub(super) limits: CollectionConversionLimits,
}

impl CollectionConversionLimitsBuilder {
    /// Creates a builder initialized with the documented defaults.
    ///
    /// # Returns
    ///
    /// A builder with the default source-byte and retained-item limits.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: CollectionConversionLimits {
                max_source_bytes: CollectionConversionLimits::DEFAULT_MAX_SOURCE_BYTES,
                max_items: CollectionConversionLimits::DEFAULT_MAX_ITEMS,
            },
        }
    }

    /// Sets the maximum UTF-8 byte length accepted for one collection source.
    ///
    /// # Parameters
    ///
    /// - `maximum`: Maximum source length in bytes.
    ///
    /// # Returns
    ///
    /// This builder with the source-byte limit updated.
    #[inline]
    #[must_use]
    pub const fn max_source_bytes(mut self, maximum: u64) -> Self {
        self.limits.max_source_bytes = maximum;
        self
    }

    /// Sets the maximum number of items retained by one collection conversion.
    ///
    /// # Parameters
    ///
    /// - `maximum`: Maximum retained item count.
    ///
    /// # Returns
    ///
    /// This builder with the retained-item limit updated.
    #[inline]
    #[must_use]
    pub const fn max_items(mut self, maximum: u64) -> Self {
        self.limits.max_items = maximum;
        self
    }

    /// Builds the configured collection limits.
    ///
    /// # Returns
    ///
    /// The collection limits assembled from the builder's configured values.
    #[inline]
    #[must_use]
    pub const fn build(self) -> CollectionConversionLimits {
        self.limits
    }
}
