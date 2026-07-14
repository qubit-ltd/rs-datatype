// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Numeric conversion tests.

use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
use num_bigint::BigInt;
use qubit_datatype::{
    DataConversionError,
    InvalidValueReason,
    DataConversionOptions,
    DataConverter,
    DataType,
    NumericConversionPolicy,
};
use url::Url;

/// Creates a very large BigInt for overflow-oriented tests.
fn create_huge_bigint() -> BigInt {
    BigInt::parse_bytes(b"10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10)
        .expect("huge BigInt literal should parse")
}

/// Test conversions from borrowed and owned string values.
#[test]
fn test_data_converter_from_string_converts_to_numbers_and_bool() {
    let port: u16 = DataConverter::from("8080")
        .to()
        .expect("string port should convert to u16");
    assert_eq!(port, 8080);

    let enabled: bool = DataConverter::from("TRUE")
        .to()
        .expect("string true should convert to bool");
    assert!(enabled);

    let disabled: bool = DataConverter::from(String::from("0"))
        .to()
        .expect("owned string zero should convert to bool");
    assert!(!disabled);

    let negative: i32 = DataConverter::from("-42")
        .to()
        .expect("negative string should convert to i32");
    assert_eq!(negative, -42);
}

/// Test signed integer targets across all supported source variants.
#[test]
fn test_data_converter_signed_integer_targets_accept_supported_sources() {
    let big_int = BigInt::from(20);
    let big_decimal = BigDecimal::from(21);

    let cases = [
        (DataConverter::from(true), 1i128),
        (DataConverter::from('A'), 65),
        (DataConverter::from(-1i8), -1),
        (DataConverter::from(2i16), 2),
        (DataConverter::from(3i32), 3),
        (DataConverter::from(4i64), 4),
        (DataConverter::from(5i128), 5),
        (DataConverter::from(6isize), 6),
        (DataConverter::from(7u8), 7),
        (DataConverter::from(8u16), 8),
        (DataConverter::from(9u32), 9),
        (DataConverter::from(10u64), 10),
        (DataConverter::from(11u128), 11),
        (DataConverter::from(12usize), 12),
        (DataConverter::from(13.0f32), 13),
        (DataConverter::from(14.0f64), 14),
        (DataConverter::from("15"), 15),
        (DataConverter::from(&big_int), 20),
        (DataConverter::from(&big_decimal), 21),
    ];
    for (source, expected) in cases {
        assert_eq!(
            source.to::<i128>().expect("source should convert to i128"),
            expected
        );
    }

    assert_eq!(
        DataConverter::from(120i16)
            .to::<i8>()
            .expect("i16 should convert to i8"),
        120
    );
    assert_eq!(
        DataConverter::from(32_000i32)
            .to::<i16>()
            .expect("i32 should convert to i16"),
        32_000
    );
    assert_eq!(
        DataConverter::from(1_000_000i64)
            .to::<i32>()
            .expect("i64 should convert to i32"),
        1_000_000
    );
    assert_eq!(
        DataConverter::from(1_000_000i128)
            .to::<isize>()
            .expect("i128 should convert to isize"),
        1_000_000isize
    );

    assert!(matches!(
        DataConverter::Empty(DataType::Int128).to::<i128>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::from("bad").to::<i128>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::InvalidSyntax { .. },
            ..
        })
    ));
    assert!(matches!(
        DataConverter::from(f64::MAX).to::<i128>(),
        Err(DataConversionError::InvalidValue { .. })
    ));
    let huge = create_huge_bigint();
    let huge_decimal = BigDecimal::from(huge.clone());
    assert!(matches!(
        DataConverter::from(&huge).to::<i128>(),
        Err(DataConversionError::InvalidValue { .. })
    ));
    assert!(matches!(
        DataConverter::from(&huge_decimal).to::<i128>(),
        Err(DataConversionError::InvalidValue { .. })
    ));
    assert!(matches!(
        DataConverter::from(
            NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date")
        )
        .to::<i128>(),
        Err(DataConversionError::Unsupported { .. })
    ));
}

