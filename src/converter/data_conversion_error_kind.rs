// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! # Data Conversion Error Kind
//!
//! Defines stable, value-free reasons for invalid conversions.

use super::data_format::DataFormat;

/// Reason an otherwise supported conversion rejected its source value.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InvalidValueReason {
    /// A blank string is forbidden by the active policy.
    #[error("blank string rejected")]
    BlankRejected,
    /// The source text does not match the target syntax.
    #[error("invalid syntax; expected {expected}")]
    InvalidSyntax {
        /// Stable description of the accepted syntax.
        expected: &'static str,
    },
    /// The source value is outside the target range.
    #[error("value out of range")]
    OutOfRange,
    /// An exact conversion would lose information.
    #[error("precision loss")]
    PrecisionLoss,
    /// A non-finite floating-point value is not accepted.
    #[error("non-finite value")]
    NonFinite,
    /// The value is invalid under the active boolean policy.
    #[error("invalid boolean")]
    InvalidBoolean,
    /// A negative value cannot represent a duration.
    #[error("negative duration")]
    NegativeDuration,
    /// The duration suffix is not supported.
    #[error("unsupported duration unit")]
    UnsupportedDurationUnit,
    /// Serialization to a structured format failed.
    #[error("{} serialization failed", format.as_str())]
    Serialization {
        /// Format whose serialization failed.
        format: DataFormat,
    },
    /// Deserialization from a structured format failed.
    #[error("{} deserialization failed", format.as_str())]
    Deserialization {
        /// Format whose deserialization failed.
        format: DataFormat,
    },
}
