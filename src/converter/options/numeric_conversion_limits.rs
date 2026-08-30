// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Resource limits for numeric conversions.
#[cfg(feature = "big-decimal")]
use qubit_budget::BigDecimalLimits;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
use qubit_budget::BigIntegerLimits;
use qubit_budget::ResourceLimit;
use qubit_budget::StringLimits;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use super::internal::NumericConversionLimitsWire;
use crate::converter::ConversionResource;

mod numeric_conversion_limits_builder;

pub use numeric_conversion_limits_builder::NumericConversionLimitsBuilder;

/// Bounds allocations and work introduced by numeric conversion.
///
/// The text limit applies to UTF-8 bytes after configured string
/// normalization. The digit limit applies to the significant decimal digits
/// of a `BigInt` result that conversion would materialize; leading zeros and a
/// zero result do not consume that budget.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericConversionLimits {
    /// Maximum normalized numeric source text length in bytes.
    text: StringLimits<ConversionResource>,
    /// Maximum decimal digits materialized for a BigInt result.
    #[cfg(feature = "big-integer")]
    big_integer: BigIntegerLimits<ConversionResource>,
    #[cfg(not(feature = "big-integer"))]
    max_big_integer_digits: ResourceLimit<ConversionResource, u64>,
    /// Maximum decimal digits materialized for a BigDecimal coefficient.
    #[cfg(feature = "big-decimal")]
    big_decimal: BigDecimalLimits<ConversionResource>,
    #[cfg(not(feature = "big-decimal"))]
    max_big_decimal_coefficient_digits: ResourceLimit<ConversionResource, u64>,
    /// Maximum absolute BigDecimal scale.
    #[cfg(not(feature = "big-decimal"))]
    max_big_decimal_scale_magnitude: ResourceLimit<ConversionResource, u64>,
}

impl NumericConversionLimits {
    /// Creates a builder initialized with the default numeric limits.
    #[inline]
    #[must_use]
    pub fn builder() -> NumericConversionLimitsBuilder {
        NumericConversionLimitsBuilder::new()
    }

    /// Converts these limits into a builder for explicit customization.
    #[inline]
    #[must_use]
    pub fn into_builder(self) -> NumericConversionLimitsBuilder {
        NumericConversionLimitsBuilder { limits: self }
    }
    /// Default maximum normalized numeric text length in bytes.
    pub const DEFAULT_MAX_TEXT_BYTES: u64 = 16_384;

    /// Default maximum decimal digits materialized for a BigInt result.
    pub const DEFAULT_MAX_BIG_INTEGER_DIGITS: u64 = 16_384;
    /// Default maximum BigDecimal coefficient digits.
    pub const DEFAULT_MAX_BIG_DECIMAL_COEFFICIENT_DIGITS: u64 = 16_384;
    /// Default maximum absolute BigDecimal scale.
    pub const DEFAULT_MAX_BIG_DECIMAL_SCALE_MAGNITUDE: u64 = 150_000;

    /// Returns the maximum normalized numeric text length in bytes.
    ///
    /// # Returns
    ///
    /// The configured normalized numeric text byte limit.
    #[inline(always)]
    #[must_use]
    pub fn max_text_bytes(&self) -> u64 {
        self.text.utf8_bytes_limit().map_or(0, |limit| limit.maximum())
    }

    /// Returns the composed text-byte limits.
    #[inline]
    #[must_use]
    pub const fn text(&self) -> &StringLimits<ConversionResource> {
        &self.text
    }

    /// Returns the numeric text resource limit.
    #[inline]
    #[must_use]
    pub fn max_text_bytes_limit(&self) -> &ResourceLimit<ConversionResource, u64> {
        self.text.utf8_bytes_limit().expect("numeric text limit is configured")
    }

    /// Returns a copy with a different numeric text byte limit.
    ///
    /// # Parameters
    ///
    /// * `maximum` - New normalized numeric text byte limit.
    ///
    /// # Returns
    ///
    /// Updated limits.
    #[inline(always)]
    pub(crate) fn with_max_text_bytes(mut self, maximum: u64) -> Self {
        self.text = StringLimits::builder()
            .utf8_bytes_limit(ResourceLimit::new(ConversionResource::NumericTextBytes, maximum))
            .build();
        self
    }

