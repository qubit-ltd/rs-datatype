// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for feature-aware conversion capability discovery.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::time::Duration;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "chrono")]
use chrono::DateTime;
#[cfg(feature = "chrono")]
use chrono::NaiveDate;
#[cfg(feature = "chrono")]
use chrono::NaiveDateTime;
#[cfg(feature = "chrono")]
use chrono::NaiveTime;
#[cfg(feature = "chrono")]
use chrono::Utc;
#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
use qubit_datatype::ConversionCapabilities;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConversionTarget;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::DataTypeOf;
#[cfg(feature = "json")]
use serde_json::Value;
#[cfg(feature = "url")]
use url::Url;

/// Converts one representative source to a runtime-selected available target.
fn convert_to(source: &DataConverter<'_>, target: DataType) -> Option<Result<(), DataConversionError>> {
    macro_rules! convert {
        ($target:ty) => {
            Some(source.to::<$target>().map(drop))
        };
    }

    match target {
        DataType::Bool => convert!(bool),
        DataType::Char => convert!(char),
        DataType::Int8 => convert!(i8),
        DataType::Int16 => convert!(i16),
        DataType::Int32 => convert!(i32),
        DataType::Int64 => convert!(i64),
        DataType::Int128 => convert!(i128),
        DataType::UInt8 => convert!(u8),
        DataType::UInt16 => convert!(u16),
        DataType::UInt32 => convert!(u32),
        DataType::UInt64 => convert!(u64),
        DataType::UInt128 => convert!(u128),
        DataType::Float32 => convert!(f32),
        DataType::Float64 => convert!(f64),
        DataType::String => convert!(String),
        #[cfg(feature = "chrono")]
        DataType::Date => convert!(NaiveDate),
        #[cfg(feature = "chrono")]
        DataType::Time => convert!(NaiveTime),
        #[cfg(feature = "chrono")]
        DataType::DateTime => convert!(NaiveDateTime),
        #[cfg(feature = "chrono")]
        DataType::Instant => convert!(DateTime<Utc>),
        #[cfg(feature = "big-integer")]
        DataType::BigInteger => convert!(BigInt),
        #[cfg(feature = "big-decimal")]
        DataType::BigDecimal => convert!(BigDecimal),
        DataType::Duration => convert!(Duration),
        #[cfg(feature = "url")]
        DataType::Url => convert!(Url),
        DataType::StringMap => convert!(HashMap<String, String>),
        #[cfg(feature = "json")]
        DataType::Json => convert!(Value),
        #[allow(unreachable_patterns)]
        _ => None,
    }
}

/// Builds one valid concrete source for every type available in this build.
fn representative_sources() -> Vec<DataConverter<'static>> {
    let mut sources = vec![
        DataConverter::from(true),
        DataConverter::from('1'),
        DataConverter::from(1_i8),
        DataConverter::from(1_i16),
        DataConverter::from(1_i32),
        DataConverter::from(1_i64),
        DataConverter::from(1_i128),
        DataConverter::from(1_u8),
        DataConverter::from(1_u16),
        DataConverter::from(1_u32),
        DataConverter::from(1_u64),
        DataConverter::from(1_u128),
        DataConverter::from(1.0_f32),
        DataConverter::from(1.0_f64),
        DataConverter::from(String::from("1")),
        DataConverter::from(Duration::from_secs(1)),
    ];
    sources.push(DataConverter::from(HashMap::from([(
        String::from("key"),
        String::from("value"),
    )])));
    #[cfg(feature = "chrono")]
    {
        sources.push(DataConverter::from(
            NaiveDate::from_ymd_opt(2026, 1, 2).expect("representative date must be valid"),
        ));
        sources.push(DataConverter::from(
            NaiveTime::from_hms_opt(3, 4, 5).expect("representative time must be valid"),
        ));
        sources.push(DataConverter::from(
            NaiveDate::from_ymd_opt(2026, 1, 2)
                .expect("representative date must be valid")
                .and_hms_opt(3, 4, 5)
                .expect("representative date-time must be valid"),
        ));
        sources.push(DataConverter::from(
            DateTime::from_timestamp(1, 0).expect("representative instant must be valid"),
        ));
    }
    #[cfg(feature = "big-integer")]
    sources.push(DataConverter::from(BigInt::from(1)));
    #[cfg(feature = "big-decimal")]
    sources.push(DataConverter::from(BigDecimal::from(1)));
    #[cfg(feature = "url")]
    sources.push(DataConverter::from(
        Url::parse("https://example.com").expect("representative URL must be valid"),
    ));
    #[cfg(feature = "json")]
    sources.push(DataConverter::from(Value::Bool(true)));
    sources
}

