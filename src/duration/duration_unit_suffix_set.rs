// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Supported suffix sets for Duration text parsing.

use serde::{
    Deserialize,
    Serialize,
};

/// Selects the unit suffixes accepted by Duration text parsing.
#[must_use]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum DurationUnitSuffixSet {
    /// Accept only ASCII suffixes, including `us` for microseconds.
    Ascii,
    /// Additionally accept `µs` and `μs` as microsecond aliases.
    #[default]
    Extended,
}