    /// Returns the maximum decimal digits materialized for a BigInt result.
    ///
    /// # Returns
    ///
    /// The configured BigInteger decimal digit limit.
    #[inline(always)]
    #[must_use]
    pub fn max_big_integer_digits(&self) -> u64 {
        #[cfg(feature = "big-integer")]
        {
            self.big_integer
                .significant_decimal_digits_limit()
                .map_or(0, |limit| limit.maximum())
        }
        #[cfg(not(feature = "big-integer"))]
        {
            self.max_big_integer_digits.maximum()
        }
    }

    /// Returns composed BigInteger limits when the feature is enabled.
    #[cfg(feature = "big-integer")]
    #[inline]
    #[must_use]
    pub const fn big_integer(&self) -> &BigIntegerLimits<ConversionResource> {
        &self.big_integer
    }

    /// Returns the BigInteger digit resource limit.
    #[inline]
    pub fn max_big_integer_digits_limit(&self) -> &ResourceLimit<ConversionResource, u64> {
        #[cfg(feature = "big-integer")]
        {
            self.big_integer
                .significant_decimal_digits_limit()
                .expect("BigInteger digit limit is configured")
        }
        #[cfg(not(feature = "big-integer"))]
        {
            &self.max_big_integer_digits
        }
    }

    /// Returns a copy with a different BigInt decimal digit limit.
    ///
    /// # Parameters
    ///
    /// * `maximum` - New materialized decimal digit limit.
    ///
    /// # Returns
    ///
    /// Updated limits.
    #[inline(always)]
    pub(crate) fn with_max_big_integer_digits(mut self, maximum: u64) -> Self {
        #[cfg(feature = "big-integer")]
        {
            self.big_integer = BigIntegerLimits::builder()
                .significant_decimal_digits_limit(ResourceLimit::new(ConversionResource::BigIntegerDigits, maximum))
                .build();
        }
        #[cfg(not(feature = "big-integer"))]
        {
            self.max_big_integer_digits = ResourceLimit::new(ConversionResource::BigIntegerDigits, maximum);
        }
        self
    }

    /// Returns the configured BigDecimal coefficient digit maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_big_decimal_coefficient_digits(&self) -> u64 {
        #[cfg(feature = "big-decimal")]
        {
            self.big_decimal
                .coefficient_limits()
                .significant_decimal_digits_limit()
                .map_or(0, |limit| limit.maximum())
        }
        #[cfg(not(feature = "big-decimal"))]
        {
            self.max_big_decimal_coefficient_digits.maximum()
        }
    }

    /// Returns a copy with a different BigDecimal coefficient digit maximum.
    #[inline]
    pub(crate) fn with_max_big_decimal_coefficient_digits(mut self, maximum: u64) -> Self {
        #[cfg(feature = "big-decimal")]
        {
            self.big_decimal = BigDecimalLimits::builder()
                .coefficient_limits(
                    BigIntegerLimits::builder()
                        .significant_decimal_digits_limit(ResourceLimit::new(
                            ConversionResource::BigDecimalCoefficientDigits,
                            maximum,
                        ))
                        .build(),
                )
                .scale_magnitude_limit(
                    *self
                        .big_decimal
                        .scale_magnitude_limit()
                        .expect("scale limit is configured"),
                )
                .build();
        }
        #[cfg(not(feature = "big-decimal"))]
        {
            self.max_big_decimal_coefficient_digits =
                ResourceLimit::new(ConversionResource::BigDecimalCoefficientDigits, maximum);
        }
        self
    }

    /// Returns the configured absolute BigDecimal scale maximum.
    #[inline(always)]
    #[must_use]
    pub fn max_big_decimal_scale_magnitude(&self) -> u64 {
        #[cfg(feature = "big-decimal")]
        {
            self.big_decimal
                .scale_magnitude_limit()
                .map_or(0, |limit| limit.maximum())
        }
        #[cfg(not(feature = "big-decimal"))]
        {
            self.max_big_decimal_scale_magnitude.maximum()
        }
    }

    /// Returns composed BigDecimal limits when the feature is enabled.
    #[cfg(feature = "big-decimal")]
    #[inline]
    #[must_use]
    pub const fn big_decimal(&self) -> &BigDecimalLimits<ConversionResource> {
        &self.big_decimal
    }

    /// Returns a copy with a different absolute BigDecimal scale maximum.
    #[inline]
    pub(crate) fn with_max_big_decimal_scale_magnitude(mut self, maximum: u64) -> Self {
        #[cfg(feature = "big-decimal")]
        {
            self.big_decimal = BigDecimalLimits::builder()
                .coefficient_limits(*self.big_decimal.coefficient_limits())
                .scale_magnitude_limit(ResourceLimit::new(
                    ConversionResource::BigDecimalScaleMagnitude,
                    maximum,
                ))
                .build();
        }
        #[cfg(not(feature = "big-decimal"))]
        {
            self.max_big_decimal_scale_magnitude =
                ResourceLimit::new(ConversionResource::BigDecimalScaleMagnitude, maximum);
        }
        self
    }
}

