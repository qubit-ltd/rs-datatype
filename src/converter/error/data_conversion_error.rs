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

use super::invalid_value_reason::InvalidValueReason;

/// Describes why a single source value could not be converted.
///
/// Every variant records both the declared source and requested target
/// [`DataType`]. Invalid-value errors additionally carry a stable,
/// value-independent [`InvalidValueReason`]. Source values are deliberately not
/// retained or formatted, which makes these errors safe to surface for secrets
/// such as environment variables.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{DataConversionError, DataConverter, InvalidValueReason};
///
/// assert!(matches!(
///     DataConverter::from("not-a-number").to::<u32>(),
///     Err(DataConversionError::InvalidValue {
///         reason: InvalidValueReason::InvalidSyntax { .. },
///         ..
///     }),
/// ));
/// ```
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
