/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Data List Conversion Result
//!
//! Defines the result alias used by reusable batch data conversions.
//!

use super::data_list_conversion_error::DataListConversionError;

/// Result type used by reusable batch data conversions.
pub type DataListConversionResult<T> = Result<T, DataListConversionError>;
