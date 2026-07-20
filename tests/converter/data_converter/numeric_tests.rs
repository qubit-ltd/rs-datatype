// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric conversion tests.

use qubit_datatype::converter::DataConversionErrorKind;

use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use num_bigint::BigInt;
use proptest::proptest;
use qubit_datatype::{
    ConversionLimit, DataConversionOptions, DataConverter, DataType, InvalidValueReason,
    NumericConversionLimits, NumericConversionOptions, StringConversionOptions,
};
use url::Url;

/// Creates a very large BigInt for overflow-oriented tests.
fn create_huge_bigint() -> BigInt {
    BigInt::parse_bytes(b"10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10)
        .expect("huge BigInt literal should parse")
}

/// Creates strict options with the supplied numeric resource limits.
fn options_with_limits(limits: NumericConversionLimits) -> DataConversionOptions {
    DataConversionOptions::strict()
        .with_numeric_options(NumericConversionOptions::strict().with_limits(limits))
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
        (DataConverter::from(7u8), 7),
        (DataConverter::from(8u16), 8),
        (DataConverter::from(9u32), 9),
        (DataConverter::from(10u64), 10),
        (DataConverter::from(11u128), 11),
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
    assert!(matches!(
        DataConverter::Unset(DataType::Int128).to::<i128>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::from("bad").to::<i128>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::InvalidSyntax { .. })
    )));
    assert!(matches!(
        DataConverter::from(f64::MAX).to::<i128>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    let huge = create_huge_bigint();
    let huge_decimal = BigDecimal::from(huge.clone());
    assert!(matches!(
        DataConverter::from(&huge).to::<i128>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::from(&huge_decimal).to::<i128>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::from(
            NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date")
        )
        .to::<i128>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
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
        (DataConverter::from(7u8), 7),
        (DataConverter::from(8u16), 8),
        (DataConverter::from(9u32), 9),
        (DataConverter::from(10u64), 10),
        (DataConverter::from(11u128), 11),
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
    assert!(matches!(
        DataConverter::Unset(DataType::UInt128).to::<u128>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::from("bad").to::<u128>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::from(1.5f64).to::<u128>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)
    )));
}

/// Test floating point target conversions across supported source variants.
#[test]
fn test_data_converter_float_targets_accept_supported_sources() {
    let big_int = BigInt::from(20);
    let big_decimal = BigDecimal::from_str("21.5").expect("test BigDecimal should parse");

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
        DataConverter::from(9u8),
        DataConverter::from(10u16),
        DataConverter::from(11u32),
        DataConverter::from(12u64),
        DataConverter::from(13u128),
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
        DataConverter::from(9u8),
        DataConverter::from(10u16),
        DataConverter::from(11u32),
        DataConverter::from(12u64),
        DataConverter::from(13u128),
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
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::from("bad").to::<f64>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::Unset(DataType::Float32).to::<f32>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::Unset(DataType::Float64).to::<f64>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::from(Duration::new(1, 0)).to::<f32>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
    ));
    assert!(matches!(
        DataConverter::from(Duration::new(1, 0)).to::<f64>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
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
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)
    )));

    assert!(matches!(
        DataConverter::from(u16::MAX as u32 + 1).to::<u16>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)
    )));

    assert!(matches!(
        DataConverter::from(i128::MAX).to::<i64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)
    )));
}

