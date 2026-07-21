// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lightweight Duration units, policies, parsing, and exact formatting.

mod duration_overflow_error;
mod duration_parse_error;
mod duration_text_options;
mod duration_unit;
mod duration_unit_parse_mode;
mod format_duration_exact;
mod parse_duration_text;
mod suffixless_duration_policy;

pub use duration_overflow_error::DurationOverflowError;
pub use duration_parse_error::DurationParseError;
pub use duration_text_options::DurationTextOptions;
pub use duration_unit::DurationUnit;
pub use duration_unit_parse_mode::DurationUnitParseMode;
pub use format_duration_exact::format_duration_exact;
pub use parse_duration_text::parse_duration_text;
pub use suffixless_duration_policy::SuffixlessDurationPolicy;
