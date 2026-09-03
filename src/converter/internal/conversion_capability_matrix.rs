// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Canonical matrix of implemented conversion paths.

use crate::DataType;

/// Reports whether an implementation path exists between two available types.
///
/// Type availability remains the responsibility of the public capability
/// facade. Keeping only path topology here makes this match the single source
/// of truth for converter dispatch relationships.
pub(in crate::converter) fn supports_conversion(from: DataType, to: DataType) -> bool {
    if from == to {
        return true;
    }

    match (from, to) {
        (DataType::String, DataType::StringMap) | (DataType::StringMap, DataType::String) => {
            cfg!(feature = "json")
        }
        (DataType::String, _) | (_, DataType::String) => true,
        (DataType::StringMap, DataType::Json) => cfg!(feature = "json"),
        (DataType::Bool | DataType::Char, target) => target.is_numeric(),
        (
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
            | DataType::BigInteger,
            target,
        ) => target.is_numeric() || matches!(target, DataType::Bool | DataType::Duration),
        (DataType::Float32 | DataType::Float64 | DataType::BigDecimal, target) => target.is_numeric(),
        (DataType::Duration, target) => target.is_integer() || target == DataType::BigInteger,
        _ => false,
    }
}
