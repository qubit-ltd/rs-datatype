// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Policy for Duration text without a unit suffix.

use serde::{Deserialize, Serialize};

use super::DurationUnit;

/// Controls parsing of Duration text that omits an explicit unit suffix.
///
/// Explicitly suffixed text such as `"10ms"` does not use this policy.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuffixlessDurationPolicy {
    /// Reject Duration text without an explicit unit suffix.
    Reject,
    /// Interpret suffixless integers using the contained unit.
    Assume(DurationUnit),
}

impl Default for SuffixlessDurationPolicy {
    /// Preserves the default convention that suffixless text is milliseconds.
    fn default() -> Self {
        Self::Assume(DurationUnit::Milliseconds)
    }
}
