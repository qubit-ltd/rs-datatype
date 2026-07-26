// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private implementation details for runtime data conversion.

#[cfg(feature = "json")]
mod canonical_string_map;
pub(super) mod numeric;
#[cfg(feature = "json")]
mod string_map_visitor;

#[cfg(feature = "json")]
pub(super) use canonical_string_map::{
    CanonicalStringMap,
    sorted_string_map_entries,
};
#[cfg(feature = "json")]
pub(super) use string_map_visitor::StringMapVisitor;