/// Verifies the current build reports the core converter paths.
#[test]
fn test_current_capabilities_report_core_paths() {
    let capabilities = ConversionCapabilities::current();

    assert!(capabilities.is_available(DataType::String));
    assert!(capabilities.is_available(DataType::StringMap));
    assert!(capabilities.supports(DataType::String, DataType::Int32));
    assert!(capabilities.supports(DataType::Duration, DataType::String));
    assert!(!capabilities.supports(DataType::StringMap, DataType::Bool));
}

/// Verifies supported targets follow stable declaration order without an
/// intermediate allocation in the API.
#[test]
fn test_supported_targets_follow_data_type_order() {
    let capabilities = ConversionCapabilities::current();
    let targets: Vec<DataType> = capabilities.supported_targets(DataType::Char).collect();
    let mut expected = vec![
        DataType::Char,
        DataType::Int8,
        DataType::Int16,
        DataType::Int32,
        DataType::Int64,
        DataType::Int128,
        DataType::UInt8,
        DataType::UInt16,
        DataType::UInt32,
        DataType::UInt64,
        DataType::UInt128,
        DataType::Float32,
        DataType::Float64,
        DataType::String,
    ];
    expected.extend(
        [DataType::BigInteger, DataType::BigDecimal]
            .into_iter()
            .filter(|data_type| capabilities.is_available(*data_type)),
    );

    assert_eq!(targets, expected);
}

/// Verifies rich data types are available exactly when their feature is
/// enabled.
#[test]
fn test_rich_type_availability_matches_features() {
    let capabilities = ConversionCapabilities::current();
    let cases = [
        (DataType::Date, cfg!(feature = "chrono")),
        (DataType::Time, cfg!(feature = "chrono")),
        (DataType::DateTime, cfg!(feature = "chrono")),
        (DataType::Instant, cfg!(feature = "chrono")),
        (DataType::BigInteger, cfg!(feature = "big-integer")),
        (DataType::BigDecimal, cfg!(feature = "big-decimal")),
        (DataType::Url, cfg!(feature = "url")),
        (DataType::Json, cfg!(feature = "json")),
    ];

    for (data_type, expected) in cases {
        assert_eq!(capabilities.is_available(data_type), expected, "{data_type}");
        assert_eq!(capabilities.supports(data_type, data_type), expected, "{data_type}",);
        assert_eq!(
            capabilities.supports(DataType::String, data_type),
            expected,
            "{data_type}",
        );
    }
}

/// Verifies unsupported or unavailable source types have no reported targets.
#[test]
fn test_supported_targets_are_empty_for_unavailable_source() {
    let capabilities = ConversionCapabilities::current();

    if !cfg!(feature = "json") {
        assert_eq!(capabilities.supported_targets(DataType::Json).count(), 0);
    }
}

/// Verifies capability reporting agrees with concrete dispatch for every
/// represented source and target in the current feature combination.
#[test]
fn test_capabilities_match_representative_conversion_dispatch() {
    let capabilities = ConversionCapabilities::current();
    let sources = representative_sources();
    let represented: HashSet<DataType> = sources.iter().map(DataConverter::data_type).collect();
    let available: HashSet<DataType> = DataType::ALL
        .iter()
        .copied()
        .filter(|data_type| capabilities.is_available(*data_type))
        .collect();

    assert_eq!(represented, available);

    for source in sources {
        let from = source.data_type();
        for target in DataType::ALL.iter().copied() {
            let Some(result) = convert_to(&source, target) else {
                assert!(!capabilities.is_available(target), "{target}");
                continue;
            };
            let reported = capabilities.supports(from, target);
            match result {
                Ok(()) => assert!(reported, "successful path {from} -> {target} must be reported"),
                Err(error) => assert_eq!(
                    error.kind() != DataConversionErrorKind::Unsupported,
                    reported,
                    "capability mismatch for {from} -> {target}: {error}",
                ),
            }
        }
    }
}