/// Test unsigned integer targets across all supported source variants.
#[test]
fn test_data_converter_unsigned_integer_targets_accept_supported_sources() {
    let cases = [
        (DataConverter::from(true), 1u128),
        (DataConverter::from('A'), 65),
        (DataConverter::from(1i8), 1),
        (DataConverter::from(2i16), 2),
        (DataConverter::from(3i32), 3),
        (DataConverter::from(4i64), 4),
        (DataConverter::from(5i128), 5),
        (DataConverter::from(6isize), 6),
        (DataConverter::from(7u8), 7),
        (DataConverter::from(8u16), 8),
        (DataConverter::from(9u32), 9),
        (DataConverter::from(10u64), 10),
        (DataConverter::from(11u128), 11),
        (DataConverter::from(12usize), 12),
        (DataConverter::from("13"), 13),
    ];
    for (source, expected) in cases {
        assert_eq!(
            source.to::<u128>().expect("source should convert to u128"),
            expected
        );
    }

    assert_eq!(
        DataConverter::from(250u16)
            .to::<u8>()
            .expect("u16 should convert to u8"),
        250
    );
    assert_eq!(
        DataConverter::from(65_000u32)
            .to::<u16>()
            .expect("u32 should convert to u16"),
        65_000
    );
    assert_eq!(
        DataConverter::from(1_000_000u64)
            .to::<u32>()
            .expect("u64 should convert to u32"),
        1_000_000
    );
    assert_eq!(
        DataConverter::from(1_000_000u128)
            .to::<u64>()
            .expect("u128 should convert to u64"),
        1_000_000
    );
    assert_eq!(
        DataConverter::from(1_000usize)
            .to::<usize>()
            .expect("usize should convert to usize"),
        1_000
    );

    assert!(matches!(
        DataConverter::Empty(DataType::UInt128).to::<u128>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::from("bad").to::<u128>(),
        Err(DataConversionError::InvalidValue { .. })
    ));
    assert!(matches!(
        DataConverter::from(1.5f64).to::<u128>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::PrecisionLoss,
            ..
        })
    ));
}

/// Test floating point target conversions across supported source variants.
#[test]
fn test_data_converter_float_targets_accept_supported_sources() {
    let big_int = BigInt::from(20);
    let big_decimal =
        BigDecimal::from_str("21.5").expect("test BigDecimal should parse");

    let f32_sources = [
        DataConverter::from(1.0f32),
        DataConverter::from(2.0f64),
        DataConverter::from(true),
        DataConverter::from('A'),
        DataConverter::from(3i8),
        DataConverter::from(4i16),
        DataConverter::from(5i32),
        DataConverter::from(6i64),
        DataConverter::from(7i128),
        DataConverter::from(8isize),
        DataConverter::from(9u8),
        DataConverter::from(10u16),
        DataConverter::from(11u32),
        DataConverter::from(12u64),
        DataConverter::from(13u128),
        DataConverter::from(14usize),
        DataConverter::from("15.5"),
        DataConverter::from(&big_int),
        DataConverter::from(&big_decimal),
    ];
    for source in f32_sources {
        assert!(
            source
                .to::<f32>()
                .expect("source should convert to f32")
                .is_finite()
        );
    }

    let f64_sources = [
        DataConverter::from(1.0f64),
        DataConverter::from(2.0f32),
        DataConverter::from(false),
        DataConverter::from('B'),
        DataConverter::from(3i8),
        DataConverter::from(4i16),
        DataConverter::from(5i32),
        DataConverter::from(6i64),
        DataConverter::from(7i128),
        DataConverter::from(8isize),
        DataConverter::from(9u8),
        DataConverter::from(10u16),
        DataConverter::from(11u32),
        DataConverter::from(12u64),
        DataConverter::from(13u128),
        DataConverter::from(14usize),
        DataConverter::from("15.5"),
        DataConverter::from(&big_int),
        DataConverter::from(&big_decimal),
    ];
    for source in f64_sources {
        assert!(
            source
                .to::<f64>()
                .expect("source should convert to f64")
                .is_finite()
        );
    }

    assert!(
        DataConverter::from(f64::INFINITY)
            .to::<f32>()
            .expect("infinity should cast to f32")
            .is_infinite()
    );
    assert!(matches!(
        DataConverter::from("bad").to::<f32>(),
        Err(DataConversionError::InvalidValue { .. })
    ));
    assert!(matches!(
        DataConverter::from("bad").to::<f64>(),
        Err(DataConversionError::InvalidValue { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Float32).to::<f32>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Float64).to::<f64>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::from(Duration::new(1, 0)).to::<f32>(),
        Err(DataConversionError::Unsupported { .. })
    ));
    assert!(matches!(
        DataConverter::from(Duration::new(1, 0)).to::<f64>(),
        Err(DataConversionError::Unsupported { .. })
    ));

    let huge = create_huge_bigint();
    let huge_decimal = BigDecimal::from(huge.clone());
    assert!(DataConverter::from(&huge).to::<f32>().is_err());
    assert!(DataConverter::from(&huge).to::<f64>().is_err());
    assert!(DataConverter::from(&huge_decimal).to::<f32>().is_err());
    assert!(DataConverter::from(&huge_decimal).to::<f64>().is_err());
}

