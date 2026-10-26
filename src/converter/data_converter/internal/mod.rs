// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private implementation details for runtime data conversion.

#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
mod big_number_text;
#[cfg(feature = "json")]
mod canonical_string_map;
pub(super) mod numeric;
#[cfg(feature = "json")]
mod string_map_visitor;

#[cfg(any(feature = "big-integer", feature = "big-decimal"))]
pub(super) use big_number_text::check_big_number_output;
#[cfg(feature = "json")]
pub(super) use canonical_string_map::CanonicalStringMap;
#[cfg(feature = "json")]
pub(super) use canonical_string_map::sorted_string_map_entries;
#[cfg(feature = "json")]
pub(super) use string_map_visitor::StringMapVisitor;
