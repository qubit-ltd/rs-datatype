// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Test holder for fixed millisecond Duration text.

use std::time::Duration;

use serde::{
    Deserialize,
    Serialize,
};

/// Holds a Duration encoded as fixed millisecond text.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DurationMillisWithUnitHolder {
    /// Duration encoded through the fixed millisecond text adapter.
    #[serde(with = "qubit_datatype::serde::duration_millis_with_unit")]
    pub(crate) duration: Duration,
}
