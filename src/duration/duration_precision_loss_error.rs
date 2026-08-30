// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Error returned when a Duration cannot be represented exactly.

/// Indicates that exact millisecond encoding would discard precision.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("duration has submillisecond precision")]
pub struct DurationPrecisionLossError;
