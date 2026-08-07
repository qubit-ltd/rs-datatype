// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Test holder for numeric millisecond Duration serialization.

use std::time::Duration;

use qubit_datatype::serde::duration_millis;
use serde::Deserialize;
use serde::Serialize;

/// Holds a Duration encoded as a numeric millisecond value.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DurationMillisHolder {
    /// Duration encoded through the numeric millisecond adapter.
    #[serde(with = "duration_millis")]
    pub(crate) duration: Duration,
}
