// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Test holder for exact millisecond Duration text.

use std::time::Duration;

use qubit_datatype::serde::duration_millis_with_unit_exact;
use serde::Deserialize;
use serde::Serialize;

/// Holds a Duration encoded as exact millisecond text.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DurationMillisWithUnitExactHolder {
    /// Duration encoded through the exact millisecond text adapter.
    #[serde(with = "duration_millis_with_unit_exact")]
    pub(crate) duration: Duration,
}
