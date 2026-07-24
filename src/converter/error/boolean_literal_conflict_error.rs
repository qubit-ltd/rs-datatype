// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Boolean literal conflict error.

/// Error returned when true and false literal sets overlap.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("boolean true and false literals overlap")]
pub struct BooleanLiteralConflictError;
