// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Borrowed-string-only deserializer used by serde adapter tests.

use serde::de::{
    Deserializer,
    Visitor,
    value::Error as ValueError,
};
use serde::forward_to_deserialize_any;

/// Exposes borrowed string input without supporting owned string requests.
pub(crate) struct BorrowedStrOnlyDeserializer<'de> {
    /// Borrowed string value supplied to the visitor.
    value: &'de str,
}

impl<'de> BorrowedStrOnlyDeserializer<'de> {
    /// Creates a borrowed-string-only deserializer.
    ///
    /// # Parameters
    ///
    /// * `value` - String value borrowed for the deserialization lifetime.
    ///
    /// # Returns
    ///
    /// A deserializer that accepts only `deserialize_str` requests.
    pub(crate) const fn new(value: &'de str) -> Self {
        Self { value }
    }
}

impl<'de> Deserializer<'de> for BorrowedStrOnlyDeserializer<'de> {
    type Error = ValueError;

    /// Rejects generic deserialization requests.
    ///
    /// # Parameters
    ///
    /// * `visitor` - Visitor that is intentionally not invoked.
    ///
    /// # Returns
    ///
    /// Always returns an error because this helper exposes only borrowed text.
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(<ValueError as serde::de::Error>::custom(
            "only deserialize_str is supported",
        ))
    }

    /// Supplies the borrowed string directly to the visitor.
    ///
    /// # Parameters
    ///
    /// * `visitor` - Visitor receiving the borrowed string.
    ///
    /// # Returns
    ///
    /// The value produced by the visitor.
    ///
    /// # Errors
    ///
    /// Returns the visitor error when it rejects the supplied string.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.value)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
