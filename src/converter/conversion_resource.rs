// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Resource identities used by conversion limits and budgets.

/// Identifies a measurable resource consumed or checked by conversion.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConversionResource {
    /// UTF-8 bytes in duration text.
    DurationTextBytes,
    /// UTF-8 bytes in numeric text.
    NumericTextBytes,
    /// UTF-8 bytes in structured text.
    StructuredTextBytes,
    /// UTF-8 bytes in one scalar collection source before scanning.
    CollectionSourceBytes,
    /// Significant decimal digits materialized by a big-number conversion.
    BigIntegerDigits,
    /// Magnitude bits of a BigInteger value.
    BigIntegerMagnitudeBits,
    /// Significant coefficient digits of a BigDecimal value.
    BigDecimalCoefficientDigits,
    /// Absolute scale magnitude of a BigDecimal value.
    BigDecimalScaleMagnitude,
    /// Number of top-level conversion items.
    Items,
    /// Cumulative input bytes.
    InputBytes,
    /// Cumulative UTF-8 payload bytes produced by built-in String targets.
    OutputBytes,
    /// Maximum root-inclusive structured value depth.
    StructuredDepth,
    /// Cumulative structured value nodes.
    StructuredNodes,
    /// Cumulative structured key, string, and number payload bytes.
    StructuredPayloadBytes,
    /// Number of items in one structured sequence.
    SequenceItems,
    /// Number of retained items in one scalar collection conversion.
    CollectionItems,
    /// Number of entries in one structured map.
    MapEntries,
}
