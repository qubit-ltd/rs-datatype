// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Boolean conversion tests.

use num_bigint::BigInt;
use qubit_datatype::{
    DataConversionError,
    DataConverter,
    DataType,
    InvalidValueReason,
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
        Err(DataConversionError::InvalidValue {
            from: DataType::String,
            to: DataType::Bool,
            reason: InvalidValueReason::InvalidBoolean,
        })
    ));
    assert!(matches!(
        DataConverter::Empty(DataType::Bool).to::<bool>(),
        Err(DataConversionError::Missing {
            from: DataType::Bool,
            to: DataType::Bool,
        })
    ));
    assert_eq!(DataConverter::from(1isize).to::<bool>(), Ok(true));
    assert_eq!(DataConverter::from(0usize).to::<bool>(), Ok(false));
    let one = BigInt::from(1u8);
    assert_eq!(DataConverter::from(&one).to::<bool>(), Ok(true));
    assert!(matches!(
        DataConverter::from('x').to::<bool>(),
        Err(DataConversionError::Unsupported { .. })
    ));
}
