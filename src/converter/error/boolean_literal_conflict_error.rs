// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Boolean literal conflict error.

/// Reports that boolean conversion policy assigns one literal to both values.
///
/// The conflict is checked using the policy's case-sensitivity setting, so a
/// case-insensitive policy also rejects literals that differ only in ASCII
/// letter case.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{BooleanConversionPolicy, BooleanLiteralConflictError};
///
/// let error = BooleanConversionPolicy::builder()
///     .true_literal("enabled")
///     .false_literal("enabled")
///     .build()
///     .unwrap_err();
/// assert_eq!(error, BooleanLiteralConflictError);
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("boolean true and false literals overlap")]
pub struct BooleanLiteralConflictError;
