// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Allocation-free writer used to measure formatted conversion output.

use std::fmt;
use std::io;

/// Counts bytes accepted by formatting and serialization writers.
#[derive(Debug, Default)]
pub(crate) struct CountingWriter {
    bytes: usize,
}

impl CountingWriter {
    /// Creates an empty byte-counting writer.
    #[inline]
    pub(crate) const fn new() -> Self {
        Self { bytes: 0 }
    }

    /// Returns the number of bytes accepted so far.
    #[inline]
    pub(crate) const fn bytes(&self) -> usize {
        self.bytes
    }

    /// Adds a formatted byte count without allowing integer wraparound.
    #[inline]
    fn add(&mut self, amount: usize) -> Result<(), ()> {
        self.bytes = self.bytes.checked_add(amount).ok_or(())?;
        Ok(())
    }
}

impl fmt::Write for CountingWriter {
    /// Counts one UTF-8 string fragment.
    fn write_str(&mut self, value: &str) -> fmt::Result {
        self.add(value.len()).map_err(|_| fmt::Error)
    }
}

impl io::Write for CountingWriter {
    /// Counts one serialized byte fragment.
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.add(bytes.len())
            .map(|()| bytes.len())
            .map_err(|_| io::Error::other("formatted output length overflow"))
    }

    /// Flushes the count-only writer.
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
