/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Data List Conversion Result
//!
//! Defines the result alias used by reusable batch data conversions.
//!
//! # Author
//!
//! Haixing Hu

use super::data_list_conversion_error::DataListConversionError;

/// Result type used by reusable batch data conversions.
pub type DataListConversionResult<T> = Result<T, DataListConversionError>;
