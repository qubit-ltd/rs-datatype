// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Policy for conversions that may round to a floating-point value.

use serde::{
    Deserialize,
    Serialize,
};

/// Controls whether floating-point rounding is permitted.
#[must_use]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum FloatRoundingPolicy {
    /// Reject a conversion unless the target float represents the source
    /// exactly.
    #[default]
    Exact,
    /// Permit IEEE 754 round-to-nearest, ties-to-even behavior.
    NearestEven,
}
