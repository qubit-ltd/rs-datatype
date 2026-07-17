// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Policy-driven comparison across numeric representations.

mod compare_numeric;
mod internal;
mod numeric_comparison_policy;
mod numeric_value_ref;

pub use compare_numeric::compare_numeric;
pub use numeric_comparison_policy::NumericComparisonPolicy;
pub use numeric_value_ref::NumericValueRef;
