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
///
/// [`Self::Exact`] provides deterministic mathematical ordering across numeric
/// representations. [`Self::Approximate`] performs a pair-dependent projection
/// and is not transitive, so it must not be used to implement [`Ord`], sort or
/// group values, or construct keys for ordered maps and sets.
#[must_use]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum NumericComparisonPolicy {
    /// Preserves exact mathematical values for deterministic ordering.
    #[default]
    Exact,
    /// Attempts finite `f64` projection when a primitive float participates.
    ///
    /// Primitive infinities are ordered separately. If either operand cannot
    /// be projected to a finite `f64`, comparison falls back to the exact path.
    /// Projected comparison is not transitive across mixed representations and
    /// is therefore unsuitable for [`Ord`], sorting, grouping, or ordered keys.
    Approximate,
}