/// Test integer conversion range checks across signed and unsigned targets.
#[test]
fn test_data_converter_numeric_conversions_check_integer_bounds() {
    let signed: i16 = DataConverter::from(255u8)
        .to()
        .expect("u8 should convert to i16");
    assert_eq!(signed, 255);

    assert!(matches!(
        DataConverter::from(-1i8).to::<u16>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::OutOfRange,
            ..
        })
    ));

    assert!(matches!(
        DataConverter::from(u16::MAX as u32 + 1).to::<u16>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::OutOfRange,
            ..
        })
    ));

    assert!(matches!(
        DataConverter::from(i128::MAX).to::<i64>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::OutOfRange,
            ..
        })
    ));
}

/// Test floating point conversion edge cases for integer and f32 targets.
#[test]
fn test_data_converter_float_conversions_check_non_finite_and_overflow() {
    let lossy = DataConversionOptions::default()
        .with_numeric_policy(NumericConversionPolicy::Lossy);
    let truncated: i8 = DataConverter::from(-12.9f64)
        .to_with(&lossy)
        .expect("finite f64 should truncate when converting to i8");
    assert_eq!(truncated, -12);

    assert!(matches!(
        DataConverter::from(f64::NAN).to::<i64>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::NonFinite,
            ..
        })
    ));

    assert!(matches!(
        DataConverter::from(f64::INFINITY).to::<i64>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::NonFinite,
            ..
        })
    ));

    assert!(matches!(
        DataConverter::from(f64::MAX).to::<f32>(),
        Err(DataConversionError::InvalidValue { .. })
    ));

    assert!(matches!(
        DataConverter::from(u128::MAX).to::<f32>(),
        Err(DataConversionError::InvalidValue { .. })
    ));
}

/// Test BigInt and BigDecimal conversions and range failures.
#[test]
fn test_data_converter_big_number_conversions_check_range() {
    let big_int = BigInt::from(i64::MAX);
    let converted: i64 = DataConverter::from(&big_int)
        .to()
        .expect("in-range BigInt should convert to i64");
    assert_eq!(converted, i64::MAX);

    let huge_int =
        BigInt::parse_bytes(b"999999999999999999999999999999999999", 10)
            .expect("test BigInt literal should parse");
    assert!(matches!(
        DataConverter::from(&huge_int).to::<i64>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::OutOfRange,
            ..
        })
    ));

    let decimal =
        BigDecimal::from_str("123.75").expect("test BigDecimal should parse");
    let converted: f64 = DataConverter::from(&decimal)
        .to()
        .expect("BigDecimal should convert to f64");
    assert!((converted - 123.75).abs() < f64::EPSILON);
}

/// Test the focused numeric module applies explicit lossy conversion.
#[test]
fn test_data_converter_numeric_lossy_conversion() {
    let options = DataConversionOptions::default()
        .with_numeric_policy(NumericConversionPolicy::Lossy);
    assert_eq!(DataConverter::from("-2.9").to_with::<i32>(&options), Ok(-2));
}

/// Test extreme decimal exponents are classified without expanding powers.
#[test]
fn test_data_converter_big_decimal_extreme_exponents_are_bounded() {
    let huge = BigDecimal::from_str("1e1000000000")
        .expect("large positive exponent should parse compactly");
    assert!(matches!(
        DataConverter::from(&huge).to::<i32>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::OutOfRange,
            ..
        }),
    ));

    let tiny = BigDecimal::from_str("1e-1000000000")
        .expect("large negative exponent should parse compactly");
    assert!(matches!(
        DataConverter::from(&tiny).to::<i32>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::PrecisionLoss,
            ..
        }),
    ));
    let lossy = DataConversionOptions::default()
        .with_numeric_policy(NumericConversionPolicy::Lossy);
    assert_eq!(DataConverter::from(&tiny).to_with::<i32>(&lossy), Ok(0));
}

/// Test textual and typed non-finite values share BigDecimal classification.
#[test]
fn test_data_converter_big_decimal_non_finite_classification_is_consistent() {
    for result in [
        DataConverter::from(f32::NAN).to::<BigDecimal>(),
        DataConverter::from(f32::NEG_INFINITY).to::<BigDecimal>(),
        DataConverter::from(f64::INFINITY).to::<BigDecimal>(),
        DataConverter::from("inf").to::<BigDecimal>(),
        DataConverter::from("NaN").to::<BigDecimal>(),
    ] {
        assert!(matches!(
            result,
            Err(DataConversionError::InvalidValue {
                to: qubit_datatype::DataType::BigDecimal,
                reason: InvalidValueReason::NonFinite,
                ..
            }),
        ));
    }
}

