// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Compile-time Data Type Mapping
//!
//! Provides the `DataTypeOf` trait and implementations to map Rust types to
//! `DataType`.

use super::data_type::DataType;
#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "chrono")]
use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
use std::collections::HashMap;
use std::time::Duration;
#[cfg(feature = "url")]
use url::Url;

/// Maps a concrete Rust type to its runtime [`DataType`] descriptor.
///
/// This trait carries type metadata only; it does not convert values. Generic
/// APIs use it when they need a stable target descriptor without receiving a
/// value of that type. Implementations for third-party types are enabled by
/// their corresponding crate features. The standard-library mapping for
/// `HashMap<String, String>` is always available because it does not depend on
/// JSON support.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use qubit_datatype::{DataType, DataTypeOf};
///
/// fn declared_type<T: DataTypeOf>() -> DataType {
///     T::DATA_TYPE
/// }
///
/// assert_eq!(declared_type::<u64>(), DataType::UInt64);
/// assert_eq!(
///     declared_type::<HashMap<String, String>>(),
///     DataType::StringMap,
/// );
/// ```
///
/// Platform-sized integers deliberately have no runtime descriptor:
///
/// ```compile_fail
/// use qubit_datatype::DataTypeOf;
///
/// let _ = usize::DATA_TYPE;
/// ```
pub trait DataTypeOf {
    /// The stable [`DataType`] corresponding to `Self`.
    const DATA_TYPE: DataType;
}

macro_rules! impl_data_type_of {
    ($( $(#[$meta:meta])* $ty:ty => $variant:ident ),+ $(,)?) => {
        $(
            $(#[$meta])*
            impl DataTypeOf for $ty {
                const DATA_TYPE: DataType = DataType::$variant;
            }
        )+
    };
}

impl_data_type_of! {
    bool => Bool,
    char => Char,
    i8 => Int8,
    i16 => Int16,
    i32 => Int32,
    i64 => Int64,
    i128 => Int128,
    u8 => UInt8,
    u16 => UInt16,
    u32 => UInt32,
    u64 => UInt64,
    u128 => UInt128,
    f32 => Float32,
    f64 => Float64,
    String => String,
    #[cfg(feature = "chrono")]
    NaiveDate => Date,
    #[cfg(feature = "chrono")]
    NaiveTime => Time,
    #[cfg(feature = "chrono")]
    NaiveDateTime => DateTime,
    #[cfg(feature = "chrono")]
    DateTime<Utc> => Instant,
    #[cfg(feature = "big-integer")]
    BigInt => BigInteger,
    #[cfg(feature = "big-decimal")]
    BigDecimal => BigDecimal,
    Duration => Duration,
    #[cfg(feature = "url")]
    Url => Url,
    HashMap<String, String> => StringMap,
    #[cfg(feature = "json")]
    serde_json::Value => Json,
}
