// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

#![cfg(all(feature = "converter", feature = "json"))]

use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::alloc::System;
use std::cell::Cell;
use std::collections::HashMap;

use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionResource;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;
use qubit_datatype::StructuredConversionLimits;

struct CountingAllocator;

thread_local! {
    static LARGE_ALLOCATIONS: Cell<Option<usize>> = const { Cell::new(None) };
}

fn record_allocation(size: usize) {
    let _ = LARGE_ALLOCATIONS.try_with(|count| {
        if let Some(current) = count.get() {
            count.set(Some(current.saturating_add(usize::from(size >= 64 * 1024))));
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
        // SAFETY: The caller guarantees that the pointer/layout pair is valid.
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
        let _ = LARGE_ALLOCATIONS.try_with(|count| count.set(Some(0)));
        Self { armed: true }
    }

    fn finish(mut self) -> usize {
        let count = LARGE_ALLOCATIONS
            .try_with(|count| count.replace(None).unwrap_or(0))
            .unwrap_or(0);
        self.armed = false;
        count
    }
}

impl Drop for AllocationTracking {
    fn drop(&mut self) {
        if self.armed {
            let _ = LARGE_ALLOCATIONS.try_with(|count| count.set(None));
        }
    }
}

fn assert_large_map_is_rejected_without_large_allocation(limit: u64) {
    let source = (0..16_384)
        .map(|index| (format!("key-{index:05}"), "value".to_owned()))
        .collect::<HashMap<_, _>>();
    let policy = ConversionPolicy::default();
    let limits = ConversionLimits::builder()
        .structured_limits(StructuredConversionLimits::builder().max_map_entries(limit).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    let tracking = AllocationTracking::start();
    let error = DataConverter::from(&source)
        .to_in::<String>(&mut session)
        .expect_err("known map length exceeds the configured entry limit");
    let large_allocations = tracking.finish();

    assert_eq!(error.resource(), Some(ConversionResource::MapEntries));
    assert_eq!(large_allocations, 0);
}

#[test]
fn test_string_map_output_rejects_large_map_at_zero_limit_before_large_allocation() {
    assert_large_map_is_rejected_without_large_allocation(0);
}

#[test]
fn test_string_map_output_rejects_large_map_at_one_limit_before_large_allocation() {
    assert_large_map_is_rejected_without_large_allocation(1);
}
