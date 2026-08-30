// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Semantic categories shared by runtime data type metadata.

/// Classifies a [`crate::DataType`] by its broad value semantics.
///
/// Categories are independent of Cargo features. They describe the stable
/// runtime vocabulary rather than whether the current build can materialize or
/// convert a concrete Rust type.
///
/// # Examples
///
/// ```
/// use qubit_datatype::DataType;
/// use qubit_datatype::DataTypeCategory;
///
/// assert_eq!(
///     DataType::Int32.info().category(),
///     DataTypeCategory::SignedInteger,
/// );
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataTypeCategory {
    /// Boolean truth values.
    Boolean,
    /// Unicode scalar values.
    Character,
    /// Fixed-width signed integers.
    SignedInteger,
    /// Fixed-width unsigned integers.
    UnsignedInteger,
    /// Primitive binary floating-point numbers.
    FloatingPoint,
    /// UTF-8 text values.
    Text,
    /// Calendar, clock, local date-time, and UTC instant values.
    Temporal,
    /// Arbitrary-precision integer and decimal numbers.
    BigNumber,
    /// Non-negative time spans.
    Duration,
    /// Resource locators.
    Locator,
    /// Map and JSON structured values.
    Structured,
}
