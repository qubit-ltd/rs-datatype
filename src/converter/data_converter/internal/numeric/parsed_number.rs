// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact parsed representations used by textual numeric conversion.

use bigdecimal::BigDecimal;
use num_bigint::BigInt;

/// Represents a parsed finite number or an explicit non-finite marker.
#[must_use]
pub(super) enum ParsedNumber {
    /// Arbitrary-precision integral value.
    Integer(
        /// Parsed arbitrary-precision integral value.
        BigInt,
    ),
    /// Arbitrary-precision decimal value.
    Decimal(
        /// Parsed arbitrary-precision decimal value.
        BigDecimal,
    ),
    /// Not-a-number marker.
    NaN,
    /// Positive infinity marker.
    PositiveInfinity,
    /// Negative infinity marker.
    NegativeInfinity,
}
