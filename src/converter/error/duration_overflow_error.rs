// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Duration Overflow Error
//!
//! Defines the low-level error returned when a unit count cannot fit in a
//! [`std::time::Duration`].

/// Reports that a unit count exceeds the representable Duration range.
///
/// The error deliberately contains no source value, so it is safe to expose
/// when conversion input may contain sensitive configuration data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("duration exceeds the range supported by std::time::Duration")]
pub struct DurationOverflowError;
