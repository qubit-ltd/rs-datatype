// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! # Data Conversion Error Kind
//!
//! Defines stable, value-free reasons for invalid conversions.

use std::fmt;

use super::data_format::DataFormat;

/// Reason an otherwise supported conversion rejected its source value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataConversionErrorKind {
    /// A blank string is forbidden by the active policy.
    BlankRejected,
    /// The source text does not match the target syntax.
    InvalidSyntax {
        /// Stable description of the accepted syntax.
        expected: &'static str,
    },
    /// The source value is outside the target range.
    OutOfRange,
    /// An exact conversion would lose information.
    PrecisionLoss,
    /// A non-finite floating-point value is not accepted.
    NonFinite,
    /// The value is invalid under the active boolean policy.
    InvalidBoolean,
    /// A negative value cannot represent a duration.
    NegativeDuration,
    /// The duration suffix is not supported.
    UnsupportedDurationUnit,
    /// Serialization to a structured format failed.
    Serialization {
        /// Format whose serialization failed.
        format: DataFormat,
    },
    /// Deserialization from a structured format failed.
    Deserialization {
        /// Format whose deserialization failed.
        format: DataFormat,
    },
}

impl fmt::Display for DataConversionErrorKind {
    /// Formats a stable description without including the source value.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankRejected => formatter.write_str("blank string rejected"),
            Self::InvalidSyntax { expected } => {
                write!(formatter, "invalid syntax; expected {expected}")
            }
            Self::OutOfRange => formatter.write_str("value out of range"),
            Self::PrecisionLoss => formatter.write_str("precision loss"),
            Self::NonFinite => formatter.write_str("non-finite value"),
            Self::InvalidBoolean => formatter.write_str("invalid boolean"),
            Self::NegativeDuration => formatter.write_str("negative duration"),
            Self::UnsupportedDurationUnit => {
                formatter.write_str("unsupported duration unit")
            }
            Self::Serialization { format } => {
                write!(formatter, "{} serialization failed", format.as_str())
            }
            Self::Deserialization { format } => {
                write!(formatter, "{} deserialization failed", format.as_str())
            }
        }
    }
}
