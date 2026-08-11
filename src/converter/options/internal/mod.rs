// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private Serde wire representations for conversion options.

mod conversion_operation_limits_wire;
mod numeric_conversion_limits_wire;
mod structured_conversion_limits_wire;
mod unchecked_boolean_conversion_policy;

pub(super) use conversion_operation_limits_wire::ConversionOperationLimitsWire;
pub(super) use numeric_conversion_limits_wire::NumericConversionLimitsWire;
pub(super) use structured_conversion_limits_wire::StructuredConversionLimitsWire;
pub(super) use unchecked_boolean_conversion_policy::UncheckedBooleanConversionPolicy;
