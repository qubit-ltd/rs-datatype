// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Policies and option groups used by data conversions.

mod blank_string_policy;
mod boolean_conversion_policy;
mod boolean_numeric_policy;
mod collection_conversion_limits;
mod collection_conversion_policy;
mod conversion_limits;
mod conversion_operation_limits;
mod conversion_policy;
mod duration_conversion_limits;
mod duration_conversion_policy;
mod duration_rounding_policy;
mod empty_item_policy;
mod float_rounding_policy;
mod fractional_to_integer_policy;
mod internal;
mod numeric_conversion_limits;
mod numeric_conversion_policy;
mod string_conversion_policy;
mod structured_conversion_limits;

pub use blank_string_policy::BlankStringPolicy;
pub use boolean_conversion_policy::BooleanConversionPolicy;
pub use boolean_numeric_policy::BooleanNumericPolicy;
pub use collection_conversion_limits::CollectionConversionLimits;
pub use collection_conversion_policy::CollectionConversionPolicy;
pub use conversion_limits::ConversionLimits;
pub use conversion_operation_limits::ConversionOperationLimits;
pub use conversion_policy::ConversionPolicy;
pub use duration_conversion_limits::DurationConversionLimits;
pub use duration_conversion_policy::DurationConversionPolicy;
pub use duration_rounding_policy::DurationRoundingPolicy;
pub use empty_item_policy::EmptyItemPolicy;
pub use float_rounding_policy::FloatRoundingPolicy;
pub use fractional_to_integer_policy::FractionalToIntegerPolicy;
pub use numeric_conversion_limits::NumericConversionLimits;
pub use numeric_conversion_policy::NumericConversionPolicy;
pub use string_conversion_policy::StringConversionPolicy;
pub use structured_conversion_limits::StructuredConversionLimits;

pub use crate::duration::DurationUnit;
pub use crate::duration::SuffixlessDurationPolicy;
