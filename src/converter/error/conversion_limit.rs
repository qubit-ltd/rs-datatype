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
    /// Duration source text exceeded its configured byte limit.
    #[error("duration text exceeds the {maximum}-byte limit")]
    DurationTextBytes {
        /// Configured maximum source text length in bytes.
        maximum: usize,
    },
    /// The normalized numeric source text exceeded its byte limit.
    #[error("numeric text exceeds the {maximum}-byte limit")]
    NumericTextBytes {
        /// Configured maximum normalized text length in bytes.
        maximum: usize,
    },
    /// The normalized structured source text exceeded its byte limit.
    #[error("structured text exceeds the {maximum}-byte limit")]
    StructuredTextBytes {
        /// Configured maximum normalized text length in bytes.
        maximum: usize,
    },
    /// A conversion would materialize too many BigInteger decimal digits.
    #[error("BigInteger result exceeds the {maximum}-decimal-digit limit")]
    BigIntegerDigits {
        /// Configured maximum materialized decimal digits.
        maximum: usize,
    },
    /// A scalar collection would retain too many items.
    #[error("collection result exceeds the {maximum}-item limit")]
    CollectionItems {
        /// Configured maximum number of retained items.
        maximum: usize,
    },
}
