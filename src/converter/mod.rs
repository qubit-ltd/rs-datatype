// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Reusable Data Conversion
//!
//! Provides data conversion utilities based on [`crate::datatype::DataType`].

mod data_conversion_target;
mod data_converter;
mod data_converters;
mod error;
mod options;
mod scalar_item;
mod scalar_items;
mod scalar_string_data_converters;

pub use data_conversion_target::DataConversionTarget;
pub use data_converter::DataConverter;
pub use data_converters::DataConverters;
pub use error::{
    BooleanLiteralConflictError,
    DataConversionError,
    DataFormat,
    DataListConversionError,
    DurationOverflowError,
    InvalidValueReason,
    ScalarItemError,
    StringNormalizationError,
};
pub use options::{
    BlankStringPolicy,
    BooleanConversionOptions,
    BooleanNumericPolicy,
    CollectionConversionOptions,
    DataConversionOptions,
    DurationConversionOptions,
    DurationUnit,
    EmptyItemPolicy,
    NumericConversionPolicy,
    StringConversionOptions,
    SuffixlessDurationPolicy,
};
pub use scalar_item::ScalarItem;
pub use scalar_items::ScalarItems;
pub use scalar_string_data_converters::ScalarStringDataConverters;
