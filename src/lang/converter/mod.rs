/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Reusable Data Conversion
//!
//! Provides language-level data conversion utilities based on [`crate::lang::DataType`].
//!
//! # Author
//!
//! Haixing Hu

mod blank_string_policy;
mod boolean_conversion_options;
mod collection_conversion_options;
mod data_conversion_error;
mod data_conversion_options;
mod data_conversion_result;
mod data_convert_to;
mod data_converter;
mod data_converters;
mod data_list_conversion_error;
mod data_list_conversion_result;
mod empty_item_policy;
mod scalar_string_data_converters;
mod string_conversion_options;

pub use blank_string_policy::BlankStringPolicy;
pub use boolean_conversion_options::BooleanConversionOptions;
pub use collection_conversion_options::CollectionConversionOptions;
pub use data_conversion_error::DataConversionError;
pub use data_conversion_options::DataConversionOptions;
pub use data_conversion_result::DataConversionResult;
pub use data_convert_to::DataConvertTo;
pub use data_converter::DataConverter;
pub use data_converters::DataConverters;
pub use data_list_conversion_error::DataListConversionError;
pub use data_list_conversion_result::DataListConversionResult;
pub use empty_item_policy::EmptyItemPolicy;
pub use scalar_string_data_converters::ScalarStringDataConverters;
pub use string_conversion_options::StringConversionOptions;
