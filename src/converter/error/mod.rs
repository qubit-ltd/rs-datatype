// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Errors and stable rejection reasons produced by data conversions.

mod boolean_literal_conflict_error;
mod data_conversion_error;
mod data_conversion_error_kind;
mod data_format;
mod data_list_conversion_error;
mod duration_overflow_error;
mod internal;
mod invalid_value_reason;
mod scalar_item_error;
mod string_normalization_error;

pub use boolean_literal_conflict_error::BooleanLiteralConflictError;
pub use data_conversion_error::DataConversionError;
pub use data_conversion_error_kind::DataConversionErrorKind;
pub use data_format::DataFormat;
pub use data_list_conversion_error::DataListConversionError;
pub use duration_overflow_error::DurationOverflowError;
pub use invalid_value_reason::InvalidValueReason;
pub use scalar_item_error::ScalarItemError;
pub use string_normalization_error::StringNormalizationError;
