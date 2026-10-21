// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Error returned when a Duration cannot be represented exactly.

/// Indicates that exact millisecond encoding would discard precision.
///
/// # Examples
///
/// ```
/// use qubit_datatype::DurationPrecisionLossError;
/// use qubit_datatype::serde::duration_millis_with_unit_exact;
/// use std::time::Duration;
///
/// let duration = Duration::new(1, 1);
/// assert_eq!(
///     duration_millis_with_unit_exact::format(&duration),
///     Err(DurationPrecisionLossError),
/// );
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("duration has submillisecond precision")]
pub struct DurationPrecisionLossError;
