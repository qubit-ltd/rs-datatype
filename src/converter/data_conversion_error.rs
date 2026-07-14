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

use std::error::Error;
use std::fmt;

use crate::datatype::DataType;

use super::data_conversion_error_kind::DataConversionErrorKind;

/// Error type returned by reusable data conversions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataConversionError {
    /// The source has no concrete value.
    Missing {
        /// Source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
    },

    /// The source and target type pair is unsupported.
    Unsupported {
        /// Source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
    },

    /// The type pair is supported but the source value is invalid.
    Invalid {
        /// Source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
        /// Value-free reason for rejection.
        kind: DataConversionErrorKind,
    },
}

impl fmt::Display for DataConversionError {
    /// Formats the conversion error for user-facing diagnostics.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataConversionError::Missing { from, to } => {
                write!(f, "Missing value for conversion from {from} to {to}")
            }
            DataConversionError::Unsupported { from, to } => {
                write!(f, "Unsupported conversion from {from} to {to}")
            }
            DataConversionError::Invalid { from, to, kind } => {
                write!(f, "Invalid conversion from {from} to {to}: {kind}")
            }
        }
    }
}

impl Error for DataConversionError {}
