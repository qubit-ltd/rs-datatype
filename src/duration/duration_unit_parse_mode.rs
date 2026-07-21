// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Strictness modes for Duration unit symbols.

use serde::{
    Deserialize,
    Serialize,
};

/// Selects the accepted Duration unit symbol set.
#[must_use]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum DurationUnitParseMode {
    /// Accept stable strict symbols, including `us`, `µs`, and `μs`.
    #[default]
    Strict,
    /// Additionally accept documented non-canonical aliases such as `m`.
    Lenient,
}
