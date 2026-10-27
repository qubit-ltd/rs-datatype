// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Boolean conversion tests.

#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
use proptest::proptest;
use qubit_datatype::BooleanConversionPolicy;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::InvalidValueReason;
use qubit_datatype::NumericConversionLimits;
use qubit_datatype::converter::DataConversionErrorKind;

/// Test bool target conversions for all supported source variants.
#[test]
fn test_data_converter_bool_target_accepts_supported_sources() {
    assert!(
        DataConverter::from(true)
            .to::<bool>()
            .expect("bool should convert to bool")
    );
    assert!(
        DataConverter::from(1i8)
            .to::<bool>()
            .expect("i8 should convert to bool")
    );
    assert!(
        !DataConverter::from(0i16)
            .to::<bool>()
            .expect("i16 should convert to bool")
    );
    assert!(
        DataConverter::from(1i32)
            .to::<bool>()
            .expect("i32 should convert to bool")
    );
    assert!(
        !DataConverter::from(0i64)
            .to::<bool>()
            .expect("i64 should convert to bool")
    );
    assert!(
        DataConverter::from(1i128)
            .to::<bool>()
            .expect("i128 should convert to bool")
    );
    assert!(
        DataConverter::from(1u8)
            .to::<bool>()
            .expect("u8 should convert to bool")
    );
    assert!(
        !DataConverter::from(0u16)
            .to::<bool>()
            .expect("u16 should convert to bool")
    );
    assert!(
        DataConverter::from(1u32)
            .to::<bool>()
            .expect("u32 should convert to bool")
    );
    assert!(
        !DataConverter::from(0u64)
            .to::<bool>()
            .expect("u64 should convert to bool")
    );
    assert!(
        DataConverter::from(1u128)
            .to::<bool>()
            .expect("u128 should convert to bool")
    );
    assert!(
        !DataConverter::from("false")
            .to::<bool>()
            .expect("false string should convert to bool")
    );

    assert!(matches!(
        DataConverter::from("maybe").to::<bool>(),
        Err(ref error) if error == &DataConversionError::invalid(
            DataType::String,
            DataType::Bool,
            InvalidValueReason::InvalidBoolean,
        )
    ));
    assert!(matches!(
        DataConverter::Unset(DataType::Bool).to::<bool>(),
        Err(ref error) if error == &DataConversionError::missing(DataType::Bool, DataType::Bool)
    ));
    #[cfg(feature = "big-integer")]
    {
        let one = BigInt::from(1u8);
        assert_eq!(DataConverter::from(&one).to::<bool>(), Ok(true));
    }
    assert!(matches!(
        DataConverter::from('x').to::<bool>(),
        Err(conversion_error) if conversion_error.kind() == DataConversionErrorKind::Unsupported
    ));
}

/// Test numeric Boolean fallback honors numeric text limits without limiting
/// configured Boolean literals.
#[test]
fn test_data_converter_bool_numeric_text_limit() {
    let options = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .numeric_limits(NumericConversionLimits::builder().max_text_bytes(1).build())
        .build();

    assert_eq!(DataConverter::from("true").to_with::<bool>(&options, &limits), Ok(true),);
    let error = DataConverter::from("10")
        .to_with::<bool>(&options, &limits)
        .expect_err("numeric Boolean text should honor the byte limit");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::NumericTextBytes));
    assert_eq!(error.configured_limit(), Some(1));
    assert_eq!(error.observed_lower_bound(), Some(2));
}

#[test]
fn test_data_converter_bool_limits_unknown_text_before_syntax() {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .numeric_limits(NumericConversionLimits::builder().max_text_bytes(4).build())
        .build();
    let input = format!("{}x", "0".repeat(16_384));
    let error = DataConverter::from(input.as_str())
        .to_with::<bool>(&policy, &limits)
        .expect_err("unknown oversized Boolean text must be rejected by the byte limit");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(error.resource(), Some(ConversionResource::NumericTextBytes));
    assert_eq!(error.configured_limit(), Some(4));
    assert_eq!(error.observed_lower_bound(), Some(input.len() as u64));
    assert_eq!(error.from_type(), Some(DataType::String));
    assert_eq!(error.to_type(), DataType::Bool);
}

#[test]
fn test_data_converter_bool_keeps_literal_and_short_invalid_precedence() {
    let boolean = BooleanConversionPolicy::builder()
        .true_literal("00000x")
        .build()
        .expect("custom literal is disjoint from false literals");
    let policy = ConversionPolicy::builder().boolean_policy(boolean).build();
    let limits = ConversionLimits::builder()
        .numeric_limits(NumericConversionLimits::builder().max_text_bytes(4).build())
        .build();
    assert_eq!(
        DataConverter::from("00000x").to_with::<bool>(&policy, &limits),
        Ok(true)
    );
    assert_eq!(DataConverter::from("true").to_with::<bool>(&policy, &limits), Ok(true));
    let error = DataConverter::from("nope")
        .to_with::<bool>(&policy, &limits)
        .expect_err("short unknown text remains an invalid Boolean");
    assert_eq!(error.reason(), Some(&InvalidValueReason::InvalidBoolean));
}

proptest! {
    /// Test that arbitrary UTF-8 strings never panic in Boolean parsing.
    #[test]
    fn test_data_converter_boolean_arbitrary_string_never_panics(value in ".*") {
        let _ = DataConverter::from(value).to::<bool>();
    }
}
