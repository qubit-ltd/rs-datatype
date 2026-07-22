// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Resource limits for numeric conversions.

use serde::{
    Deserialize,
    Serialize,
};

/// Bounds allocations and work introduced by numeric conversion.
///
/// The text limit applies to UTF-8 bytes after configured string
/// normalization. The digit limit applies to the significant decimal digits
/// of a `BigInt` result that conversion would materialize; leading zeros and a
/// zero result do not consume that budget.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct NumericConversionLimits {
    /// Maximum normalized numeric source text length in bytes.
    max_text_bytes: usize,
    /// Maximum decimal digits materialized for a BigInt result.
    max_big_integer_digits: usize,
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
        self.max_text_bytes
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
        self.max_text_bytes = maximum;
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
        self.max_big_integer_digits
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
        self.max_big_integer_digits = maximum;
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
            max_text_bytes: Self::DEFAULT_MAX_TEXT_BYTES,
            max_big_integer_digits: Self::DEFAULT_MAX_BIG_INTEGER_DIGITS,
        }
    }
}
