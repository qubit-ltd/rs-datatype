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

mod conversion_budget;
mod conversion_resource;
mod conversion_session;
mod data_conversion_target;
mod data_converter;
mod data_converters;
mod error;
mod options;
mod scalar_item;
mod scalar_items;
mod scalar_string_data_converters;

pub use conversion_resource::ConversionResource;
pub use conversion_session::ConversionSession;
pub use data_conversion_target::DataConversionTarget;
pub use data_converter::DataConverter;
pub use data_converters::DataConverters;
pub use error::BooleanLiteralConflictError;
pub use error::DataConversionError;
pub use error::DataConversionErrorKind;
pub use error::DataFormat;
pub use error::DataListConversionError;
pub use error::DurationOverflowError;
pub use error::InvalidValueReason;
pub use error::ScalarItemError;
pub use error::StringNormalizationError;
pub use options::BlankStringPolicy;
pub use options::BooleanConversionPolicy;
pub use options::BooleanNumericPolicy;
pub use options::CollectionConversionLimits;
pub use options::CollectionConversionPolicy;
pub use options::ConversionLimits;
pub use options::ConversionOperationLimits;
pub use options::ConversionPolicy;
pub use options::DurationConversionLimits;
pub use options::DurationConversionPolicy;
pub use options::DurationRoundingPolicy;
pub use options::DurationUnit;
pub use options::EmptyItemPolicy;
pub use options::FloatRoundingPolicy;
pub use options::FractionalToIntegerPolicy;
pub use options::NumericConversionLimits;
pub use options::NumericConversionPolicy;
pub use options::StringConversionPolicy;
pub use options::StructuredConversionLimits;
pub use options::SuffixlessDurationPolicy;
pub use scalar_item::ScalarItem;
pub use scalar_items::ScalarItems;
pub use scalar_string_data_converters::ScalarStringDataConverters;
