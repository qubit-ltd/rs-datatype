// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Allocation regressions for conversion rejection before materialization.

use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::alloc::System;
use std::cell::Cell;

#[cfg(feature = "big-integer")]
use num_bigint::BigInt;
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;
#[cfg(feature = "big-integer")]
use qubit_datatype::NumericConversionLimits;
#[cfg(feature = "url")]
use url::Url;

/// Delegates allocation to the system while measuring only the current test
/// thread.
struct TrackingAllocator;

thread_local! {
    static MAX_ALLOCATION: Cell<Option<usize>> = const { Cell::new(None) };
}

/// Records a requested allocation size without allocating or panicking during
/// TLS teardown.
fn record_allocation(size: usize) {
    let _ = MAX_ALLOCATION.try_with(|maximum| {
        if let Some(previous) = maximum.get() {
            maximum.set(Some(previous.max(size)));
        }
    });
}

// SAFETY: Every operation forwards its original pointer and layout unchanged to
// System. The thread-local counter owns no allocation and never unwinds.
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record_allocation(layout.size());
        // SAFETY: The caller provides a valid allocation layout, forwarded unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record_allocation(layout.size());
        // SAFETY: The caller provides a valid allocation layout, forwarded unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record_allocation(size);
        // SAFETY: The pointer originates from System and the caller supplies its
        // original layout and a valid new size, all forwarded unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: The pointer originates from System and the caller supplies the
        // matching layout; this wrapper preserves both values.
        unsafe { System.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator;

/// Stops measurement even when the measured operation unwinds.
struct MeasurementGuard;

impl Drop for MeasurementGuard {
    fn drop(&mut self) {
        MAX_ALLOCATION.with(|maximum| maximum.set(None));
    }
}

/// Runs one operation and returns its result and largest allocation request.
/// Input setup and result assertions must occur outside this non-nested scope.
fn measure<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    MAX_ALLOCATION.with(|maximum| maximum.set(Some(0)));
    let guard = MeasurementGuard;
    let result = operation();
    let maximum = MAX_ALLOCATION.with(|maximum| maximum.get().unwrap_or(0));
    drop(guard);
    (result, maximum)
}

/// Creates a small output budget before measuring conversion allocations.
fn output_limits() -> ConversionLimits {
    ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_output_bytes(8).build())
        .build()
}

/// Borrowed String rejection must not allocate a copy of the source payload.
#[test]
fn test_borrowed_string_rejection_does_not_copy_payload() {
    let input = "x".repeat(1024 * 1024);
    let policy = ConversionPolicy::default();
    let limits = output_limits();
    for consume in [false, true] {
        let source = DataConverter::from(input.as_str());
        let mut session = ConversionSession::new(&policy, &limits);
        let (result, maximum) = measure(|| {
            if consume {
                source.into_target_in::<String>(&mut session)
            } else {
                source.to_in::<String>(&mut session)
            }
        });
        let error = result.expect_err("the borrowed String exceeds the output budget");
        assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
        assert_eq!(session.output_bytes_used(), 0);
        assert!(maximum < 4096, "rejected String allocated {maximum} bytes");
    }
}

/// Borrowed URL rejection must not clone its serialized URL payload.
#[cfg(feature = "url")]
#[test]
fn test_borrowed_url_rejection_does_not_copy_payload() {
    let input = Url::parse(&format!("https://example.com/{}", "x".repeat(1024 * 1024)))
        .expect("the large test URL has a valid scheme, host, and path");
    let policy = ConversionPolicy::default();
    let limits = output_limits();
    for consume in [false, true] {
        let source = DataConverter::from(&input);
        let mut session = ConversionSession::new(&policy, &limits);
        let (result, maximum) = measure(|| {
            if consume {
                source.into_target_in::<String>(&mut session)
            } else {
                source.to_in::<String>(&mut session)
            }
        });
        let error = result.expect_err("the borrowed URL exceeds the output budget");
        assert_eq!(error.resource(), Some(ConversionResource::OutputBytes));
        assert_eq!(session.output_bytes_used(), 0);
        assert!(maximum < 4096, "rejected URL allocated {maximum} bytes");
    }
}

/// Obvious digit overflow must not format the full arbitrary-precision integer.
#[cfg(feature = "big-integer")]
#[test]
fn test_big_integer_rejection_does_not_format_payload() {
    let input = BigInt::from(1_u8) << 1_000_000_usize;
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .numeric_limits(NumericConversionLimits::builder().max_big_integer_digits(8).build())
        .build();
    for consume in [false, true] {
        let source = DataConverter::from(&input);
        let mut session = ConversionSession::new(&policy, &limits);
        let (result, maximum) = measure(|| {
            if consume {
                source.into_target_in::<BigInt>(&mut session)
            } else {
                source.to_in::<BigInt>(&mut session)
            }
        });
        let error = result.expect_err("the large BigInt exceeds the eight-digit budget");
        assert_eq!(error.resource(), Some(ConversionResource::BigIntegerDigits));
        assert_eq!(error.observation_is_exact(), Some(false));
        assert!(
            error
                .observed_lower_bound()
                .expect("digit-limit errors retain a lower bound")
                > 8
        );
        assert!(maximum < 4096, "rejected BigInt allocated {maximum} bytes");
    }
}

/// A generous digit budget must preserve allocation-free owned integer
/// conversion.
#[cfg(feature = "big-integer")]
#[test]
fn test_owned_big_integer_with_maximum_digit_limit_does_not_format_payload() {
    let input = BigInt::from(1_u8) << 1_000_000_usize;
    let expected = input.clone();
    let source = DataConverter::from(input);
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .numeric_limits(
            NumericConversionLimits::builder()
                .max_big_integer_digits(u64::MAX)
                .build(),
        )
        .build();
    let mut session = ConversionSession::new(&policy, &limits);
    let (result, maximum) = measure(|| source.into_target_in::<BigInt>(&mut session));
    assert_eq!(result.expect("the integer fits the maximum digit budget"), expected);
    assert_eq!(maximum, 0, "owned BigInt conversion allocated {maximum} bytes");
}