fn assert_supported_contract<T>(source: DataConverter<'_>, expected: T)
where
    T: DataConversionTarget + DataTypeOf + Debug + PartialEq,
{
    let from = source.data_type();
    let to = T::DATA_TYPE;
    assert!(ConversionCapabilities::current().supports(from, to), "{from} -> {to}");
    assert_eq!(source.to::<T>(), Ok(expected), "{from} -> {to}");
}

fn assert_unsupported_contract<T>(source: DataConverter<'_>, from: DataType, to: DataType)
where
    T: DataConversionTarget + DataTypeOf + Debug,
{
    let error = source.to::<T>().unwrap_err();
    assert!(!ConversionCapabilities::current().supports(from, to), "{from} -> {to}");
    assert_eq!(error.kind(), DataConversionErrorKind::Unsupported);
    assert_eq!(error.from_type(), Some(from));
    assert_eq!(error.to_type(), to);
}

macro_rules! assert_numeric_targets {
    ($source:expr, $value:expr) => {{
        assert_supported_contract($source, $value as i8);
        assert_supported_contract($source, $value as i16);
        assert_supported_contract($source, $value as i32);
        assert_supported_contract($source, $value as i64);
        assert_supported_contract($source, $value as i128);
        assert_supported_contract($source, $value as u8);
        assert_supported_contract($source, $value as u16);
        assert_supported_contract($source, $value as u32);
        assert_supported_contract($source, $value as u64);
        assert_supported_contract($source, $value as u128);
        assert_supported_contract($source, $value as f32);
        assert_supported_contract($source, $value as f64);
    }};
}

macro_rules! assert_integer_targets {
    ($source:expr) => {{
        assert_supported_contract($source, 1_i8);
        assert_supported_contract($source, 1_i16);
        assert_supported_contract($source, 1_i32);
        assert_supported_contract($source, 1_i64);
        assert_supported_contract($source, 1_i128);
        assert_supported_contract($source, 1_u8);
        assert_supported_contract($source, 1_u16);
        assert_supported_contract($source, 1_u32);
        assert_supported_contract($source, 1_u64);
        assert_supported_contract($source, 1_u128);
    }};
}

macro_rules! assert_integer_duration_paths {
    ($value:expr) => {{
        assert_supported_contract(DataConverter::from($value), true);
        assert_supported_contract(DataConverter::from($value), Duration::from_millis(1));
    }};
}

#[test]
fn test_independent_core_conversion_contracts() {
    assert_numeric_targets!(DataConverter::from(1_i8), 1);
    assert_numeric_targets!(DataConverter::from(1_i16), 1);
    assert_numeric_targets!(DataConverter::from(1_i32), 1);
    assert_numeric_targets!(DataConverter::from(1_i64), 1);
    assert_numeric_targets!(DataConverter::from(1_i128), 1);
    assert_numeric_targets!(DataConverter::from(1_u8), 1);
    assert_numeric_targets!(DataConverter::from(1_u16), 1);
    assert_numeric_targets!(DataConverter::from(1_u32), 1);
    assert_numeric_targets!(DataConverter::from(1_u64), 1);
    assert_numeric_targets!(DataConverter::from(1_u128), 1);
    assert_numeric_targets!(DataConverter::from(1.0_f32), 1);
    assert_numeric_targets!(DataConverter::from(1.0_f64), 1);
    assert_numeric_targets!(DataConverter::from(true), 1);
    assert_numeric_targets!(DataConverter::from('A'), 65);

    assert_integer_duration_paths!(1_i8);
    assert_integer_duration_paths!(1_i16);
    assert_integer_duration_paths!(1_i32);
    assert_integer_duration_paths!(1_i64);
    assert_integer_duration_paths!(1_i128);
    assert_integer_duration_paths!(1_u8);
    assert_integer_duration_paths!(1_u16);
    assert_integer_duration_paths!(1_u32);
    assert_integer_duration_paths!(1_u64);
    assert_integer_duration_paths!(1_u128);
    assert_integer_targets!(DataConverter::from(Duration::from_millis(1)));

    assert_supported_contract(DataConverter::from("true"), true);
    assert_supported_contract(DataConverter::from("A"), 'A');
    assert_supported_contract(DataConverter::from("1"), 1_i8);
    assert_supported_contract(DataConverter::from("1"), 1_i16);
    assert_supported_contract(DataConverter::from("1"), 1_i32);
    assert_supported_contract(DataConverter::from("1"), 1_i64);
    assert_supported_contract(DataConverter::from("1"), 1_i128);
    assert_supported_contract(DataConverter::from("1"), 1_u8);
    assert_supported_contract(DataConverter::from("1"), 1_u16);
    assert_supported_contract(DataConverter::from("1"), 1_u32);
    assert_supported_contract(DataConverter::from("1"), 1_u64);
    assert_supported_contract(DataConverter::from("1"), 1_u128);
    assert_supported_contract(DataConverter::from("1"), 1.0_f32);
    assert_supported_contract(DataConverter::from("1"), 1.0_f64);
    assert_supported_contract(DataConverter::from("1ms"), Duration::from_millis(1));
    assert_supported_contract(DataConverter::from(true), String::from("true"));
    assert_supported_contract(DataConverter::from('A'), String::from("A"));
    assert_supported_contract(DataConverter::from(1_i8), String::from("1"));
    assert_supported_contract(DataConverter::from(1_i16), String::from("1"));
    assert_supported_contract(DataConverter::from(1_i32), String::from("1"));
    assert_supported_contract(DataConverter::from(1_i64), String::from("1"));
    assert_supported_contract(DataConverter::from(1_i128), String::from("1"));
    assert_supported_contract(DataConverter::from(1_u8), String::from("1"));
    assert_supported_contract(DataConverter::from(1_u16), String::from("1"));
    assert_supported_contract(DataConverter::from(1_u32), String::from("1"));
    assert_supported_contract(DataConverter::from(1_u64), String::from("1"));
    assert_supported_contract(DataConverter::from(1_u128), String::from("1"));
    assert_supported_contract(DataConverter::from(1.0_f32), String::from("1"));
    assert_supported_contract(DataConverter::from(1.0_f64), String::from("1"));

    assert_unsupported_contract::<Duration>(DataConverter::from(true), DataType::Bool, DataType::Duration);
    assert_unsupported_contract::<bool>(DataConverter::from('A'), DataType::Char, DataType::Bool);
    assert_unsupported_contract::<Duration>(DataConverter::from(1.0_f64), DataType::Float64, DataType::Duration);
    assert_unsupported_contract::<bool>(
        DataConverter::from(HashMap::<String, String>::new()),
        DataType::StringMap,
        DataType::Bool,
    );
}

#[cfg(feature = "big-integer")]
#[test]
fn test_big_integer_conversion_contracts() {
    assert_supported_contract(DataConverter::from(BigInt::from(1)), 1_i64);
    assert_supported_contract(DataConverter::from(BigInt::from(1)), true);
    assert_supported_contract(DataConverter::from(BigInt::from(1)), Duration::from_millis(1));
    assert_supported_contract(DataConverter::from(true), BigInt::from(1));
    assert_supported_contract(DataConverter::from(Duration::from_millis(1)), BigInt::from(1));
}

#[cfg(feature = "big-decimal")]
#[test]
fn test_big_decimal_conversion_contracts() {
    assert_supported_contract(DataConverter::from(BigDecimal::from(1)), 1_i64);
    assert_supported_contract(DataConverter::from(BigDecimal::from(1)), 1.0_f64);
    assert_supported_contract(DataConverter::from(1_i64), BigDecimal::from(1));
}

#[cfg(all(feature = "big-integer", feature = "big-decimal"))]
#[test]
fn test_big_number_cross_conversion_contracts() {
    assert_supported_contract(DataConverter::from(BigInt::from(1)), BigDecimal::from(1));
    assert_supported_contract(DataConverter::from(BigDecimal::from(1)), BigInt::from(1));
}

#[cfg(feature = "chrono")]
#[test]
fn test_chrono_conversion_contracts() {
    let date = NaiveDate::from_ymd_opt(2026, 1, 2).unwrap();
    let time = NaiveTime::from_hms_opt(3, 4, 5).unwrap();
    let datetime = date.and_time(time);
    let instant = DateTime::parse_from_rfc3339("2026-01-02T03:04:05Z")
        .unwrap()
        .with_timezone(&Utc);
    assert_supported_contract(DataConverter::from("2026-01-02"), date);
    assert_supported_contract(DataConverter::from("03:04:05"), time);
    assert_supported_contract(DataConverter::from("2026-01-02T03:04:05"), datetime);
    assert_supported_contract(DataConverter::from("2026-01-02T03:04:05Z"), instant);
    assert_supported_contract(DataConverter::from(date), date);
    assert_supported_contract(DataConverter::from(time), time);
    assert_supported_contract(DataConverter::from(datetime), datetime);
    assert_supported_contract(DataConverter::from(instant), instant);
    assert_supported_contract(DataConverter::from(date), String::from("2026-01-02"));
    assert_supported_contract(DataConverter::from(time), String::from("03:04:05"));
    assert_supported_contract(DataConverter::from(datetime), String::from("2026-01-02T03:04:05"));
    assert_supported_contract(DataConverter::from(instant), String::from("2026-01-02T03:04:05+00:00"));
    assert_unsupported_contract::<i32>(DataConverter::from(date), DataType::Date, DataType::Int32);
}

#[cfg(feature = "url")]
#[test]
fn test_url_conversion_contracts() {
    let url = Url::parse("https://example.com/").unwrap();
    assert_supported_contract(DataConverter::from("https://example.com/"), url.clone());
    assert_supported_contract(DataConverter::from(url.clone()), url.clone());
    assert_supported_contract(DataConverter::from(url), String::from("https://example.com/"));
}

#[cfg(feature = "json")]
#[test]
fn test_json_conversion_contracts() {
    let map = HashMap::from([(String::from("k"), String::from("v"))]);
    let value = serde_json::json!({"k": "v"});
    assert_supported_contract(DataConverter::from("{\"k\":\"v\"}"), map.clone());
    assert_supported_contract(DataConverter::from(map.clone()), value.clone());
    assert_supported_contract(DataConverter::from(map), String::from("{\"k\":\"v\"}"));
    assert_supported_contract(DataConverter::from("{\"k\":\"v\"}"), value.clone());
    assert_supported_contract(DataConverter::from(value.clone()), value.clone());
    assert_supported_contract(DataConverter::from(value), String::from("{\"k\":\"v\"}"));
}
