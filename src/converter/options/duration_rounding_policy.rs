// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Policy for rounding Duration output units.

use serde::{
    Deserialize,
    Serialize,
};

/// Controls Duration conversion when the selected output unit has a remainder.
///
/// This enum intentionally defines a closed policy set. Exhaustive matching is
/// part of its API contract; adding a variant is a deliberate breaking change.
#[must_use]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum DurationRoundingPolicy {
    /// Reject a Duration that is not an exact multiple of the output unit.
    #[default]
    Reject,
    /// Round to the nearest output unit, with exact halves rounded upward.
    HalfUp,
}
