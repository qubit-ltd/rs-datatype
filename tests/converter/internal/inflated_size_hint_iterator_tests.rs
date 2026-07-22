// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Iterator with an intentionally dishonest lower size bound.

/// Yields one value while advertising the largest possible lower bound.
pub(in crate::converter) struct InflatedSizeHintIterator {
    /// The only value still available from the iterator.
    value: Option<&'static str>,
}

impl InflatedSizeHintIterator {
    /// Creates an iterator that yields `value` once.
    pub(in crate::converter) const fn new(value: &'static str) -> Self {
        Self { value: Some(value) }
    }
}

impl Iterator for InflatedSizeHintIterator {
    type Item = &'static str;

    /// Returns the remaining test value.
    fn next(&mut self) -> Option<Self::Item> {
        self.value.take()
    }

    /// Returns an intentionally invalid lower bound for robustness testing.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}
