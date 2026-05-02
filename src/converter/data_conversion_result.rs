/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Data Conversion Result
//!
//! Defines the result alias used by reusable data conversions.
//!

use super::data_conversion_error::DataConversionError;

/// Result type used by reusable data conversions.
pub type DataConversionResult<T> = Result<T, DataConversionError>;
