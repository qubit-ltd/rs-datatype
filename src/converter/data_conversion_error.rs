// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Conversion Error
//!
//! Defines errors returned by reusable data conversions.

use crate::datatype::DataType;

use super::data_conversion_error_kind::InvalidValueReason;

/// Error type returned by reusable data conversions.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DataConversionError {
    /// The source has no concrete value.
    #[error("Missing value for conversion from {from} to {to}")]
    Missing {
        /// Source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
    },

    /// The source and target type pair is unsupported.
    #[error("Unsupported conversion from {from} to {to}")]
    Unsupported {
        /// Source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
    },

    /// The type pair is supported but the source value is invalid.
    #[error("Invalid conversion from {from} to {to}: {reason}")]
    InvalidValue {
        /// Source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
        /// Value-free reason for rejection.
        reason: InvalidValueReason,
    },
}
