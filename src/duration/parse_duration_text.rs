// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Parsing for canonical non-negative Duration text.

use std::time::Duration;

use super::{
    DurationParseError,
    DurationTextOptions,
    DurationUnit,
    DurationUnitParseMode,
    SuffixlessDurationPolicy,
};

/// Parses non-negative integer Duration text according to explicit policies.
///
/// This function does not trim or otherwise normalize `text`.
///
/// # Parameters
///
/// * `text` - Duration text containing digits and an optional unit suffix.
/// * `options` - Suffixless-input and explicit-suffix policies.
///
/// # Returns
///
/// The represented Duration.
///
/// # Errors
///
/// Returns [`DurationParseError::LimitExceeded`] when the source text exceeds
/// the configured byte limit, [`DurationParseError::InvalidSyntax`] for
/// malformed text, [`DurationParseError::UnsupportedUnit`] for an unknown
/// alphabetic suffix, [`DurationParseError::NonCanonicalUnit`] for a
/// Lenient-only alias in strict mode, and [`DurationParseError::OutOfRange`]
/// for numeric or Duration overflow.
pub fn parse_duration_text(
    text: &str,
    options: &DurationTextOptions,
) -> Result<Duration, DurationParseError> {
    if text.len() > options.max_text_bytes() {
        return Err(DurationParseError::LimitExceeded {
            maximum: options.max_text_bytes(),
        });
    }
    let split_at = text
        .bytes()
        .position(|byte| !byte.is_ascii_digit())
        .unwrap_or(text.len());
    let (digits, suffix) = text.split_at(split_at);
    if digits.is_empty() {
        return Err(DurationParseError::InvalidSyntax);
    }
    let unit = resolve_unit(suffix, options)?;
    let value = digits
        .parse::<u128>()
        .map_err(|_| DurationParseError::OutOfRange)?;
    unit.duration_from_u128(value)
        .map_err(|_| DurationParseError::OutOfRange)
}

/// Resolves an explicit or omitted suffix using the configured policies.
///
/// # Parameters
///
/// * `suffix` - Explicit suffix text, or an empty string when omitted.
/// * `options` - Suffixless and accepted-suffix policies.
///
/// # Returns
///
/// The resolved duration unit.
///
/// # Errors
///
/// Returns invalid syntax for a rejected omission or malformed suffix,
/// [`DurationParseError::NonCanonicalUnit`] for a Lenient-only alias in
/// strict mode, and unsupported unit for an unknown alphabetic suffix.
fn resolve_unit(
    suffix: &str,
    options: &DurationTextOptions,
) -> Result<DurationUnit, DurationParseError> {
    if suffix.is_empty() {
        return match options.suffixless_policy() {
            SuffixlessDurationPolicy::Reject => {
                Err(DurationParseError::InvalidSyntax)
            }
            SuffixlessDurationPolicy::Assume(unit) => Ok(unit),
        };
    }
    let result = match options.unit_parse_mode() {
        DurationUnitParseMode::Strict => DurationUnit::parse_strict(suffix),
        DurationUnitParseMode::Lenient => DurationUnit::parse_lenient(suffix),
    };
    match result {
        Ok(unit) => Ok(unit),
        Err(DurationParseError::UnsupportedUnit { .. })
            if !suffix.chars().all(char::is_alphabetic) =>
        {
            Err(DurationParseError::InvalidSyntax)
        }
        Err(error) => Err(error),
    }
}
