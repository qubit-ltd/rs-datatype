// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Canonical matrix of implemented conversion paths.

use crate::DataType;

/// Describes one family of built-in conversion edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConversionEdge {
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

/// The single declaration of built-in conversion edge families.
const CONVERSION_EDGES: &[ConversionEdge] = &[
    ConversionEdge::String,
    ConversionEdge::StringMapJson,
    ConversionEdge::BooleanCharacterNumeric,
    ConversionEdge::IntegerNumericBooleanDuration,
    ConversionEdge::FloatNumeric,
    ConversionEdge::DurationInteger,
];

/// Reports whether an implementation path exists between two available types.
///
/// Type availability remains the responsibility of the public capability
/// facade. The built-in target adapter consults this same table before
/// dispatching to a conversion implementation, so capability discovery and
/// built-in dispatch cannot register different edge families.
pub(in crate::converter) fn supports_conversion(from: DataType, to: DataType) -> bool {
    if from == to {
        return true;
    }

    CONVERSION_EDGES.iter().any(|edge| match edge {
        ConversionEdge::String => {
            let string_map_pair = (from == DataType::String && to == DataType::StringMap)
                || (from == DataType::StringMap && to == DataType::String);
            (!string_map_pair && (from == DataType::String || to == DataType::String))
                || (cfg!(feature = "json") && string_map_pair)
        }
        ConversionEdge::StringMapJson => cfg!(feature = "json") && from == DataType::StringMap && to == DataType::Json,
        ConversionEdge::BooleanCharacterNumeric => matches!(from, DataType::Bool | DataType::Char) && to.is_numeric(),
        ConversionEdge::IntegerNumericBooleanDuration => {
            matches!(
                from,
                DataType::Int8
                    | DataType::Int16
                    | DataType::Int32
                    | DataType::Int64
                    | DataType::Int128
                    | DataType::UInt8
                    | DataType::UInt16
                    | DataType::UInt32
                    | DataType::UInt64
                    | DataType::UInt128
                    | DataType::BigInteger
            ) && (to.is_numeric() || matches!(to, DataType::Bool | DataType::Duration))
        }
        ConversionEdge::FloatNumeric => {
            matches!(from, DataType::Float32 | DataType::Float64 | DataType::BigDecimal) && to.is_numeric()
        }
        ConversionEdge::DurationInteger => {
            from == DataType::Duration && (to.is_integer() || to == DataType::BigInteger)
        }
    })
}
