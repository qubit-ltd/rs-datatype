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
//! failures must retain their structured budget facts. Each fresh session is
//! checked independently against the generated output budget. On success,
//! committed output bytes equal the UTF-8 output length and fit the limit. On
//! output-limit failure, the resource and configured limit match, all budget
//! facts exist, and used <= limit, remaining == limit - used, and requested >
//! remaining. A failed transaction commits no output bytes to its fresh
//! session. Error `used` can include staged writes, and `requested` can be one
//! write or a conservative reservation, so neither is equated with committed
//! usage or the complete output length.

#![no_main]

use std::collections::HashMap;

use libfuzzer_sys::fuzz_target;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConversionError;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConverter;

/// Bounds the fuzzer-controlled source and the derived output budget.
const MAX_INPUT_SIZE: usize = 4 * 1024;

fuzz_target!(|data: &[u8]| {
    if data.len() > MAX_INPUT_SIZE {
        return;
    }

    let output_limit = data.first().map_or(0, |value| u64::from(*value));
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .operation_limits(
            ConversionOperationLimits::builder()
                .max_output_bytes(output_limit)
                .build(),
        )
        .build();

    let number = i128::from_le_bytes(padded_bytes::<16>(data));
    assert_borrowed_and_owned_number(number, &policy, &limits, output_limit);

    let text = String::from_utf8_lossy(data);
    assert_borrowed_and_owned_text(text.as_ref(), &policy, &limits, output_limit);

    {
        let map = HashMap::from([
            ("a".to_owned(), text.as_ref().to_owned()),
            ("b".to_owned(), number.to_string()),
        ]);
        assert_borrowed_and_owned_map(&map, &policy, &limits, output_limit);
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
fn assert_borrowed_and_owned_number(number: i128, policy: &ConversionPolicy, limits: &ConversionLimits, limit: u64) {
    let mut borrowed_session = ConversionSession::new(policy, limits);
    let borrowed_result = DataConverter::from(number).to_in::<String>(&mut borrowed_session);
    assert_budget_result(&borrowed_result, &borrowed_session, limit);
    let mut owned_session = ConversionSession::new(policy, limits);
    let owned_result = DataConverter::from(number).into_target_in::<String>(&mut owned_session);
    assert_budget_result(&owned_result, &owned_session, limit);
    assert_equivalent_results(&borrowed_result, &owned_result);
}

/// Compares borrowed and owned Unicode text conversion accounting.
fn assert_borrowed_and_owned_text(text: &str, policy: &ConversionPolicy, limits: &ConversionLimits, limit: u64) {
    let mut borrowed_session = ConversionSession::new(policy, limits);
    let borrowed_result = DataConverter::from(text).to_in::<String>(&mut borrowed_session);
    assert_budget_result(&borrowed_result, &borrowed_session, limit);
    let mut owned_session = ConversionSession::new(policy, limits);
    let owned_result = DataConverter::from(text.to_owned()).into_target_in::<String>(&mut owned_session);
    assert_budget_result(&owned_result, &owned_session, limit);
    assert_equivalent_results(&borrowed_result, &owned_result);
}

/// Compares borrowed and owned canonical JSON map conversion accounting.
fn assert_borrowed_and_owned_map(
    map: &HashMap<String, String>,
    policy: &ConversionPolicy,
    limits: &ConversionLimits,
    limit: u64,
) {
    let mut borrowed_session = ConversionSession::new(policy, limits);
    let borrowed_result = DataConverter::from(map).to_in::<String>(&mut borrowed_session);
    assert_budget_result(&borrowed_result, &borrowed_session, limit);
    let mut owned_session = ConversionSession::new(policy, limits);
    let owned_result = DataConverter::from(map.clone()).into_target_in::<String>(&mut owned_session);
    assert_budget_result(&owned_result, &owned_session, limit);
    assert_equivalent_results(&borrowed_result, &owned_result);
}

/// Checks that both conversion modes have the same public outcome class.
fn assert_equivalent_results(
    borrowed: &Result<String, DataConversionError>,
    owned: &Result<String, DataConversionError>,
) {
    match (borrowed, owned) {
        (Ok(left), Ok(right)) => assert_eq!(left, right),
        (Err(left), Err(right)) => {
            assert_eq!(left.kind(), right.kind());
            assert_eq!(left.from_type(), right.from_type());
            assert_eq!(left.to_type(), right.to_type());
        }
        _ => panic!("borrowed and owned conversion outcomes diverged"),
    }
}

/// Checks one conversion result against its independent output-budget session.
fn assert_budget_result(result: &Result<String, DataConversionError>, session: &ConversionSession<'_>, limit: u64) {
    match result {
        Ok(text) => {
            let length = u64::try_from(text.len()).expect("bounded fuzz output fits u64");
            assert!(length <= limit);
            assert_eq!(session.output_bytes_used(), length);
        }
        Err(error) => {
            assert_eq!(session.output_bytes_used(), 0);
            if error.kind() == DataConversionErrorKind::LimitExceeded {
                assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
                assert_eq!(error.configured_limit(), Some(limit));
                let used = error.used().expect("cumulative usage is present");
                let remaining = error.remaining().expect("remaining budget is present");
                let requested = error.requested().expect("rejected request is present");
                assert!(used <= limit);
                assert_eq!(remaining, limit - used);
                assert!(requested > remaining);
            }
        }
    }
}
