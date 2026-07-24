// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Visitor for exact unit-suffixed duration text.

use std::{
    fmt,
    time::Duration,
};

use serde::de::{
    Error as DeserializeError,
    Visitor,
};

use super::super::duration_with_unit::parse;

/// Parses borrowed or owned exact unit-suffixed duration text.
pub(in crate::serde_impl) struct DurationWithUnitVisitor;

impl Visitor<'_> for DurationWithUnitVisitor {
    type Value = Duration;

    /// Describes the strict unit-suffixed string accepted by this visitor.
    ///
    /// # Parameters
    ///
    /// * `formatter` - Formatter receiving the expected input description.
    ///
    /// # Returns
    ///
    /// The formatter result after writing the duration grammar.
    #[inline(always)]
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a strict unit-suffixed duration string")
    }

    /// Parses a borrowed or transient string without taking ownership.
    ///
    /// # Type Parameters
    ///
    /// * `E` - Serde deserialization error type.
    ///
    /// # Parameters
    ///
    /// * `value` - Duration text supplied by the deserializer.
    ///
    /// # Returns
    ///
    /// The parsed [`Duration`].
    ///
    /// # Errors
    ///
    /// Returns the visitor error when [`parse`] rejects the text.
    #[inline(always)]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        parse(value).map_err(E::custom)
    }

    /// Parses an owned string supplied by a non-borrowing deserializer.
    ///
    /// # Type Parameters
    ///
    /// * `E` - Serde deserialization error type.
    ///
    /// # Parameters
    ///
    /// * `value` - Owned duration text supplied by the deserializer.
    ///
    /// # Returns
    ///
    /// The parsed [`Duration`].
    ///
    /// # Errors
    ///
    /// Returns the visitor error when [`parse`] rejects the text.
    #[inline(always)]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        self.visit_str(&value)
    }
}
