// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Builder for numeric conversion limits.

use super::NumericConversionLimits;

/// Builder for [`NumericConversionLimits`].
///
/// The text limit applies after string normalization. The other limits bound
/// the corresponding big-number result representations. These settings do not
/// bound every temporary allocation or the independent `NumberRef` comparison
/// API.
///
/// # Examples
///
/// ```
/// use qubit_datatype::NumericConversionLimits;
///
/// let limits = NumericConversionLimits::builder()
///     .max_text_bytes(256)
///     .max_big_integer_digits(64)
///     .max_big_decimal_coefficient_digits(64)
///     .max_big_decimal_scale_magnitude(32)
///     .build();
/// assert_eq!(limits.max_text_bytes(), 256);
/// assert_eq!(limits.max_big_integer_digits(), 64);
/// assert_eq!(limits.max_big_decimal_coefficient_digits(), 64);
/// assert_eq!(limits.max_big_decimal_scale_magnitude(), 32);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericConversionLimitsBuilder {
    /// Numeric limits being configured.
    pub(super) limits: NumericConversionLimits,
}

impl NumericConversionLimitsBuilder {
    /// Creates a builder initialized with the documented defaults.
    ///
    /// # Returns
    ///
    /// A builder containing the default numeric conversion limits.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            limits: NumericConversionLimits::default(),
        }
    }

    /// Sets the maximum UTF-8 byte length of normalized numeric text.
    ///
    /// # Parameters
    ///
    /// * `maximum` - Maximum number of bytes accepted after string
    ///   normalization.
    ///
    /// # Returns
    ///
    /// This builder with the normalized text byte limit set to `maximum`.
    #[inline]
    #[must_use]
    pub fn max_text_bytes(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_text_bytes(maximum),
        }
    }

    /// Sets the maximum number of significant decimal digits materialized for a
    /// `BigInt`.
    ///
    /// # Parameters
    ///
    /// * `maximum` - Maximum number of significant result digits; leading zeros
    ///   do not count.
    ///
    /// # Returns
    ///
    /// This builder with the BigInt digit limit set to `maximum`.
    #[inline]
    #[must_use]
    pub fn max_big_integer_digits(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_big_integer_digits(maximum),
        }
    }

    /// Sets the maximum number of significant decimal digits in a `BigDecimal`
    /// coefficient.
    ///
    /// # Parameters
    ///
    /// * `maximum` - Maximum number of significant coefficient digits.
    ///
    /// # Returns
    ///
    /// This builder with the BigDecimal coefficient digit limit set to
    /// `maximum`.
    #[inline]
    #[must_use]
    pub fn max_big_decimal_coefficient_digits(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_big_decimal_coefficient_digits(maximum),
        }
    }

    /// Sets the maximum absolute scale magnitude accepted for a `BigDecimal`.
    ///
    /// # Parameters
    ///
    /// * `maximum` - Maximum absolute value of the decimal scale.
    ///
    /// # Returns
    ///
    /// This builder with the BigDecimal scale magnitude limit set to
    /// `maximum`.
    #[inline]
    #[must_use]
    pub fn max_big_decimal_scale_magnitude(self, maximum: u64) -> Self {
        Self {
            limits: self.limits.with_max_big_decimal_scale_magnitude(maximum),
        }
    }

    /// Builds the configured numeric limits.
    ///
    /// # Returns
    ///
    /// The configured numeric conversion limits.
    #[inline]
    pub fn build(self) -> NumericConversionLimits {
        self.limits
    }
}
