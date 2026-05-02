/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Blank String Policy
//!
//! Defines how blank string sources are interpreted during conversion.
//!

/// Controls how blank string sources are interpreted during conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlankStringPolicy {
    /// Keep blank strings as real string values.
    Preserve,
    /// Treat blank strings as missing values.
    TreatAsMissing,
    /// Reject blank strings as invalid input.
    Reject,
}
