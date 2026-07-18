// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private numeric comparison algorithms.

#[cfg(feature = "big-decimal")]
mod exact_decimal;
#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
mod exact_rational;
mod fixed_numeric;

#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
pub(super) use exact_rational::compare_exact_rational;
pub(super) use fixed_numeric::compare_fixed;
