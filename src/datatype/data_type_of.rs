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
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
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

impl DataTypeOf for bool {
    const DATA_TYPE: DataType = DataType::Bool;
}
impl DataTypeOf for char {
    const DATA_TYPE: DataType = DataType::Char;
}
impl DataTypeOf for i8 {
    const DATA_TYPE: DataType = DataType::Int8;
}
impl DataTypeOf for i16 {
    const DATA_TYPE: DataType = DataType::Int16;
}
impl DataTypeOf for i32 {
    const DATA_TYPE: DataType = DataType::Int32;
}
impl DataTypeOf for i64 {
    const DATA_TYPE: DataType = DataType::Int64;
}
impl DataTypeOf for i128 {
    const DATA_TYPE: DataType = DataType::Int128;
}
impl DataTypeOf for u8 {
    const DATA_TYPE: DataType = DataType::UInt8;
}
impl DataTypeOf for u16 {
    const DATA_TYPE: DataType = DataType::UInt16;
}
impl DataTypeOf for u32 {
    const DATA_TYPE: DataType = DataType::UInt32;
}
impl DataTypeOf for u64 {
    const DATA_TYPE: DataType = DataType::UInt64;
}
impl DataTypeOf for u128 {
    const DATA_TYPE: DataType = DataType::UInt128;
}
impl DataTypeOf for f32 {
    const DATA_TYPE: DataType = DataType::Float32;
}
impl DataTypeOf for f64 {
    const DATA_TYPE: DataType = DataType::Float64;
}
impl DataTypeOf for String {
    const DATA_TYPE: DataType = DataType::String;
}
#[cfg(feature = "chrono")]
impl DataTypeOf for NaiveDate {
    const DATA_TYPE: DataType = DataType::Date;
}
#[cfg(feature = "chrono")]
impl DataTypeOf for NaiveTime {
    const DATA_TYPE: DataType = DataType::Time;
}
#[cfg(feature = "chrono")]
impl DataTypeOf for NaiveDateTime {
    const DATA_TYPE: DataType = DataType::DateTime;
}
#[cfg(feature = "chrono")]
impl DataTypeOf for DateTime<Utc> {
    const DATA_TYPE: DataType = DataType::Instant;
}
#[cfg(feature = "big-integer")]
impl DataTypeOf for BigInt {
    const DATA_TYPE: DataType = DataType::BigInteger;
}
#[cfg(feature = "big-decimal")]
impl DataTypeOf for BigDecimal {
    const DATA_TYPE: DataType = DataType::BigDecimal;
}
impl DataTypeOf for Duration {
    const DATA_TYPE: DataType = DataType::Duration;
}
#[cfg(feature = "url")]
impl DataTypeOf for Url {
    const DATA_TYPE: DataType = DataType::Url;
}
impl DataTypeOf for HashMap<String, String> {
    const DATA_TYPE: DataType = DataType::StringMap;
}
#[cfg(feature = "json")]
impl DataTypeOf for serde_json::Value {
    const DATA_TYPE: DataType = DataType::Json;
}
