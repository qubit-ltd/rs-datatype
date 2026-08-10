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
    /// A cumulative input byte budget was exhausted.
    #[error("conversion input exceeds the {maximum}-byte budget")]
    InputBytes {
        /// Configured cumulative input byte maximum.
        maximum: usize,
    },
    /// A cumulative output byte budget was exhausted.
    #[error("conversion output exceeds the {maximum}-byte budget")]
    OutputBytes {
        /// Configured cumulative output byte maximum.
        maximum: usize,
    },
    /// A structured depth limit or node budget was exceeded.
    #[error("structured depth exceeds the {maximum}-level limit")]
    StructuredDepth {
        /// Configured root-inclusive depth maximum.
        maximum: usize,
    },
    /// A cumulative structured node budget was exhausted.
    #[error("structured value exceeds the {maximum}-node budget")]
    StructuredNodes {
        /// Configured cumulative node maximum.
        maximum: usize,
    },
    /// A sequence item point limit was exceeded.
    #[error("sequence exceeds the {maximum}-item limit")]
    SequenceItems {
        /// Configured sequence item maximum.
        maximum: usize,
    },
    /// A map entry point limit was exceeded.
    #[error("map exceeds the {maximum}-entry limit")]
    MapEntries {
        /// Configured map entry maximum.
        maximum: usize,
    },
}

impl ConversionLimit {
    /// Derives the legacy maximum-only view from complete conversion facts.
    pub(crate) fn from_source(
        source: &super::conversion_limit_exceeded::ConversionLimitExceeded,
    ) -> Self {
        let maximum = source
            .maximum()
            .unwrap_or(source.limit().unwrap_or_default());
        match source.resource() {
            crate::converter::ConversionResource::DurationTextBytes => {
                Self::DurationTextBytes { maximum }
            }
            crate::converter::ConversionResource::NumericTextBytes => {
                Self::NumericTextBytes { maximum }
            }
            crate::converter::ConversionResource::StructuredTextBytes => {
                Self::StructuredTextBytes { maximum }
            }
            crate::converter::ConversionResource::BigIntegerDigits => {
                Self::BigIntegerDigits { maximum }
            }
            crate::converter::ConversionResource::InputBytes => Self::InputBytes { maximum },
            crate::converter::ConversionResource::OutputBytes => Self::OutputBytes { maximum },
            crate::converter::ConversionResource::StructuredDepth => {
                Self::StructuredDepth { maximum }
            }
            crate::converter::ConversionResource::StructuredNodes => {
                Self::StructuredNodes { maximum }
            }
            crate::converter::ConversionResource::SequenceItems => Self::SequenceItems { maximum },
            crate::converter::ConversionResource::MapEntries => Self::MapEntries { maximum },
            crate::converter::ConversionResource::Items => Self::CollectionItems { maximum },
        }
    }
}
