// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Duration conversion tests.

use qubit_datatype::converter::DataConversionErrorKind;

use std::time::Duration;

use chrono::NaiveDate;
use num_bigint::BigInt;
use proptest::strategy::{
    Just,
    Strategy,
};
use proptest::{
    prop_assert_eq,
    prop_oneof,
    proptest,
};
use qubit_datatype::{
    DataConversionOptions,
    DataConverter,
    DataType,
    DurationConversionOptions,
    DurationUnit,
    InvalidValueReason,
    NumericConversionPolicy,
    SuffixlessDurationPolicy,
};

/// Test Duration string formatting and parsing.
#[test]
fn test_data_converter_duration_string_conversion() {
    let duration = Duration::new(1, 500);
    let lossy = DataConversionOptions::lossy();
    let text: String = DataConverter::from(duration)
        .to_with(&lossy)
        .expect("Duration should convert to string");
    assert_eq!(text, "1000ms");

    let direct: Duration = DataConverter::from(duration)
        .to()
        .expect("Duration should convert to Duration");
    assert_eq!(direct, duration);

    let parsed: Duration = DataConverter::from("1000000500ns")
        .to()
        .expect("duration string should parse");
    assert_eq!(parsed, duration);

    let millis: Duration = DataConverter::from("10ms")
        .to()
        .expect("duration string with milliseconds should parse");
    assert_eq!(millis, Duration::from_millis(10));

    let seconds: Duration = DataConverter::from("2s")
        .to()
        .expect("duration string with seconds should parse");
    assert_eq!(seconds, Duration::from_secs(2));

    let minutes: Duration = DataConverter::from("2m")
        .to()
        .expect("duration string with minutes should parse");
    assert_eq!(minutes, Duration::from_secs(120));

    let hours: Duration = DataConverter::from("2h")
        .to()
        .expect("duration string with hours should parse");
    assert_eq!(hours, Duration::from_secs(7200));

    let days: Duration = DataConverter::from("2d")
        .to()
        .expect("duration string with days should parse");
    assert_eq!(days, Duration::from_secs(172800));

    for source in ["10us", "10µs", "10μs"] {
        let micros: Duration = DataConverter::from(source)
            .to()
            .expect("every supported microsecond suffix should parse");
        assert_eq!(micros, Duration::from_micros(10));
    }

    let bare_default: Duration = DataConverter::from("10")
        .to()
        .expect("bare duration string should use default milliseconds");
    assert_eq!(bare_default, Duration::from_millis(10));

    let options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default().with_suffixless_string_policy(
            SuffixlessDurationPolicy::Assume(DurationUnit::Seconds),
        ),
    );
    let bare_seconds: Duration = DataConverter::from("10")
        .to_with(&options)
        .expect("bare duration string should use configured seconds");
    assert_eq!(bare_seconds, Duration::from_secs(10));

    let reject_bare = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default()
            .with_suffixless_string_policy(SuffixlessDurationPolicy::Reject),
    );
    assert!(matches!(
        DataConverter::from("10").to_with::<Duration>(&reject_bare),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::InvalidSyntax { .. })),
    ));

    let no_suffix = DataConversionOptions::lossy().with_duration_options(
        DurationConversionOptions::default()
            .with_output_unit(DurationUnit::Seconds)
            .with_append_unit_suffix(false),
    );
    let text: String = DataConverter::from(Duration::from_millis(1500))
        .to_with(&no_suffix)
        .expect("Duration should convert to suffixless rounded seconds");
    assert_eq!(text, "2");

    assert!(matches!(
        DataConverter::from("badns").to::<Duration>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::from("").to::<Duration>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::from("10fortnights").to::<Duration>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::UnsupportedDurationUnit)
    )));
    let overflowing_duration =
        format!("{}ns", (u64::MAX as u128 + 1) * 1_000_000_000);
    assert!(matches!(
        DataConverter::from(overflowing_duration).to::<Duration>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    let overflowing_days = format!("{}d", u64::MAX / (24 * 60 * 60) + 1);
    assert!(matches!(
        DataConverter::from(overflowing_days).to::<Duration>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Duration).to::<Duration>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Missing
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<Duration>(),
        Ok(duration) if duration == Duration::from_millis(1)
    ));
}

