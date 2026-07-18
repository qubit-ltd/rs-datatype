// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Resource limits enforced by data conversion.

/// Identifies the conversion resource limit that was exceeded.
#[must_use]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum ConversionLimit {
    /// The normalized numeric source text exceeded its byte limit.
    #[error("numeric text exceeds the {maximum}-byte limit")]
    NumericTextBytes {
        /// Configured maximum normalized text length in bytes.
        maximum: usize,
    },
    /// A conversion would materialize too many BigInteger decimal digits.
    #[error("BigInteger result exceeds the {maximum}-decimal-digit limit")]
    BigIntegerDigits {
        /// Configured maximum materialized decimal digits.
        maximum: usize,
    },
}
