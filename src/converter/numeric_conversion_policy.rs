// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! # Numeric Conversion Policy
//!
//! Defines whether numeric conversions may lose information.

use serde::{
    Deserialize,
    Serialize,
};

/// Controls whether numeric conversions must preserve the source value.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum NumericConversionPolicy {
    /// Reject conversions that require truncation, rounding, or precision
    /// loss.
    #[default]
    Exact,
    /// Permit the explicitly documented lossy conversion behavior.
    Lossy,
}
