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
    DurationUnitSuffixSet,
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
/// Returns [`DurationParseError::InvalidSyntax`] for malformed text,
/// [`DurationParseError::UnsupportedUnit`] for an unknown alphabetic suffix,
/// and [`DurationParseError::OutOfRange`] for numeric or Duration overflow.
pub fn parse_duration_text(
    text: &str,
    options: &DurationTextOptions,
) -> Result<Duration, DurationParseError> {
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
/// Returns invalid syntax for a rejected omission or malformed suffix, and
/// unsupported unit for an unknown alphabetic suffix.
fn resolve_unit(
    suffix: &str,
    options: &DurationTextOptions,
) -> Result<DurationUnit, DurationParseError> {
    if suffix.is_empty() {
        return match options.suffixless_policy {
            SuffixlessDurationPolicy::Reject => {
                Err(DurationParseError::InvalidSyntax)
            }
            SuffixlessDurationPolicy::Assume(unit) => Ok(unit),
        };
    }
    if let Some(unit) = explicit_unit(suffix, options.unit_suffix_set) {
        return Ok(unit);
    }
    let alphabetic = match options.unit_suffix_set {
        DurationUnitSuffixSet::Ascii => {
            suffix.bytes().all(|byte| byte.is_ascii_alphabetic())
        }
        DurationUnitSuffixSet::Extended => {
            suffix.chars().all(char::is_alphabetic)
        }
    };
    if alphabetic {
        Err(DurationParseError::UnsupportedUnit {
            unit: suffix.to_owned(),
        })
    } else {
        Err(DurationParseError::InvalidSyntax)
    }
}

/// Matches a suffix from the selected supported set.
///
/// # Parameters
///
/// * `suffix` - Explicit suffix to match.
/// * `suffix_set` - Accepted ASCII or extended suffix profile.
///
/// # Returns
///
/// The matching unit, or `None` for an unsupported suffix.
fn explicit_unit(
    suffix: &str,
    suffix_set: DurationUnitSuffixSet,
) -> Option<DurationUnit> {
    match suffix {
        "ns" => Some(DurationUnit::Nanoseconds),
        "us" => Some(DurationUnit::Microseconds),
        "ms" => Some(DurationUnit::Milliseconds),
        "s" => Some(DurationUnit::Seconds),
        "m" => Some(DurationUnit::Minutes),
        "h" => Some(DurationUnit::Hours),
        "d" => Some(DurationUnit::Days),
        "µs" | "μs" if suffix_set == DurationUnitSuffixSet::Extended => {
            Some(DurationUnit::Microseconds)
        }
        _ => None,
    }
}
