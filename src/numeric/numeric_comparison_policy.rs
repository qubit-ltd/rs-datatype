// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric comparison policy definitions.

use serde::{
    Deserialize,
    Serialize,
};

/// Selects exact or floating-projection numeric comparison.
#[must_use]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum NumericComparisonPolicy {
    /// Preserves the exact mathematical value of every representation.
    #[default]
    Exact,
    /// Uses finite `f64` projection when at least one operand is a float.
    Approximate,
}
