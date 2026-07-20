// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Policies and option groups used by data conversions.

mod blank_string_policy;
mod boolean_conversion_options;
mod boolean_numeric_policy;
mod collection_conversion_options;
mod data_conversion_options;
mod duration_conversion_options;
mod duration_rounding_policy;
mod empty_item_policy;
mod float_rounding_policy;
mod fractional_to_integer_policy;
mod internal;
mod numeric_conversion_limits;
mod numeric_conversion_options;
mod string_conversion_options;
mod structured_conversion_limits;

pub use crate::duration::{DurationUnit, SuffixlessDurationPolicy};
pub use blank_string_policy::BlankStringPolicy;
pub use boolean_conversion_options::BooleanConversionOptions;
pub use boolean_numeric_policy::BooleanNumericPolicy;
pub use collection_conversion_options::CollectionConversionOptions;
pub use data_conversion_options::DataConversionOptions;
pub use duration_conversion_options::DurationConversionOptions;
pub use duration_rounding_policy::DurationRoundingPolicy;
pub use empty_item_policy::EmptyItemPolicy;
pub use float_rounding_policy::FloatRoundingPolicy;
pub use fractional_to_integer_policy::FractionalToIntegerPolicy;
pub use numeric_conversion_limits::NumericConversionLimits;
pub use numeric_conversion_options::NumericConversionOptions;
pub use string_conversion_options::StringConversionOptions;
pub use structured_conversion_limits::StructuredConversionLimits;
