// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Source constructors for DataConverter.

use std::borrow::Cow;
use std::collections::HashMap;
use std::time::Duration;

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
#[cfg(feature = "url")]
use url::Url;

use super::DataConverter;

macro_rules! impl_from_copy {
    ($source:ty, $variant:ident) => {
        impl<'a> From<$source> for DataConverter<'a> {
            #[inline]
            fn from(value: $source) -> Self {
                Self::$variant(value)
            }
        }
        impl<'a> From<&'a $source> for DataConverter<'a> {
            #[inline]
            fn from(value: &'a $source) -> Self {
                Self::$variant(*value)
            }
        }
    };
}

macro_rules! impl_from_cow {
    ($source:ty, $variant:ident) => {
        impl<'a> From<$source> for DataConverter<'a> {
            #[inline]
            fn from(value: $source) -> Self {
                Self::$variant(Cow::Owned(value))
            }
        }
        impl<'a> From<&'a $source> for DataConverter<'a> {
            #[inline]
            fn from(value: &'a $source) -> Self {
                Self::$variant(Cow::Borrowed(value))
            }
        }
    };
}

impl_from_copy!(bool, Bool);
impl_from_copy!(char, Char);
impl_from_copy!(i8, Int8);
impl_from_copy!(i16, Int16);
impl_from_copy!(i32, Int32);
impl_from_copy!(i64, Int64);
impl_from_copy!(i128, Int128);
impl_from_copy!(u8, UInt8);
impl_from_copy!(u16, UInt16);
impl_from_copy!(u32, UInt32);
impl_from_copy!(u64, UInt64);
impl_from_copy!(u128, UInt128);
impl_from_copy!(f32, Float32);
impl_from_copy!(f64, Float64);
#[cfg(feature = "chrono")]
impl_from_copy!(NaiveDate, Date);
#[cfg(feature = "chrono")]
impl_from_copy!(NaiveTime, Time);
#[cfg(feature = "chrono")]
impl_from_copy!(NaiveDateTime, DateTime);
#[cfg(feature = "chrono")]
impl_from_copy!(DateTime<Utc>, Instant);
impl_from_copy!(Duration, Duration);
#[cfg(feature = "big-number")]
impl_from_cow!(BigInt, BigInteger);
#[cfg(feature = "big-number")]
impl_from_cow!(BigDecimal, BigDecimal);
#[cfg(feature = "url")]
impl_from_cow!(Url, Url);
impl_from_cow!(HashMap<String, String>, StringMap);
#[cfg(feature = "json")]
impl_from_cow!(serde_json::Value, Json);

impl<'a> From<&'a str> for DataConverter<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::String(Cow::Borrowed(value))
    }
}

impl<'a> From<&'a String> for DataConverter<'a> {
    #[inline]
    fn from(value: &'a String) -> Self {
        Self::String(Cow::Borrowed(value))
    }
}

impl<'a> From<String> for DataConverter<'a> {
    #[inline]
    fn from(value: String) -> Self {
        Self::String(Cow::Owned(value))
    }
}
