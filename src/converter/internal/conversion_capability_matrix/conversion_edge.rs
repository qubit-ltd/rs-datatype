// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion edge families used by the built-in capability matrix.

/// Describes one family of built-in conversion edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConversionEdge {
    /// Converts any source to or from String.
    String,
    /// Converts StringMap to JSON when JSON support is compiled in.
    StringMapJson,
    /// Converts booleans and characters to numeric types.
    BooleanCharacterNumeric,
    /// Converts integer types to numeric, boolean, and duration types.
    IntegerNumericBooleanDuration,
    /// Converts floating-point and BigDecimal types to numeric types.
    FloatNumeric,
    /// Converts durations to integer types.
    DurationInteger,
}
