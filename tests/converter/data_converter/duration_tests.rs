// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Duration conversion tests.

use std::time::Duration;

use chrono::NaiveDate;
use num_bigint::BigInt;
use qubit_datatype::{
    DataConversionError,
    DataConversionErrorKind,
    DataConversionOptions,
    DataConverter,
    DataType,
    DurationConversionOptions,
    DurationUnit,
    NumericConversionPolicy,
};

/// Test Duration string formatting and parsing.
#[test]
fn test_data_converter_duration_string_conversion() {
    let duration = Duration::new(1, 500);
    let lossy = DataConversionOptions::default()
        .with_numeric_policy(NumericConversionPolicy::Lossy);
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

    let micros: Duration = DataConverter::from("10us")
        .to()
        .expect("duration string with microseconds should parse");
    assert_eq!(micros, Duration::from_micros(10));

    let bare_default: Duration = DataConverter::from("10")
        .to()
        .expect("bare duration string should use default milliseconds");
    assert_eq!(bare_default, Duration::from_millis(10));

    let options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default().with_unit(DurationUnit::Seconds),
    );
    let bare_seconds: Duration = DataConverter::from("10")
        .to_with(&options)
        .expect("bare duration string should use configured seconds");
    assert_eq!(bare_seconds, Duration::from_secs(10));

    let no_suffix = DataConversionOptions::default()
        .with_numeric_policy(NumericConversionPolicy::Lossy)
        .with_duration_options(
            DurationConversionOptions::default()
                .with_unit(DurationUnit::Seconds)
                .with_append_unit_suffix(false),
        );
    let text: String = DataConverter::from(Duration::from_millis(1500))
        .to_with(&no_suffix)
        .expect("Duration should convert to suffixless rounded seconds");
    assert_eq!(text, "2");

    assert!(matches!(
        DataConverter::from("badns").to::<Duration>(),
        Err(DataConversionError::Invalid { .. })
    ));
    assert!(matches!(
        DataConverter::from("").to::<Duration>(),
        Err(DataConversionError::Invalid { .. })
    ));
    assert!(matches!(
        DataConverter::from("10fortnights").to::<Duration>(),
        Err(DataConversionError::Invalid {
            kind: DataConversionErrorKind::UnsupportedDurationUnit,
            ..
        })
    ));
    let overflowing_duration =
        format!("{}ns", (u64::MAX as u128 + 1) * 1_000_000_000);
    assert!(matches!(
        DataConverter::from(overflowing_duration).to::<Duration>(),
        Err(DataConversionError::Invalid { .. })
    ));
    let overflowing_days = format!("{}d", u64::MAX / (24 * 60 * 60) + 1);
    assert!(matches!(
        DataConverter::from(overflowing_days).to::<Duration>(),
        Err(DataConversionError::Invalid { .. })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Duration).to::<Duration>(),
        Err(DataConversionError::Missing { .. })
    ));
    assert!(matches!(
        DataConverter::from(1i32).to::<Duration>(),
        Ok(duration) if duration == Duration::from_millis(1)
    ));
}

/// Test Duration conversions with integer sources and targets.
#[test]
fn test_data_converter_duration_integer_conversion_uses_configured_unit() {
    let duration: Duration = DataConverter::from(1500u64)
        .to()
        .expect("integer duration should use default milliseconds");
    assert_eq!(duration, Duration::from_millis(1500));

    let options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default().with_unit(DurationUnit::Seconds),
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
        DataConverter::from(1isize),
        DataConverter::from(1u8),
        DataConverter::from(1u16),
        DataConverter::from(1u32),
        DataConverter::from(1u128),
        DataConverter::from(1usize),
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
        Err(DataConversionError::Invalid {
            kind: DataConversionErrorKind::NegativeDuration,
            ..
        })
    ));
    assert!(
        DataConverter::from(u128::from(u64::MAX) + 1)
            .to::<Duration>()
            .is_ok()
    );

    let overflowing_options = DataConversionOptions::default()
        .with_duration_options(
            DurationConversionOptions::default().with_unit(DurationUnit::Days),
        );
    assert!(matches!(
        DataConverter::from(u64::MAX).to_with::<Duration>(&overflowing_options),
        Err(DataConversionError::Invalid { .. })
    ));
    assert!(matches!(
        DataConverter::from(i128::from(u64::MAX))
            .to_with::<Duration>(&overflowing_options),
        Err(DataConversionError::Invalid { .. })
    ));

    let negative_big_integer = BigInt::from(-1);
    assert!(matches!(
        DataConverter::from(&negative_big_integer).to::<Duration>(),
        Err(DataConversionError::Invalid {
            kind: DataConversionErrorKind::NegativeDuration,
            ..
        })
    ));
    let overflowing_big_integer = BigInt::from(u64::MAX);
    assert!(matches!(
        DataConverter::from(&overflowing_big_integer)
            .to_with::<Duration>(&overflowing_options),
        Err(DataConversionError::Invalid { .. })
    ));
    assert!(matches!(
        DataConverter::from(
            NaiveDate::from_ymd_opt(2026, 5, 1).expect("test date")
        )
        .to::<Duration>(),
        Err(DataConversionError::Unsupported { .. })
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
        DurationConversionOptions::default().with_unit(DurationUnit::Seconds),
    );
    let duration = Duration::from_millis(1_500);
    assert!(matches!(
        DataConverter::from(duration).to_with::<String>(&exact),
        Err(DataConversionError::Invalid {
            kind: DataConversionErrorKind::PrecisionLoss,
            ..
        }),
    ));
    assert!(matches!(
        DataConverter::from(duration).to_with::<u64>(&exact),
        Err(DataConversionError::Invalid {
            kind: DataConversionErrorKind::PrecisionLoss,
            ..
        }),
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
            DurationConversionOptions::default().with_unit(unit),
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
        DurationConversionOptions::default().with_unit(DurationUnit::Seconds),
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
            Err(DataConversionError::Invalid {
                kind: DataConversionErrorKind::OutOfRange,
                ..
            }),
        ));
    }
}
