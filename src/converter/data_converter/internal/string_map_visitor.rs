// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Duplicate-aware visitor for string-valued JSON objects.

use std::collections::HashMap;
use std::fmt;

use serde::de::{
    MapAccess,
    Visitor,
};

/// Accepts JSON objects whose keys are unique and values are strings.
pub(in crate::converter::data_converter) struct StringMapVisitor;

impl<'de> Visitor<'de> for StringMapVisitor {
    type Value = HashMap<String, String>;

    /// Describes the accepted JSON shape.
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON object with unique keys and string values")
    }

    /// Reads the map while checking duplicate keys.
    ///
    /// # Parameters
    ///
    /// * `access` - Serde map access for the object being decoded.
    ///
    /// # Returns
    ///
    /// Returns an owned string map when every key is unique.
    ///
    /// # Errors
    ///
    /// Returns the deserializer error for invalid entries or duplicate keys.
    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut result =
            HashMap::with_capacity(access.size_hint().unwrap_or(0));
        while let Some((key, value)) = access.next_entry::<String, String>()? {
            if result.insert(key, value).is_some() {
                return Err(serde::de::Error::custom("duplicate object key"));
            }
        }
        Ok(result)
    }
}
