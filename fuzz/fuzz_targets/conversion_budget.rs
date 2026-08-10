// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.
// =============================================================================
//! Fuzzes transactional output accounting for borrowed and owned conversions.
//!
//! The target exercises the public session APIs with numeric, Unicode, and
//! canonical JSON-string-map sources. The invariants are that successful
//! conversions charge exactly their UTF-8 payload and failed output renders do
//! not charge a partial prefix.

#![no_main]

use std::collections::HashMap;

use libfuzzer_sys::fuzz_target;
use qubit_datatype::ConversionBudgetLimits;
use qubit_datatype::ConversionResource;
use qubit_datatype::DataConversionErrorKind;
use qubit_datatype::DataConversionOptions;
use qubit_datatype::DataConverter;

/// Bounds the fuzzer-controlled source and the derived output budget.
const MAX_INPUT_SIZE: usize = 4 * 1024;

fuzz_target!(|data: &[u8]| {
    if data.len() > MAX_INPUT_SIZE {
        return;
    }

    let output_limit = data
        .first()
        .map_or(0, |value| usize::from(*value));
    let options = DataConversionOptions::strict().with_budget_limits(
        ConversionBudgetLimits::default().with_max_output_bytes(output_limit),
    );

    let number = i128::from_le_bytes(padded_bytes::<16>(data));
    assert_borrowed_and_owned_number(number, &options);

    let text = String::from_utf8_lossy(data);
    assert_borrowed_and_owned_text(text.as_ref(), &options);

    {
        let map = HashMap::from([
            ("a".to_owned(), text.as_ref().to_owned()),
            ("b".to_owned(), number.to_string()),
        ]);
        assert_borrowed_and_owned_map(&map, &options);
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
    options: &DataConversionOptions,
) {
    let mut borrowed = options.session();
    let mut owned = options.session();
    let borrowed_result = DataConverter::from(number)
        .to_in::<String>(&mut borrowed);
    let owned_result = DataConverter::from(number)
        .into_target_in::<String>(&mut owned);
    assert_equivalent_results(&borrowed_result, &owned_result);
    assert_output_accounting(&borrowed_result, &borrowed);
    assert_output_accounting(&owned_result, &owned);
}

/// Compares borrowed and owned Unicode text conversion accounting.
fn assert_borrowed_and_owned_text(
    text: &str,
    options: &DataConversionOptions,
) {
    let mut borrowed = options.session();
    let mut owned = options.session();
    let borrowed_result = DataConverter::from(text)
        .to_in::<String>(&mut borrowed);
    let owned_result = DataConverter::from(text.to_owned())
        .into_target_in::<String>(&mut owned);
    assert_equivalent_results(&borrowed_result, &owned_result);
    assert_output_accounting(&borrowed_result, &borrowed);
    assert_output_accounting(&owned_result, &owned);
}

/// Compares borrowed and owned canonical JSON map conversion accounting.
fn assert_borrowed_and_owned_map(
    map: &HashMap<String, String>,
    options: &DataConversionOptions,
) {
    let mut borrowed = options.session();
    let mut owned = options.session();
    let borrowed_result = DataConverter::from(map).to_in::<String>(&mut borrowed);
    let owned_result = DataConverter::from(map.clone())
        .into_target_in::<String>(&mut owned);
    assert_equivalent_results(&borrowed_result, &owned_result);
    assert_output_accounting(&borrowed_result, &borrowed);
    assert_output_accounting(&owned_result, &owned);
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

/// Verifies exact success charging and zero partial charging on failure.
fn assert_output_accounting(
    result: &Result<String, qubit_datatype::DataConversionError>,
    session: &qubit_datatype::ConversionSession<'_>,
) {
    match result {
        Ok(value) => assert_eq!(session.used_output_bytes(), value.len()),
        Err(error) => {
            assert!(
                session.used_output_bytes() == 0
                    || error.kind() != DataConversionErrorKind::LimitExceeded
                    || error.budget_error().is_some_and(|budget| {
                        *budget.resource() == ConversionResource::OutputBytes
                    })
            );
        }
    }
}
