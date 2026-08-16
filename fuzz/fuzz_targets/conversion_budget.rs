// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Fuzzes bounded borrowed and owned conversion outcomes.
//!
//! The target exercises numeric, Unicode, and canonical JSON-string-map
//! sources. Borrowed and owned conversions must agree, and output-limit
//! failures must retain their structured budget facts.

#![no_main]

use std::collections::HashMap;

use libfuzzer_sys::fuzz_target;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConverter;

/// Bounds the fuzzer-controlled source and the derived output budget.
const MAX_INPUT_SIZE: usize = 4 * 1024;

fuzz_target!(|data: &[u8]| {
    if data.len() > MAX_INPUT_SIZE {
        return;
    }

    let output_limit = data.first().map_or(0, |value| usize::from(*value));
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_output_bytes(
                    u64::try_from(output_limit)
                        .expect("fuzz output limit byte fits u64"),
                )
                .build(),
        )
        .build();

    let number = i128::from_le_bytes(padded_bytes::<16>(data));
    assert_borrowed_and_owned_number(number, &policy, &limits);

    let text = String::from_utf8_lossy(data);
    assert_borrowed_and_owned_text(text.as_ref(), &policy, &limits);

    {
        let map = HashMap::from([
            ("a".to_owned(), text.as_ref().to_owned()),
            ("b".to_owned(), number.to_string()),
        ]);
        assert_borrowed_and_owned_map(&map, &policy, &limits);
    }
});

/// Pads arbitrary bytes into a fixed-width little-endian integer.
fn padded_bytes<const N: usize>(data: &[u8]) -> [u8; N] {
    let mut result = [0_u8; N];
    let length = data.len().min(N);
    result[..length].copy_from_slice(&data[..length]);
    result
}

/// Compares borrowed and owned integer conversion accounting.
fn assert_borrowed_and_owned_number(
    number: i128,
    policy: &ConversionPolicy,
    limits: &ConversionLimits,
) {
    let borrowed_result =
        DataConverter::from(number).to_with::<String>(policy, limits);
    let owned_result =
        DataConverter::from(number).into_target_with::<String>(policy, limits);
    assert_equivalent_results(&borrowed_result, &owned_result);
    assert_output_failure(&borrowed_result);
    assert_output_failure(&owned_result);
}

/// Compares borrowed and owned Unicode text conversion accounting.
fn assert_borrowed_and_owned_text(
    text: &str,
    policy: &ConversionPolicy,
    limits: &ConversionLimits,
) {
    let borrowed_result =
        DataConverter::from(text).to_with::<String>(policy, limits);
    let owned_result = DataConverter::from(text.to_owned())
        .into_target_with::<String>(policy, limits);
    assert_equivalent_results(&borrowed_result, &owned_result);
    assert_output_failure(&borrowed_result);
    assert_output_failure(&owned_result);
}

/// Compares borrowed and owned canonical JSON map conversion accounting.
fn assert_borrowed_and_owned_map(
    map: &HashMap<String, String>,
    policy: &ConversionPolicy,
    limits: &ConversionLimits,
) {
    let borrowed_result =
        DataConverter::from(map).to_with::<String>(policy, limits);
    let owned_result = DataConverter::from(map.clone())
        .into_target_with::<String>(policy, limits);
    assert_equivalent_results(&borrowed_result, &owned_result);
    assert_output_failure(&borrowed_result);
    assert_output_failure(&owned_result);
}

/// Checks that both conversion modes have the same public outcome class.
fn assert_equivalent_results(
    borrowed: &Result<String, qubit_datatype::DataConversionError>,
    owned: &Result<String, qubit_datatype::DataConversionError>,
) {
    match (borrowed, owned) {
        (Ok(left), Ok(right)) => assert_eq!(left, right),
        (Err(left), Err(right)) => assert_eq!(left.kind(), right.kind()),
        _ => panic!("borrowed and owned conversion outcomes diverged"),
    }
}

/// Verifies output-limit failures retain their complete budget facts.
fn assert_output_failure(
    result: &Result<String, qubit_datatype::DataConversionError>,
) {
    if let Err(error) = result
        && error.kind() == DataConversionErrorKind::LimitExceeded
    {
        assert!(error.budget_error().is_some_and(|budget| {
            *budget.resource() == ConversionResource::OutputBytes
        }));
    }
}
