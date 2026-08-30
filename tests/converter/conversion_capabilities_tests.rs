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
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
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

/// Verifies the capability matrix agrees with every concrete conversion
/// implementation available in the current feature combination.
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