/// Test floating point conversion edge cases for integer and f32 targets.
#[test]
fn test_data_converter_float_conversions_check_non_finite_and_overflow() {
    let lossy = DataConversionOptions::lossy();
    let truncated: i8 = DataConverter::from(-12.9f64)
        .to_with(&lossy)
        .expect("finite f64 should truncate when converting to i8");
    assert_eq!(truncated, -12);

    assert!(matches!(
        DataConverter::from(f64::NAN).to::<i64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::NonFinite)
    )));

    assert!(matches!(
        DataConverter::from(f64::INFINITY).to::<i64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::NonFinite)
    )));

    assert!(matches!(
        DataConverter::from(f64::MAX).to::<f32>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));

    assert!(matches!(
        DataConverter::from(u128::MAX).to::<f32>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
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

    let huge_int = BigInt::parse_bytes(b"999999999999999999999999999999999999", 10)
        .expect("test BigInt literal should parse");
    assert!(matches!(
        DataConverter::from(&huge_int).to::<i64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)
    )));

    let decimal = BigDecimal::from_str("123.75").expect("test BigDecimal should parse");
    let converted: f64 = DataConverter::from(&decimal)
        .to()
        .expect("BigDecimal should convert to f64");
    assert!((converted - 123.75).abs() < f64::EPSILON);
}

/// Test the focused numeric module applies explicit lossy conversion.
#[test]
fn test_data_converter_numeric_lossy_conversion() {
    let options = DataConversionOptions::lossy();
    assert_eq!(DataConverter::from("-2.9").to_with::<i32>(&options), Ok(-2));
}

/// Test extreme decimal exponents are classified without expanding powers.
#[test]
fn test_data_converter_big_decimal_extreme_exponents_are_bounded() {
    let huge = BigDecimal::from_str("1e1000000000")
        .expect("large positive exponent should parse compactly");
    assert!(matches!(
        DataConverter::from(&huge).to::<i32>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)),
    ));

    let tiny = BigDecimal::from_str("1e-1000000000")
        .expect("large negative exponent should parse compactly");
    assert!(matches!(
        DataConverter::from(&tiny).to::<i32>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)),
    ));
    let lossy = DataConversionOptions::lossy();
    assert_eq!(DataConverter::from(&tiny).to_with::<i32>(&lossy), Ok(0));
}

/// Test exponent expansion cannot exhaust memory when producing a `BigInt`.
#[test]
fn test_data_converter_bigint_exponent_expansion_is_bounded() {
    let huge = BigDecimal::from_str("1e738508196")
        .expect("large positive exponent should parse compactly");
    for result in [
        DataConverter::from(&huge).to::<BigInt>(),
        DataConverter::from("1e738508196").to::<BigInt>(),
    ] {
        assert!(matches!(
            result,
            Err(conversion_error)
                if conversion_error.kind() == DataConversionErrorKind::LimitExceeded
                    && conversion_error.limit() == Some(
                        &ConversionLimit::BigIntegerDigits {
                            maximum: NumericConversionLimits::DEFAULT_MAX_BIG_INTEGER_DIGITS,
                        },
                    ),
        ));
    }
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
            Err(conversion_error)
                if conversion_error.kind() == DataConversionErrorKind::InvalidValue
                    && conversion_error.to_type() == qubit_datatype::DataType::BigDecimal
                    && matches!(
                        conversion_error.reason(),
                        Some(InvalidValueReason::NonFinite)
                    ),
        ));
    }
}

