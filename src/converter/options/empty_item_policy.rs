// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Empty Item Policy
//!
//! Defines how empty collection items are interpreted after splitting.

use serde::{
    Deserialize,
    Serialize,
};

/// Controls how empty collection items are interpreted after splitting.
#[must_use]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum EmptyItemPolicy {
    /// Keep empty items and pass them to the element converter.
    #[default]
    Keep,
    /// Drop empty items before element conversion.
    Skip,
    /// Reject empty items as invalid input.
    Reject,
}
