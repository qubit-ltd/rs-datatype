// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#![no_main]

use bigdecimal::BigDecimal;
use libfuzzer_sys::fuzz_target;
use num_bigint::BigInt;
use qubit_datatype::{NumericComparisonPolicy, NumericValueRef, compare_numeric};

const FIXED_INPUT_SIZE: usize = 42;
const MAX_COEFFICIENT_SIZE: usize = 128;

fuzz_target!(|data: &[u8]| {
    if data.len() < FIXED_INPUT_SIZE || data.len() > FIXED_INPUT_SIZE + MAX_COEFFICIENT_SIZE {
        return;
    }
    let Some(signed_bytes) = read_array::<16>(data, 0) else {
        return;
    };
    let Some(unsigned_bytes) = read_array::<16>(data, 16) else {
        return;
    };
    let Some(float_bytes) = read_array::<8>(data, 32) else {
        return;
    };
    let Some(scale_bytes) = read_array::<2>(data, 40) else {
        return;
    };

    let signed = i128::from_le_bytes(signed_bytes);
    let unsigned = u128::from_le_bytes(unsigned_bytes);
    let float = f64::from_bits(u64::from_le_bytes(float_bytes));
    if float.is_nan() {
        return;
    }
    let coefficient = BigInt::from_signed_bytes_le(&data[FIXED_INPUT_SIZE..]);
    let scale = i64::from(u16::from_le_bytes(scale_bytes) % 257) - 128;
    let decimal = BigDecimal::new(coefficient.clone(), scale);
    let values = [
        NumericValueRef::Int128(signed),
        NumericValueRef::UInt128(unsigned),
        NumericValueRef::Float64(float),
        NumericValueRef::BigInteger(&coefficient),
        NumericValueRef::BigDecimal(&decimal),
    ];

    for policy in [
        NumericComparisonPolicy::Exact,
        NumericComparisonPolicy::Approximate,
    ] {
        for &left in &values {
            for &right in &values {
                let forward = compare_numeric(left, right, policy)
                    .expect("generated non-NaN numeric values must be ordered");
                let reverse = compare_numeric(right, left, policy)
                    .expect("generated non-NaN numeric values must be ordered");
                assert_eq!(reverse, forward.reverse());
            }
        }
    }
});

/// Reads a fixed-size little-endian field beginning at `offset`.
///
/// # Parameters
///
/// * `data` - Input bytes containing the field.
/// * `offset` - Byte offset at which the field begins.
///
/// # Returns
///
/// The copied field, or `None` when the requested range is out of bounds.
fn read_array<const N: usize>(data: &[u8], offset: usize) -> Option<[u8; N]> {
    let end = offset.checked_add(N)?;
    data.get(offset..end)?.try_into().ok()
}