/// Test numeric parsing and narrowing boundary branches.
#[test]
fn test_data_converter_numeric_boundary_branches() {
    assert!(DataConverter::from(i16::MAX).to::<i8>().is_err());
    assert!(DataConverter::from(i32::MAX).to::<i16>().is_err());
    assert!(DataConverter::from(i64::MAX).to::<i32>().is_err());
    assert!(DataConverter::from(i128::MAX).to::<i64>().is_err());
    assert!(DataConverter::from(i128::MAX).to::<isize>().is_err());
    assert!(DataConverter::from(u16::MAX).to::<u8>().is_err());
    assert!(DataConverter::from(u32::MAX).to::<u16>().is_err());
    assert!(DataConverter::from(u64::MAX).to::<u32>().is_err());
    assert!(DataConverter::from(u128::MAX).to::<u64>().is_err());
    assert!(DataConverter::from(u128::MAX).to::<usize>().is_err());

    assert!(DataConverter::from("nan").to::<f32>().unwrap().is_nan());
    assert_eq!(DataConverter::from("inf").to::<f32>(), Ok(f32::INFINITY));
    assert_eq!(
        DataConverter::from("-infinity").to::<f32>(),
        Ok(f32::NEG_INFINITY),
    );
    assert!(DataConverter::from("NaN").to::<f64>().unwrap().is_nan());
    assert_eq!(
        DataConverter::from("+infinity").to::<f64>(),
        Ok(f64::INFINITY)
    );
    assert_eq!(
        DataConverter::from("-inf").to::<f64>(),
        Ok(f64::NEG_INFINITY),
    );
    assert!(matches!(
        DataConverter::from("NaN").to::<i32>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::NonFinite,
            ..
        }),
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Float32).to::<f32>(),
        Err(DataConversionError::Missing { .. }),
    ));
    assert!(matches!(
        DataConverter::from(Duration::from_secs(1)).to::<f64>(),
        Err(DataConversionError::Unsupported { .. }),
    ));
    assert_eq!(DataConverter::from("15").to::<f64>(), Ok(15.0));
    assert!(matches!(
        DataConverter::from(0.1f64).to::<f32>(),
        Err(DataConversionError::InvalidValue {
            reason: InvalidValueReason::PrecisionLoss,
            ..
        }),
    ));

    let zero = BigDecimal::from(0);
    assert_eq!(DataConverter::from(&zero).to::<i32>(), Ok(0));
    let integral_decimal = BigDecimal::from_str("12.0").unwrap();
    assert_eq!(DataConverter::from(&integral_decimal).to::<i32>(), Ok(12));
    let fractional = BigDecimal::from_str("12.9").unwrap();
    let lossy = DataConversionOptions::default()
        .with_numeric_policy(NumericConversionPolicy::Lossy);
    assert_eq!(
        DataConverter::from(&fractional).to_with::<i32>(&lossy),
        Ok(12)
    );

    assert_eq!(
        DataConverter::from(1.25f32).to::<BigDecimal>(),
        Ok(BigDecimal::from_str("1.25").unwrap()),
    );
    assert_eq!(
        DataConverter::from(1.25f64).to::<BigDecimal>(),
        Ok(BigDecimal::from_str("1.25").unwrap()),
    );
    assert_eq!(
        DataConverter::from("12").to::<BigDecimal>(),
        Ok(BigDecimal::from(12))
    );

    let date = NaiveDate::from_ymd_opt(2026, 7, 12).unwrap();
    let time = NaiveTime::from_hms_opt(1, 2, 3).unwrap();
    let datetime = NaiveDateTime::new(date, time);
    let instant = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
    let url = Url::parse("https://example.com").unwrap();
    let map = HashMap::from([("k".to_string(), "v".to_string())]);
    let json = serde_json::json!({"k": "v"});
    assert!(DataConverter::from(date).to::<BigDecimal>().is_err());
    assert!(DataConverter::from(time).to::<BigDecimal>().is_err());
    assert!(DataConverter::from(datetime).to::<BigDecimal>().is_err());
    assert!(DataConverter::from(instant).to::<BigDecimal>().is_err());
    assert!(
        DataConverter::from(Duration::from_secs(1))
            .to::<BigDecimal>()
            .is_err()
    );
    assert!(DataConverter::from(&url).to::<BigDecimal>().is_err());
    assert!(DataConverter::from(&map).to::<BigDecimal>().is_err());
    assert!(DataConverter::from(&json).to::<BigDecimal>().is_err());
}
