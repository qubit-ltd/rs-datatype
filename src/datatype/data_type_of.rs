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
#[cfg(feature = "big-number")]
use bigdecimal::BigDecimal;
#[cfg(feature = "chrono")]
use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
#[cfg(feature = "big-number")]
use num_bigint::BigInt;
#[cfg(feature = "json")]
use std::collections::HashMap;
use std::time::Duration;
#[cfg(feature = "url")]
use url::Url;

/// Marker trait for mapping concrete Rust types to `DataType`.
pub trait DataTypeOf {
    /// The `DataType` corresponding to this Rust type.
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
#[cfg(feature = "big-number")]
impl DataTypeOf for BigInt {
    const DATA_TYPE: DataType = DataType::BigInteger;
}
#[cfg(feature = "big-number")]
impl DataTypeOf for BigDecimal {
    const DATA_TYPE: DataType = DataType::BigDecimal;
}
impl DataTypeOf for isize {
    const DATA_TYPE: DataType = DataType::IntSize;
}
impl DataTypeOf for usize {
    const DATA_TYPE: DataType = DataType::UIntSize;
}
impl DataTypeOf for Duration {
    const DATA_TYPE: DataType = DataType::Duration;
}
#[cfg(feature = "url")]
impl DataTypeOf for Url {
    const DATA_TYPE: DataType = DataType::Url;
}
#[cfg(feature = "json")]
impl DataTypeOf for HashMap<String, String> {
    const DATA_TYPE: DataType = DataType::StringMap;
}
#[cfg(feature = "json")]
impl DataTypeOf for serde_json::Value {
    const DATA_TYPE: DataType = DataType::Json;
}
