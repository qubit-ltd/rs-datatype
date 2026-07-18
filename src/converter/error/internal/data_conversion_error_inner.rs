// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Conversion Error Inner
//!
//! Defines the private variant representation of data conversion errors.

use crate::datatype::DataType;

use super::super::conversion_limit::ConversionLimit;
use super::super::invalid_value_reason::InvalidValueReason;

/// Variant-specific details stored by a public conversion error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(in crate::converter::error) enum DataConversionErrorInner {
    /// The source has no concrete value.
    #[error("Missing value for conversion from {from} to {to}")]
    Missing {
        /// Declared source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
    },
    /// A first-value conversion was requested from an empty collection.
    #[error("Cannot convert the first value of an empty collection to {to}")]
    EmptyCollection {
        /// Requested target data type.
        to: DataType,
    },
    /// The source and target type pair is unsupported.
    #[error("Unsupported conversion from {from} to {to}")]
    Unsupported {
        /// Declared source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
    },
    /// The type pair is supported but the source value is invalid.
    #[error("Invalid conversion from {from} to {to}: {reason}")]
    InvalidValue {
        /// Declared source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
        /// Stable, value-free rejection reason.
        reason: InvalidValueReason,
    },
    /// The conversion would exceed a configured resource limit.
    #[error("Conversion limit exceeded from {from} to {to}: {limit}")]
    LimitExceeded {
        /// Declared source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
        /// Configured resource limit that was exceeded.
        limit: ConversionLimit,
    },
}
