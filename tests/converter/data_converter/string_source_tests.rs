// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! String-source normalization tests.

use qubit_datatype::BlankStringPolicy;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConverter;
use qubit_datatype::DataType;
use qubit_datatype::InvalidValueReason;
use qubit_datatype::StringConversionPolicy;

/// Test that blank-string policy outcomes retain the requested target context.
#[test]
fn test_string_source_normalization_maps_policy_errors() {
    let missing_options = ConversionPolicy::default().with_string_policy(
        StringConversionPolicy::default()
            .with_blank_string_policy(BlankStringPolicy::TreatAsMissing),
    );
    assert_eq!(
        DataConverter::from(" ")
            .to_with::<u32>(&missing_options, ConversionLimits::default_ref()),
        Err(DataConversionError::missing(
            DataType::String,
            DataType::UInt32
        )),
    );

    let reject_options = ConversionPolicy::default().with_string_policy(
        StringConversionPolicy::default()
            .with_blank_string_policy(BlankStringPolicy::Reject),
    );
    assert_eq!(
        DataConverter::from(" ")
            .to_with::<bool>(&reject_options, ConversionLimits::default_ref()),
        Err(DataConversionError::invalid(
            DataType::String,
            DataType::Bool,
            InvalidValueReason::BlankRejected,
        )),
    );
}
