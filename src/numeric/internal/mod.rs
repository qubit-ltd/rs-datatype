// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private numeric comparison algorithms.

#[cfg(feature = "big-integer")]
mod big_integer_float;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
mod exact_rational;
mod fixed_numeric;
mod number_repr;

#[cfg(feature = "big-integer")]
pub(in crate::numeric) use big_integer_float::compare_big_integer_float;
#[cfg(feature = "big-decimal")]
pub(in crate::numeric) use exact_rational::decimal_rational;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
pub(in crate::numeric) use exact_rational::f32_rational;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
pub(in crate::numeric) use exact_rational::f64_rational;
pub(in crate::numeric) use fixed_numeric::compare_magnitude;
pub(in crate::numeric) use fixed_numeric::finite_parts;
pub(super) use number_repr::NumberRepr;
