// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Boolean conversion tests.

use qubit_datatype::converter::DataConversionErrorKind;

#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
use proptest::proptest;
use qubit_datatype::{
    ConversionLimit,
    DataConversionError,
    DataConversionOptions,
    DataConverter,
    DataType,
    InvalidValueReason,
    NumericConversionLimits,
    NumericConversionOptions,
};

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
    let options = DataConversionOptions::strict().with_numeric_options(
        NumericConversionOptions::strict().with_limits(
            NumericConversionLimits::default().with_max_text_bytes(1),
        ),
    );

    assert_eq!(
        DataConverter::from("true").to_with::<bool>(&options),
        Ok(true),
    );
    let error = DataConverter::from("10")
        .to_with::<bool>(&options)
        .expect_err("numeric Boolean text should honor the byte limit");
    assert_eq!(error.kind(), DataConversionErrorKind::LimitExceeded);
    assert_eq!(
        error.limit(),
        Some(&ConversionLimit::NumericTextBytes { maximum: 1 }),
    );
}

proptest! {
    /// Test that arbitrary UTF-8 strings never panic in Boolean parsing.
    #[test]
    fn test_data_converter_boolean_arbitrary_string_never_panics(value in ".*") {
        let _ = DataConverter::from(value).to::<bool>();
    }
}
