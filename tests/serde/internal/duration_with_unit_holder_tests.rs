// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Test holder for exact unit-suffixed Duration text.

use std::time::Duration;

use serde::{
    Deserialize,
    Serialize,
};

/// Holds a Duration encoded as exact unit-suffixed text.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DurationWithUnitHolder {
    /// Duration encoded through the exact unit-suffixed adapter.
    #[serde(with = "qubit_datatype::serde::duration_with_unit")]
    pub(crate) duration: Duration,
}
