/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Compile-time Data Type Mapping
//!
//! Provides the `DataTypeOf` trait and implementations to map Rust types to `DataType`.
//!
//! # Author
//!
//! Haixing Hu

use super::data_type::DataType;
use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
use num_bigint::BigInt;
use std::collections::HashMap;
use std::time::Duration;
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
impl DataTypeOf for NaiveDate {
    const DATA_TYPE: DataType = DataType::Date;
}
impl DataTypeOf for NaiveTime {
    const DATA_TYPE: DataType = DataType::Time;
}
impl DataTypeOf for NaiveDateTime {
    const DATA_TYPE: DataType = DataType::DateTime;
}
impl DataTypeOf for DateTime<Utc> {
    const DATA_TYPE: DataType = DataType::Instant;
}
impl DataTypeOf for BigInt {
    const DATA_TYPE: DataType = DataType::BigInteger;
}
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
impl DataTypeOf for Url {
    const DATA_TYPE: DataType = DataType::Url;
}
impl DataTypeOf for HashMap<String, String> {
    const DATA_TYPE: DataType = DataType::StringMap;
}
impl DataTypeOf for serde_json::Value {
    const DATA_TYPE: DataType = DataType::Json;
}
