/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Data Conversion Error
//!
//! Defines errors returned by reusable data conversions.
//!
//! # Author
//!
//! Haixing Hu

use std::error::Error;
use std::fmt;

use crate::lang::DataType;

/// Error type returned by reusable data conversions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataConversionError {
    /// No concrete source value is available.
    NoValue,

    /// The source value type cannot be converted to the requested target type.
    ConversionFailed {
        /// Source data type.
        from: DataType,
        /// Target data type.
        to: DataType,
    },

    /// Conversion failed because the source value content is invalid or out of
    /// range for the target type.
    ConversionError(String),

    /// JSON serialization failed while converting a value to JSON text or a
    /// JSON value.
    JsonSerializationError(String),

    /// JSON deserialization failed while parsing JSON text.
    JsonDeserializationError(String),
}

impl fmt::Display for DataConversionError {
    /// Formats the conversion error for user-facing diagnostics.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataConversionError::NoValue => write!(f, "No value"),
            DataConversionError::ConversionFailed { from, to } => {
                write!(f, "Type conversion failed: from {from} to {to}")
            }
            DataConversionError::ConversionError(message) => {
                write!(f, "Conversion error: {message}")
            }
            DataConversionError::JsonSerializationError(message) => {
                write!(f, "JSON serialization error: {message}")
            }
            DataConversionError::JsonDeserializationError(message) => {
                write!(f, "JSON deserialization error: {message}")
            }
        }
    }
}

impl Error for DataConversionError {}
