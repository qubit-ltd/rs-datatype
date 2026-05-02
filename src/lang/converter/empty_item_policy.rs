/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Empty Item Policy
//!
//! Defines how empty collection items are interpreted after splitting.
//!
//! # Author
//!
//! Haixing Hu

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