/// Test decimal text parsing across syntax, precision, and range boundaries.
#[test]
fn test_data_converter_core_numeric_parser_covers_decimal_boundaries() {
    let lossy = DataConversionOptions::lossy();

    assert_eq!(DataConverter::from("+1").to::<i128>(), Ok(1));
    assert_eq!(DataConverter::from("1e2").to::<i128>(), Ok(100));
    assert_eq!(DataConverter::from("1e-2").to_with::<i128>(&lossy), Ok(0));
    assert_eq!(DataConverter::from("-0.0").to::<i128>(), Ok(0));
    assert_eq!(
        DataConverter::from(i128::MIN.to_string()).to::<i128>(),
        Ok(i128::MIN)
    );
    assert_eq!(
        DataConverter::from(u128::MAX.to_string()).to::<u128>(),
        Ok(u128::MAX)
    );

    for value in ["", "+", "e1", ".", "1e", "1e+", "1eX", "1e1x", "1..0"] {
        assert!(matches!(
            DataConverter::from(value).to::<i128>(),
            Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::InvalidSyntax { .. })
        )));
    }
    for value in [
        "340282366920938463463374607431768211456",
        "1e999999999999999999999",
        "1e39",
        "35e37",
    ] {
        assert!(matches!(
            DataConverter::from(value).to::<u128>(),
            Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)
        )));
    }
    assert!(matches!(
        DataConverter::from("170141183460469231731687303715884105728")
            .to::<i128>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)
    )));
    assert!(matches!(
        DataConverter::from("-1").to::<u128>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)
    )));

    assert_eq!(DataConverter::from("0.0").to::<f64>(), Ok(0.0));
    assert_eq!(DataConverter::from("0.5").to::<f64>(), Ok(0.5));
    assert_eq!(DataConverter::from("1e1").to::<f64>(), Ok(10.0));
    assert_eq!(DataConverter::from("-1").to::<f64>(), Ok(-1.0));
    assert_eq!(
        DataConverter::from("0e999999999999999999999").to::<f64>(),
        Ok(0.0),
    );
    for value in [
        "0.1",
        "999999999999999999999999999999999999999",
        "1e-4294967296",
    ] {
        assert!(matches!(
            DataConverter::from(value).to::<f64>(),
            Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)
        )));
    }
    assert!(matches!(
        DataConverter::from("1e500").to::<f64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)
    )));
}

/// Test arbitrary-precision targets across every numeric source family.
#[test]
fn test_data_converter_big_integer_target_covers_numeric_sources() {
    let integral_decimal = BigDecimal::from_str("12.0").unwrap();
    let sources = [
        (DataConverter::from(true), BigInt::from(1)),
        (DataConverter::from('A'), BigInt::from(65)),
        (DataConverter::from(-1i8), BigInt::from(-1)),
        (DataConverter::from(2i16), BigInt::from(2)),
        (DataConverter::from(3i32), BigInt::from(3)),
        (DataConverter::from(4i64), BigInt::from(4)),
        (DataConverter::from(5i128), BigInt::from(5)),
        (DataConverter::from(7u8), BigInt::from(7)),
        (DataConverter::from(8u16), BigInt::from(8)),
        (DataConverter::from(9u32), BigInt::from(9)),
        (DataConverter::from(10u64), BigInt::from(10)),
        (DataConverter::from(11u128), BigInt::from(11)),
        (DataConverter::from(12.0f32), BigInt::from(12)),
        (DataConverter::from(13.0f64), BigInt::from(13)),
        (DataConverter::from(&integral_decimal), BigInt::from(12)),
        (DataConverter::from("-14"), BigInt::from(-14)),
        (
            DataConverter::from(Duration::from_secs(2)),
            BigInt::from(2_000),
        ),
    ];
    for (source, expected) in sources {
        assert_eq!(source.to::<BigInt>(), Ok(expected));
    }

    let lossy = DataConversionOptions::lossy();
    assert!(matches!(
        DataConverter::from(12.5f64).to::<BigInt>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)
    )));
    assert_eq!(
        DataConverter::from(12.5f64).to_with::<BigInt>(&lossy),
        Ok(BigInt::from(12))
    );
    assert!(matches!(
        DataConverter::from(f64::NAN).to::<BigInt>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::NonFinite)
    )));
    let fractional_decimal = BigDecimal::from_str("12.5").unwrap();
    assert!(matches!(
        DataConverter::from(&fractional_decimal).to::<BigInt>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)
    )));

    let imprecise_integer = BigInt::from(9_007_199_254_740_993u64);
    assert!(matches!(
        DataConverter::from(&imprecise_integer).to::<f64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)
    )));
    assert_eq!(
        DataConverter::from(&imprecise_integer).to_with::<f64>(&lossy),
        Ok(9_007_199_254_740_992.0)
    );
    let imprecise_decimal = BigDecimal::from_str("0.1").unwrap();
    assert!(matches!(
        DataConverter::from(&imprecise_decimal).to::<f64>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)
    )));
    assert!(
        DataConverter::from(&imprecise_decimal)
            .to_with::<f64>(&lossy)
            .expect("lossy decimal conversion should succeed")
            .is_finite()
    );

    assert_eq!(
        DataConverter::from(15u8).to::<BigDecimal>(),
        Ok(BigDecimal::from(15))
    );
    let map = HashMap::from([("key".to_owned(), "value".to_owned())]);
    assert!(matches!(
        DataConverter::from(&map).to::<BigInt>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
    ));
}

