// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Policy for converting fractional values to integers.

use serde::{Deserialize, Serialize};

/// Controls whether a fractional numeric value may be converted to an integer.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FractionalToIntegerPolicy {
    /// Reject values with a non-zero fractional component.
    #[default]
    Reject,
    /// Discard the fractional component by truncating toward zero.
    Truncate,
}