/// Test that numeric input, suffixless strings, and output use independent
/// Duration units.
#[test]
fn test_data_converter_duration_uses_independent_unit_policies() {
    let options = DataConversionOptions::strict().with_duration_options(
        DurationConversionOptions::default()
            .with_numeric_input_unit(DurationUnit::Seconds)
            .with_suffixless_string_policy(SuffixlessDurationPolicy::Assume(
                DurationUnit::Minutes,
            ))
            .with_output_unit(DurationUnit::Hours),
    );

    assert_eq!(
        DataConverter::from(2u64).to_with::<Duration>(&options),
        Ok(Duration::from_secs(2)),
    );
    assert_eq!(
        DataConverter::from("2").to_with::<Duration>(&options),
        Ok(Duration::from_secs(120)),
    );
    assert_eq!(
        DataConverter::from(Duration::from_secs(7_200))
            .to_with::<String>(&options),
        Ok("2h".to_string()),
    );
}

/// Test Duration conversions with integer sources and targets.
#[test]
fn test_data_converter_duration_integer_conversion_uses_configured_unit() {
    let duration: Duration = DataConverter::from(1500u64)
        .to()
        .expect("integer duration should use default milliseconds");
    assert_eq!(duration, Duration::from_millis(1500));

    let options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default()
            .with_numeric_input_unit(DurationUnit::Seconds)
            .with_output_unit(DurationUnit::Seconds),
    );
    let duration: Duration = DataConverter::from(2u64)
        .to_with(&options)
        .expect("integer duration should use configured seconds");
    assert_eq!(duration, Duration::from_secs(2));

    let integer_sources = [
        DataConverter::from(1i8),
        DataConverter::from(1i16),
        DataConverter::from(1i64),
        DataConverter::from(1i128),
        DataConverter::from(1u8),
        DataConverter::from(1u16),
        DataConverter::from(1u32),
        DataConverter::from(1u128),
    ];
    for source in integer_sources {
        let duration: Duration = source
            .to_with(&options)
            .expect("integer source should convert with configured seconds");
        assert_eq!(duration, Duration::from_secs(1));
    }

    let big_integer = BigInt::from(3u8);
    let duration: Duration = DataConverter::from(&big_integer)
        .to_with(&options)
        .expect("BigInteger should convert to Duration");
    assert_eq!(duration, Duration::from_secs(3));

    let lossy_options = options
        .clone()
        .with_numeric_policy(NumericConversionPolicy::Lossy);
    let units: u64 = DataConverter::from(Duration::from_millis(1499))
        .to_with(&lossy_options)
        .expect("Duration should round to configured integer unit");
    assert_eq!(units, 1);

    let units: u64 = DataConverter::from(Duration::from_millis(1500))
        .to_with(&lossy_options)
        .expect("Duration should round half up to configured integer unit");
    assert_eq!(units, 2);

    let signed_units: i64 = DataConverter::from(Duration::from_millis(1500))
        .to_with(&lossy_options)
        .expect("Duration should convert to signed integer units");
    assert_eq!(signed_units, 2);

    assert!(matches!(
        DataConverter::from(-1i32).to::<Duration>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::NegativeDuration)
    )));
    assert!(
        DataConverter::from(u128::from(u64::MAX) + 1)
            .to::<Duration>()
            .is_ok()
    );

    let overflowing_options = DataConversionOptions::default()
        .with_duration_options(
            DurationConversionOptions::default()
                .with_numeric_input_unit(DurationUnit::Days),
        );
    assert!(matches!(
        DataConverter::from(u64::MAX).to_with::<Duration>(&overflowing_options),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::from(i128::from(u64::MAX))
            .to_with::<Duration>(&overflowing_options),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));

    let negative_big_integer = BigInt::from(-1);
    assert!(matches!(
        DataConverter::from(&negative_big_integer).to::<Duration>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::NegativeDuration)
    )));
    let huge_negative_big_integer =
        -(BigInt::from(u128::MAX) + BigInt::from(1u8));
    assert!(matches!(
        DataConverter::from(&huge_negative_big_integer).to::<Duration>(),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::NegativeDuration)
    )));
    let overflowing_big_integer = BigInt::from(u64::MAX);
    assert!(matches!(
        DataConverter::from(&overflowing_big_integer)
            .to_with::<Duration>(&overflowing_options),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::InvalidValue
    ));
    assert!(matches!(
        DataConverter::from(
            NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date")
        )
        .to::<Duration>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
    ));
}

/// Test canonical duration text conversion.
#[test]
fn test_data_converter_duration_text() {
    assert_eq!(
        DataConverter::from("2s").to::<Duration>(),
        Ok(Duration::from_secs(2))
    );
}