impl Default for NumericConversionLimits {
    /// Creates the default numeric conversion limits.
    ///
    /// # Returns
    ///
    /// Limits using the documented default byte and digit budgets.
    #[inline(always)]
    fn default() -> Self {
        Self {
            text: StringLimits::builder()
                .utf8_bytes_limit(ResourceLimit::new(
                    ConversionResource::NumericTextBytes,
                    Self::DEFAULT_MAX_TEXT_BYTES,
                ))
                .build(),
            #[cfg(feature = "big-integer")]
            big_integer: BigIntegerLimits::builder()
                .significant_decimal_digits_limit(ResourceLimit::new(
                    ConversionResource::BigIntegerDigits,
                    Self::DEFAULT_MAX_BIG_INTEGER_DIGITS,
                ))
                .build(),
            #[cfg(not(feature = "big-integer"))]
            max_big_integer_digits: ResourceLimit::new(
                ConversionResource::BigIntegerDigits,
                Self::DEFAULT_MAX_BIG_INTEGER_DIGITS,
            ),
            #[cfg(feature = "big-decimal")]
            big_decimal: BigDecimalLimits::builder()
                .coefficient_limits(
                    BigIntegerLimits::builder()
                        .significant_decimal_digits_limit(ResourceLimit::new(
                            ConversionResource::BigDecimalCoefficientDigits,
                            Self::DEFAULT_MAX_BIG_DECIMAL_COEFFICIENT_DIGITS,
                        ))
                        .build(),
                )
                .scale_magnitude_limit(ResourceLimit::new(
                    ConversionResource::BigDecimalScaleMagnitude,
                    Self::DEFAULT_MAX_BIG_DECIMAL_SCALE_MAGNITUDE,
                ))
                .build(),
            #[cfg(not(feature = "big-decimal"))]
            max_big_decimal_coefficient_digits: ResourceLimit::new(
                ConversionResource::BigDecimalCoefficientDigits,
                Self::DEFAULT_MAX_BIG_DECIMAL_COEFFICIENT_DIGITS,
            ),
            #[cfg(not(feature = "big-decimal"))]
            max_big_decimal_scale_magnitude: ResourceLimit::new(
                ConversionResource::BigDecimalScaleMagnitude,
                Self::DEFAULT_MAX_BIG_DECIMAL_SCALE_MAGNITUDE,
            ),
        }
    }
}

impl Serialize for NumericConversionLimits {
    /// Serializes the configured limits through the private wire shape.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NumericConversionLimitsWire {
            max_text_bytes: self.max_text_bytes(),
            max_big_integer_digits: self.max_big_integer_digits(),
            max_big_decimal_coefficient_digits: self.max_big_decimal_coefficient_digits(),
            max_big_decimal_scale_magnitude: self.max_big_decimal_scale_magnitude(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NumericConversionLimits {
    /// Deserializes limits from the private wire shape.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = NumericConversionLimitsWire::deserialize(deserializer)?;
        Ok(Self::default()
            .with_max_text_bytes(wire.max_text_bytes)
            .with_max_big_integer_digits(wire.max_big_integer_digits)
            .with_max_big_decimal_coefficient_digits(wire.max_big_decimal_coefficient_digits)
            .with_max_big_decimal_scale_magnitude(wire.max_big_decimal_scale_magnitude))
    }
}
