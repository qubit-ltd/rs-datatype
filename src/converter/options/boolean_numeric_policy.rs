// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! # Boolean Numeric Policy
//!
//! Defines how integer values may be converted to booleans.

use serde::{
    Deserialize,
    Serialize,
};

/// Controls conversion from integer values to booleans.
///
/// This policy applies both to integer source variants and to integer-shaped
/// strings that did not match a configured boolean literal.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum BooleanNumericPolicy {
    /// Accept only zero as false and one as true.
    #[default]
    ZeroOrOne,
    /// Interpret zero as false and every non-zero integer as true.
    NonZero,
    /// Reject every numeric source.
    Reject,
}