/// Test duration formatting and integer conversion honor numeric exactness.
#[test]
fn test_data_converter_duration_targets_honor_numeric_policy() {
    let exact = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default()
            .with_output_unit(DurationUnit::Seconds),
    );
    let duration = Duration::from_millis(1_500);
    assert!(matches!(
        DataConverter::from(duration).to_with::<String>(&exact),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)),
    ));
    assert!(matches!(
        DataConverter::from(duration).to_with::<u64>(&exact),
        Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::PrecisionLoss)),
    ));

    let lossy = exact.with_numeric_policy(NumericConversionPolicy::Lossy);
    assert_eq!(
        DataConverter::from(duration).to_with::<String>(&lossy),
        Ok("2s".to_string()),
    );
    assert_eq!(DataConverter::from(duration).to_with::<u64>(&lossy), Ok(2));
}

/// Test large unit counts that still fit in `Duration` after decomposition.
#[test]
fn test_data_converter_duration_accepts_large_representable_unit_counts() {
    let count = u128::from(u64::MAX) + 1;
    for (unit, suffix, expected) in [
        (
            DurationUnit::Nanoseconds,
            "ns",
            Duration::new(u64::MAX / 1_000_000_000, 709_551_616),
        ),
        (
            DurationUnit::Microseconds,
            "us",
            Duration::new(u64::MAX / 1_000_000, 551_616_000),
        ),
        (
            DurationUnit::Milliseconds,
            "ms",
            Duration::new(u64::MAX / 1_000, 616_000_000),
        ),
    ] {
        let options = DataConversionOptions::default().with_duration_options(
            DurationConversionOptions::default().with_numeric_input_unit(unit),
        );
        assert_eq!(
            DataConverter::from(count).to_with::<Duration>(&options),
            Ok(expected),
        );
        assert_eq!(
            DataConverter::from(format!("{count}{suffix}")).to::<Duration>(),
            Ok(expected),
        );
    }
}

/// Test every duration overflow boundary is classified as out of range.
#[test]
fn test_data_converter_duration_rejects_unrepresentable_counts() {
    let seconds = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default()
            .with_numeric_input_unit(DurationUnit::Seconds)
            .with_suffixless_string_policy(SuffixlessDurationPolicy::Assume(
                DurationUnit::Seconds,
            )),
    );
    let above_u128 = BigInt::from(u128::MAX) + BigInt::from(1u8);
    for result in [
        DataConverter::from(&above_u128).to_with::<Duration>(&seconds),
        DataConverter::from(u128::MAX).to_with::<Duration>(&seconds),
        DataConverter::from(format!("{}s", u128::MAX)).to::<Duration>(),
        DataConverter::from(format!("{}0", u128::MAX)).to::<Duration>(),
    ] {
        assert!(matches!(
            result,
            Err(conversion_error) if matches!(conversion_error.reason(), Some(InvalidValueReason::OutOfRange)),
        ));
    }
}

/// Creates arbitrary supported Duration units for property tests.
fn duration_unit_strategy() -> impl Strategy<Value = DurationUnit> {
    prop_oneof![
        Just(DurationUnit::Nanoseconds),
        Just(DurationUnit::Microseconds),
        Just(DurationUnit::Milliseconds),
        Just(DurationUnit::Seconds),
        Just(DurationUnit::Minutes),
        Just(DurationUnit::Hours),
        Just(DurationUnit::Days),
    ]
}

proptest! {
    /// Test that exact integer Duration values survive formatting and parsing.
    #[test]
    fn test_data_converter_duration_exact_round_trip_property(
        unit in duration_unit_strategy(),
        value in 0u64..=1_000_000,
    ) {
        let options = DataConversionOptions::strict().with_duration_options(
            DurationConversionOptions::default()
                .with_numeric_input_unit(unit)
                .with_suffixless_string_policy(
                    SuffixlessDurationPolicy::Assume(unit),
                )
                .with_output_unit(unit),
        );
        let duration = DataConverter::from(value)
            .to_with::<Duration>(&options)
            .expect("bounded unit count should fit Duration");
        let text = DataConverter::from(duration)
            .to_with::<String>(&options)
            .expect("exactly represented Duration should format");
        let restored = DataConverter::from(text)
            .to_with::<Duration>(&options)
            .expect("formatted Duration should parse");

        prop_assert_eq!(restored, duration);
    }

    /// Test that arbitrary UTF-8 strings never panic in Duration parsing.
    #[test]
    fn test_data_converter_duration_arbitrary_string_never_panics(value in ".*") {
        let _ = DataConverter::from(value).to::<Duration>();
    }
}
