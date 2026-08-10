// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Resource limits for numeric conversions.

use qubit_budget::ResourceLimit;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use crate::converter::ConversionResource;

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
    max_text_bytes: ResourceLimit<ConversionResource, usize>,
    /// Maximum decimal digits materialized for a BigInt result.
    max_big_integer_digits: ResourceLimit<ConversionResource, usize>,
}

impl NumericConversionLimits {
    /// Default maximum normalized numeric text length in bytes.
    pub const DEFAULT_MAX_TEXT_BYTES: usize = 16_384;

    /// Default maximum decimal digits materialized for a BigInt result.
    pub const DEFAULT_MAX_BIG_INTEGER_DIGITS: usize = 16_384;

    /// Returns the maximum normalized numeric text length in bytes.
    ///
    /// # Returns
    ///
    /// The configured normalized numeric text byte limit.
    #[inline(always)]
    #[must_use]
    pub const fn max_text_bytes(&self) -> usize {
        self.max_text_bytes.maximum()
    }

    /// Returns the numeric text resource limit.
    #[inline(always)]
    pub const fn max_text_bytes_limit(&self) -> &ResourceLimit<ConversionResource, usize> {
        &self.max_text_bytes
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
    pub const fn with_max_text_bytes(mut self, maximum: usize) -> Self {
        self.max_text_bytes = ResourceLimit::new(ConversionResource::NumericTextBytes, maximum);
        self
    }

    /// Returns the maximum decimal digits materialized for a BigInt result.
    ///
    /// # Returns
    ///
    /// The configured BigInteger decimal digit limit.
    #[inline(always)]
    #[must_use]
    pub const fn max_big_integer_digits(&self) -> usize {
        self.max_big_integer_digits.maximum()
    }

    /// Returns the BigInteger digit resource limit.
    #[inline(always)]
    pub const fn max_big_integer_digits_limit(&self) -> &ResourceLimit<ConversionResource, usize> {
        &self.max_big_integer_digits
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
    pub const fn with_max_big_integer_digits(mut self, maximum: usize) -> Self {
        self.max_big_integer_digits =
            ResourceLimit::new(ConversionResource::BigIntegerDigits, maximum);
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
            max_text_bytes: ResourceLimit::new(
                ConversionResource::NumericTextBytes,
                Self::DEFAULT_MAX_TEXT_BYTES,
            ),
            max_big_integer_digits: ResourceLimit::new(
                ConversionResource::BigIntegerDigits,
                Self::DEFAULT_MAX_BIG_INTEGER_DIGITS,
            ),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct NumericConversionLimitsWire {
    max_text_bytes: usize,
    max_big_integer_digits: usize,
}

impl Default for NumericConversionLimitsWire {
    fn default() -> Self {
        let limits = NumericConversionLimits::default();
        Self {
            max_text_bytes: limits.max_text_bytes(),
            max_big_integer_digits: limits.max_big_integer_digits(),
        }
    }
}

impl Serialize for NumericConversionLimits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NumericConversionLimitsWire {
            max_text_bytes: self.max_text_bytes(),
            max_big_integer_digits: self.max_big_integer_digits(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NumericConversionLimits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = NumericConversionLimitsWire::deserialize(deserializer)?;
        Ok(Self::default()
            .with_max_text_bytes(wire.max_text_bytes)
            .with_max_big_integer_digits(wire.max_big_integer_digits))
    }
}
