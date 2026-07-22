// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Expected categories for DataConverter matrix tests.

/// Expected result of an i32 conversion matrix row.
pub(in crate::converter) enum MatrixOutcome {
    /// Conversion succeeds with this value.
    Supported(i32),
    /// Source and target types are not a supported pair.
    Unsupported,
    /// Source text does not match integer syntax.
    InvalidSyntax,
    /// The source contains no value.
    Missing,
}
