// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Blank String Policy
//!
//! Defines how blank string sources are interpreted during conversion.

use serde::{
    Deserialize,
    Serialize,
};

/// Controls how blank string sources are interpreted during conversion.
///
/// This enum intentionally defines a closed policy set. Exhaustive matching is
/// part of its API contract; adding a variant is a deliberate breaking change.
#[must_use]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum BlankStringPolicy {
    /// Keep blank strings as real string values.
    #[default]
    Preserve,
    /// Treat blank strings as missing values.
    TreatAsMissing,
    /// Reject blank strings as invalid input.
    Reject,
}
