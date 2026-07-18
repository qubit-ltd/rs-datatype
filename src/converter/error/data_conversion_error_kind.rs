// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Conversion Error Kind
//!
//! Defines stable classifications for data conversion errors.

/// Stable classification of a [`super::DataConversionError`].
#[must_use]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataConversionErrorKind {
    /// The source has no concrete value.
    Missing,
    /// A first-value conversion was requested from an empty collection.
    EmptyCollection,
    /// The source and target type pair is unsupported.
    Unsupported,
    /// The type pair is supported but the source value is invalid.
    InvalidValue,
    /// A configured conversion resource limit was exceeded.
    LimitExceeded,
}
