/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Data Conversion Result
//!
//! Defines the result alias used by reusable data conversions.
//!
//! # Author
//!
//! Haixing Hu

use super::data_conversion_error::DataConversionError;

/// Result type used by reusable data conversions.
pub type DataConversionResult<T> = Result<T, DataConversionError>;
