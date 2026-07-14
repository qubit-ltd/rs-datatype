// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Tests mirroring the focused DataConverter implementation modules.

mod boolean_tests;
#[cfg(all(feature = "big-number", feature = "chrono"))]
mod duration_tests;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
mod numeric_tests;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
mod source_tests;
mod string_source_tests;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
mod structured_tests;
#[cfg(all(
    feature = "big-number",
    feature = "chrono",
    feature = "url",
    feature = "json"
))]
mod text_tests;