/// Test decimal text follows the fractional policy for BigInt targets.
#[test]
fn test_data_converter_big_integer_target_applies_policy_to_decimal_text() {
    assert_eq!(
        DataConverter::from("12.0").to::<BigInt>(),
        Ok(BigInt::from(12))
    );
    assert_eq!(
        DataConverter::from("1.2e2").to::<BigInt>(),
        Ok(BigInt::from(120))
    );
    assert!(matches!(
        DataConverter::from("12.5").to::<BigInt>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)
    )));

    let lossy = DataConversionOptions::lossy();
    assert_eq!(
        DataConverter::from("-12.5").to_with::<BigInt>(&lossy),
        Ok(BigInt::from(-12))
    );
}

proptest! {
    /// Test that arbitrary UTF-8 strings never panic in numeric parsing.
    #[test]
    fn test_data_converter_numeric_arbitrary_string_never_panics(value in ".*") {
        let converter = DataConverter::from(value);
        let _ = converter.to::<i128>();
        let _ = converter.to::<u128>();
        let _ = converter.to::<f64>();
    }
}

/// Test numeric parsing and narrowing boundary branches.
#[test]
fn test_data_converter_numeric_boundary_branches() {
    assert!(DataConverter::from(i16::MAX).to::<i8>().is_err());
    assert!(DataConverter::from(i32::MAX).to::<i16>().is_err());
    assert!(DataConverter::from(i64::MAX).to::<i32>().is_err());
    assert!(DataConverter::from(i128::MAX).to::<i64>().is_err());
    assert!(DataConverter::from(u16::MAX).to::<u8>().is_err());
    assert!(DataConverter::from(u32::MAX).to::<u16>().is_err());
    assert!(DataConverter::from(u64::MAX).to::<u32>().is_err());
    assert!(DataConverter::from(u128::MAX).to::<u64>().is_err());

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
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::NonFinite)),
    ));
    assert!(matches!(
        DataConverter::Unset(DataType::Float32).to::<f32>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing,
    ));
    assert!(matches!(
        DataConverter::from(Duration::from_secs(1)).to::<f64>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported,
    ));
    assert_eq!(DataConverter::from("15").to::<f64>(), Ok(15.0));
    assert!(matches!(
        DataConverter::from(0.1f64).to::<f32>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)),
    ));

    let zero = BigDecimal::from(0);
    assert_eq!(DataConverter::from(&zero).to::<i32>(), Ok(0));
    let integral_decimal = BigDecimal::from_str("12.0").unwrap();
    assert_eq!(DataConverter::from(&integral_decimal).to::<i32>(), Ok(12));
    let fractional = BigDecimal::from_str("12.9").unwrap();
    let lossy = DataConversionOptions::lossy();
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

/// Test numeric text byte limits after string normalization.
#[test]
fn test_numeric_text_byte_limit_boundaries() {
    let limits = NumericConversionLimits::default().with_max_text_bytes(3);
    let options = options_with_limits(limits)
        .with_string_options(StringConversionOptions::default().with_trim(true));

    assert_eq!(
        DataConverter::from(" 123 ").to_with::<u32>(&options),
        Ok(123),
    );
    let error = DataConverter::from("1234")
        .to_with::<u32>(&options)
        .expect_err("one byte over the configured limit must fail");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::UInt32);
    assert_eq!(
        error.limit(),
        Some(&ConversionLimit::NumericTextBytes { maximum: 3 }),
    );
    assert_eq!(error.reason(), None);
}

