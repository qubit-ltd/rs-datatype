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

mod admitted_conversion;
mod admitted_scalar_item;
mod conversion_capabilities;
mod conversion_context;
mod conversion_resource;
mod conversion_session;
mod conversion_string_writer;
mod data_conversion_target;
mod data_converter;
mod data_converters;
mod error;
mod internal;
mod options;
mod scalar_item;
mod scalar_items;
mod scalar_string_data_converters;

pub use admitted_conversion::AdmittedConversion;
pub use admitted_scalar_item::AdmittedScalarItem;
pub use conversion_capabilities::ConversionCapabilities;
pub use conversion_context::ConversionContext;
pub use conversion_resource::ConversionResource;
pub use conversion_session::ConversionSession;
pub use conversion_string_writer::ConversionStringWriter;
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
pub(crate) use internal::BuiltinDataConversionTarget;
pub use options::BlankStringPolicy;
pub use options::BooleanConversionPolicy;
pub use options::BooleanConversionPolicyBuilder;
pub use options::BooleanNumericPolicy;
pub use options::CollectionConversionLimits;
pub use options::CollectionConversionLimitsBuilder;
pub use options::CollectionConversionPolicy;
pub use options::CollectionConversionPolicyBuilder;
pub use options::ConversionLimits;
pub use options::ConversionLimitsBuilder;
pub use options::ConversionOperationLimits;
pub use options::ConversionOperationLimitsBuilder;
pub use options::ConversionPolicy;
pub use options::ConversionPolicyBuilder;
pub use options::DurationConversionLimits;
pub use options::DurationConversionLimitsBuilder;
pub use options::DurationConversionPolicy;
pub use options::DurationConversionPolicyBuilder;
pub use options::DurationRoundingPolicy;
pub use options::DurationUnit;
pub use options::EmptyItemPolicy;
pub use options::FloatRoundingPolicy;
pub use options::FractionalToIntegerPolicy;
pub use options::NumericConversionLimits;
pub use options::NumericConversionLimitsBuilder;
pub use options::NumericConversionPolicy;
pub use options::NumericConversionPolicyBuilder;
pub use options::StringConversionPolicy;
pub use options::StringConversionPolicyBuilder;
pub use options::StructuredConversionLimits;
pub use options::StructuredConversionLimitsBuilder;
pub use options::SuffixlessDurationPolicy;
pub use scalar_item::ScalarItem;
pub use scalar_items::ScalarItems;
pub use scalar_string_data_converters::ScalarStringDataConverters;
