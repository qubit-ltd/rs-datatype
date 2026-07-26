// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Canonical serialization support for string maps.

use std::collections::HashMap;

use serde::ser::SerializeMap;
use serde::{
    Serialize,
    Serializer,
};

/// Borrows a string map and serializes its entries in lexicographic key order.
pub(in crate::converter::data_converter) struct CanonicalStringMap<'a> {
    /// String map whose entries are serialized in canonical order.
    pub(in crate::converter::data_converter) value: &'a HashMap<String, String>,
}

/// Collects borrowed map entries in lexicographic key order.
///
/// # Parameters
///
/// * `value` - String map whose entries need canonical ordering.
///
/// # Returns
///
/// Borrowed entries sorted by ascending key.
#[must_use]
pub(in crate::converter::data_converter) fn sorted_string_map_entries(
    value: &HashMap<String, String>,
) -> Vec<(&String, &String)> {
    let mut entries = value.iter().collect::<Vec<_>>();
    entries.sort_unstable_by_key(|(key, _)| *key);
    entries
}

impl Serialize for CanonicalStringMap<'_> {
    /// Serializes map entries in lexicographic key order.
    ///
    /// # Type Parameters
    ///
    /// * `S` - Serializer receiving the map entries.
    ///
    /// # Parameters
    ///
    /// * `serializer` - Serializer receiving the ordered string map.
    ///
    /// # Returns
    ///
    /// The serializer result after emitting every map entry.
    ///
    /// # Errors
    ///
    /// Returns the serializer error when initializing the map, writing an
    /// entry, or ending the map fails.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries = sorted_string_map_entries(self.value);
        let mut map = serializer.serialize_map(Some(entries.len()))?;
        for (key, value) in entries {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
