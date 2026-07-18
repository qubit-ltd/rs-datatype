// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private numeric comparison algorithms.

#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
mod exact_rational;
mod fixed_numeric;
mod number_repr;

#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
pub(in crate::numeric) use exact_rational::{
    f32_rational,
    f64_rational,
};
#[cfg(feature = "big-decimal")]
pub(in crate::numeric) use exact_rational::decimal_rational;
pub(in crate::numeric) use fixed_numeric::{
    compare_magnitude,
    finite_parts,
};
pub(super) use number_repr::NumberRepr;
