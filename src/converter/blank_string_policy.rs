/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Blank String Policy
//!
//! Defines how blank string sources are interpreted during conversion.
//!
//! # Author
//!
//! Haixing Hu

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