/// Test text-to-float rounding still enforces its text byte budget.
#[test]
fn test_numeric_text_limit_applies_before_float_parsing() {
    let options = DataConversionOptions::strict().with_numeric_options(
        NumericConversionOptions::env_friendly()
            .with_limits(NumericConversionLimits::default().with_max_text_bytes(3)),
    );

    assert_eq!(
        DataConverter::from("0.1").to_with::<f32>(&options),
        Ok(0.1_f32),
    );
    let error = DataConverter::from("0.10")
        .to_with::<f32>(&options)
        .expect_err("limit checking must precede target float parsing");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(
        error.limit(),
        Some(&ConversionLimit::NumericTextBytes { maximum: 3 }),
    );
}

/// Test BigInteger decimal digit limits for text materialization.
#[test]
fn test_big_integer_digit_limit_text_boundaries() {
    let at_limit =
        options_with_limits(NumericConversionLimits::default().with_max_big_integer_digits(4));
    assert_eq!(
        DataConverter::from("1e3").to_with::<BigInt>(&at_limit),
        Ok(BigInt::from(1_000)),
    );

    let over_limit =
        options_with_limits(NumericConversionLimits::default().with_max_big_integer_digits(3));
    let error = DataConverter::from("1e3")
        .to_with::<BigInt>(&over_limit)
        .expect_err("four result digits must exceed a three-digit limit");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(
        error.limit(),
        Some(&ConversionLimit::BigIntegerDigits { maximum: 3 }),
    );

    let zero_limit =
        options_with_limits(NumericConversionLimits::default().with_max_big_integer_digits(0));
    assert_eq!(
        DataConverter::from("0e999").to_with::<BigInt>(&zero_limit),
        Ok(BigInt::from(0)),
    );
    assert_eq!(
        DataConverter::from("0001").to_with::<BigInt>(&options_with_limits(
            NumericConversionLimits::default().with_max_big_integer_digits(1),
        )),
        Ok(BigInt::from(1)),
    );
    assert!(matches!(
        DataConverter::from("1").to_with::<BigInt>(&zero_limit),
        Err(error) if error.kind() == DataConversionErrorKind::LimitExceeded,
    ));
}

/// Test BigDecimal expansion uses the configurable BigInteger digit limit.
#[test]
fn test_big_integer_digit_limit_big_decimal_expansion() {
    let decimal = BigDecimal::new(BigInt::from(1), -3);
    let at_limit =
        options_with_limits(NumericConversionLimits::default().with_max_big_integer_digits(4));
    assert_eq!(
        DataConverter::from(&decimal).to_with::<BigInt>(&at_limit),
        Ok(BigInt::from(1_000)),
    );

    let over_limit =
        options_with_limits(NumericConversionLimits::default().with_max_big_integer_digits(3));
    let error = DataConverter::from(&decimal)
        .to_with::<BigInt>(&over_limit)
        .expect_err("BigDecimal expansion must honor the digit limit");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(
        error.limit(),
        Some(&ConversionLimit::BigIntegerDigits { maximum: 3 }),
    );
}

/// Verifies consuming big-number identity conversion preserves values and
/// continues to enforce the BigInteger digit limit.
#[test]
fn test_data_converter_consuming_big_number_identity_preserves_limits() {
    let integer = BigInt::from(12_345_u32);
    assert_eq!(
        DataConverter::from(integer.clone()).into_target::<BigInt>(),
        Ok(integer),
    );

    let decimal = BigDecimal::from_str("12345.6789").expect("test BigDecimal literal should parse");
    assert_eq!(
        DataConverter::from(decimal.clone()).into_target::<BigDecimal>(),
        Ok(decimal),
    );

    let options =
        options_with_limits(NumericConversionLimits::default().with_max_big_integer_digits(4));
    let error = DataConverter::from(BigInt::from(12_345_u32))
        .into_target_with::<BigInt>(&options)
        .expect_err("consuming BigInteger identity must honor the digit limit");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(
        error.limit(),
        Some(&ConversionLimit::BigIntegerDigits { maximum: 4 }),
    );
}
