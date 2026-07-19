// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Crate-private declarations shared by data type mapping consumers.

/// Applies a consumer macro to every supported Rust data type mapping.
///
/// Each mapping supplies optional feature attributes, the [`DataType`] variant,
/// the Rust type, and the `DataConverter` storage strategy. Consumers must
/// accept `$(#[$meta])* ($variant, $source, $strategy)` entries.
///
/// [`DataType`]: super::DataType
macro_rules! for_each_data_type_mapping {
    ($consumer:ident $(, $argument:tt)*) => {
        $consumer! {
            $($argument)*;
            (Bool, bool, copy),
            (Char, char, copy),
            (Int8, i8, copy),
            (Int16, i16, copy),
            (Int32, i32, copy),
            (Int64, i64, copy),
            (Int128, i128, copy),
            (UInt8, u8, copy),
            (UInt16, u16, copy),
            (UInt32, u32, copy),
            (UInt64, u64, copy),
            (UInt128, u128, copy),
            (Float32, f32, copy),
            (Float64, f64, copy),
            (String, ::std::string::String, string),
            #[cfg(feature = "chrono")]
            (Date, ::chrono::NaiveDate, copy),
            #[cfg(feature = "chrono")]
            (Time, ::chrono::NaiveTime, copy),
            #[cfg(feature = "chrono")]
            (DateTime, ::chrono::NaiveDateTime, copy),
            #[cfg(feature = "chrono")]
            (Instant, ::chrono::DateTime<::chrono::Utc>, copy),
            #[cfg(feature = "big-integer")]
            (BigInteger, ::num_bigint::BigInt, cow),
            #[cfg(feature = "big-decimal")]
            (BigDecimal, ::bigdecimal::BigDecimal, cow),
            (Duration, ::std::time::Duration, copy),
            #[cfg(feature = "url")]
            (Url, ::url::Url, cow),
            (
                StringMap,
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::string::String,
                >,
                cow
            ),
            #[cfg(feature = "json")]
            (Json, ::serde_json::Value, cow),
        }
    };
}

pub(crate) use for_each_data_type_mapping;
