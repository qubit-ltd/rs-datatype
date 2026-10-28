// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

#![cfg(any(feature = "big-integer", feature = "big-decimal"))]

use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::alloc::System;
use std::cell::Cell;
use std::cmp::Ordering;

#[cfg(feature = "big-decimal")]
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use qubit_datatype::NumberRef;
use qubit_datatype::NumericComparisonPolicy;

struct CountingAllocator;

thread_local! {
    static ALLOCATIONS: Cell<Option<(usize, usize)>> = const { Cell::new(None) };
}

fn record_allocation(size: usize) {
    let _ = ALLOCATIONS.try_with(|allocations| {
        if let Some((count, large_count)) = allocations.get() {
            allocations.set(Some((
                count.saturating_add(1),
                large_count.saturating_add(usize::from(size >= 64 * 1024)),
            )));
        }
    });
}

// SAFETY: Every operation delegates to the system allocator with the same
// layout.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record_allocation(layout.size());
        // SAFETY: The caller guarantees that `layout` is valid.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: The caller guarantees that `pointer` was allocated with `layout`.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record_allocation(layout.size());
        // SAFETY: The caller guarantees that `layout` is valid.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        record_allocation(new_size);
        // SAFETY: The caller guarantees the pointer/layout pair is valid.
        unsafe { System.realloc(pointer, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

struct AllocationTracking {
    armed: bool,
}

impl AllocationTracking {
    fn start() -> Self {
        let _ = ALLOCATIONS.try_with(|allocations| allocations.set(Some((0, 0))));
        Self { armed: true }
    }

    fn finish(mut self) -> (usize, usize) {
        let counts = ALLOCATIONS
            .try_with(|allocations| allocations.replace(None).unwrap_or((0, 0)))
            .unwrap_or((0, 0));
        self.armed = false;
        counts
    }
}

impl Drop for AllocationTracking {
    fn drop(&mut self) {
        if self.armed {
            let _ = ALLOCATIONS.try_with(|allocations| allocations.set(None));
        }
    }
}

fn compare_exact(left: NumberRef<'_>, right: NumberRef<'_>) -> Option<Ordering> {
    left.compare(right, NumericComparisonPolicy::Exact)
}

#[test]
#[cfg(feature = "big-integer")]
fn test_big_integer_float_easy_comparisons_do_not_allocate() {
    let huge_positive = BigInt::from(1_u8) << 4096;
    let huge_negative = -&huge_positive;
    let zero = BigInt::from(0_u8);
    let one = BigInt::from(1_u8);
    let cases = [
        (NumberRef::from(&zero), NumberRef::from(0.0_f64), Ordering::Equal),
        (NumberRef::from(0.0_f64), NumberRef::from(&zero), Ordering::Equal),
        (NumberRef::from(&zero), NumberRef::from(-0.0_f32), Ordering::Equal),
        (NumberRef::from(-0.0_f32), NumberRef::from(&zero), Ordering::Equal),
        (NumberRef::from(&zero), NumberRef::from(-0.0_f64), Ordering::Equal),
        (NumberRef::from(-0.0_f64), NumberRef::from(&zero), Ordering::Equal),
        (NumberRef::from(&zero), NumberRef::from(1.0_f32), Ordering::Less),
        (NumberRef::from(1.0_f32), NumberRef::from(&zero), Ordering::Greater),
        (NumberRef::from(&zero), NumberRef::from(-1.0_f32), Ordering::Greater),
        (NumberRef::from(-1.0_f32), NumberRef::from(&zero), Ordering::Less),
        (NumberRef::from(&zero), NumberRef::from(1.0_f64), Ordering::Less),
        (NumberRef::from(1.0_f64), NumberRef::from(&zero), Ordering::Greater),
        (NumberRef::from(&zero), NumberRef::from(-1.0_f64), Ordering::Greater),
        (NumberRef::from(-1.0_f64), NumberRef::from(&zero), Ordering::Less),
        (
            NumberRef::from(&huge_positive),
            NumberRef::from(-1.0_f64),
            Ordering::Greater,
        ),
        (
            NumberRef::from(-1.0_f64),
            NumberRef::from(&huge_positive),
            Ordering::Less,
        ),
        (
            NumberRef::from(&huge_negative),
            NumberRef::from(1.0_f64),
            Ordering::Less,
        ),
        (
            NumberRef::from(1.0_f64),
            NumberRef::from(&huge_negative),
            Ordering::Greater,
        ),
        (
            NumberRef::from(&huge_positive),
            NumberRef::from(1.0_f64),
            Ordering::Greater,
        ),
        (
            NumberRef::from(1.0_f64),
            NumberRef::from(&huge_positive),
            Ordering::Less,
        ),
        (
            NumberRef::from(&one),
            NumberRef::from(f64::from_bits(1)),
            Ordering::Greater,
        ),
        (
            NumberRef::from(f64::from_bits(1)),
            NumberRef::from(&one),
            Ordering::Less,
        ),
        (
            NumberRef::from(&one),
            NumberRef::from(-f64::from_bits(1)),
            Ordering::Greater,
        ),
        (
            NumberRef::from(-f64::from_bits(1)),
            NumberRef::from(&one),
            Ordering::Less,
        ),
    ];

    for &(left, right, expected) in &cases {
        let (actual, allocations) = count_allocations(|| std::hint::black_box(compare_exact(left, right)));
        assert_eq!(actual, Some(expected));
        assert_eq!(allocations, 0);
    }
}

#[cfg(feature = "big-decimal")]
#[test]
fn test_big_decimal_scale_fallback_avoids_redundant_coefficient_clone() {
    for scale in [4097_i64, -4097, i64::MIN, i64::MAX] {
        let coefficient = -(BigInt::from(1_u8) << 1_048_576_usize);
        let decimal = BigDecimal::new(coefficient, scale);
        for zero in [NumberRef::from(0_i32), NumberRef::from(0.0_f64)] {
            for reverse in [false, true] {
                let (left, right, expected) = if reverse {
                    (zero, NumberRef::from(&decimal), Ordering::Greater)
                } else {
                    (NumberRef::from(&decimal), zero, Ordering::Less)
                };
                let (actual, large_allocations) = count_large_allocations(|| {
                    std::hint::black_box(compare_exact(std::hint::black_box(left), std::hint::black_box(right)))
                });
                assert_eq!(actual, Some(expected));
                assert!(
                    large_allocations <= 1,
                    "scale={scale}, reverse={reverse}, large allocations={large_allocations}"
                );
            }
        }
    }
}

#[cfg(feature = "big-integer")]
fn count_allocations<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    let tracking = AllocationTracking::start();
    let result = operation();
    let (allocations, _) = tracking.finish();
    (result, allocations)
}

#[cfg(feature = "big-decimal")]
fn count_large_allocations<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    let tracking = AllocationTracking::start();
    let result = operation();
    let (_, large_allocations) = tracking.finish();
    (result, large_allocations)
}
