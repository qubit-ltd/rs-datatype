// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for duration conversion limits.

use super::DurationConversionLimits;

/// Builder for [`DurationConversionLimits`].
///
/// # Examples
///
/// ```
/// use qubit_datatype::DurationConversionLimits;
///
/// let limits = DurationConversionLimits::builder()
///     .max_text_bytes(4_096)
///     .build();
/// assert_eq!(limits.max_text_bytes(), 4_096);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationConversionLimitsBuilder {
    /// Duration limits being configured.
    pub(super) limits: DurationConversionLimits,
}

impl DurationConversionLimitsBuilder {
    /// Creates a builder initialized with the documented default.
    ///
    /// # Returns
    ///
    /// A builder configured with the default maximum duration text size.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: DurationConversionLimits {
                max_text_bytes: DurationConversionLimits::DEFAULT_MAX_TEXT_BYTES,
            },
        }
    }

    /// Configures the maximum duration source text size.
    ///
    /// # Parameters
    ///
    /// - `maximum`: Maximum accepted source-text length in UTF-8 bytes.
    ///
    /// # Returns
    ///
    /// This builder with its maximum source-text length updated.
    #[inline]
    #[must_use]
    pub const fn max_text_bytes(mut self, maximum: usize) -> Self {
        self.limits.max_text_bytes = maximum;
        self
    }

    /// Builds the configured duration limits.
    ///
    /// # Returns
    ///
    /// The duration limits assembled from the configured maximum source-text
    /// length.
    #[inline]
    pub const fn build(self) -> DurationConversionLimits {
        self.limits
    }
}
