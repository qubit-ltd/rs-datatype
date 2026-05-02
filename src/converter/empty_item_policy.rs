/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Empty Item Policy
//!
//! Defines how empty collection items are interpreted after splitting.
//!

/// Controls how empty collection items are interpreted after splitting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmptyItemPolicy {
    /// Keep empty items and pass them to the element converter.
    Keep,
    /// Drop empty items before element conversion.
    Skip,
    /// Reject empty items as invalid input.
    Reject,
}
