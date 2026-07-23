// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Runtime data type descriptors and policy-driven conversion utilities.
//!
//! The default feature set is empty and exposes the lightweight [`DataType`]
//! vocabulary plus [`DataTypeOf`]. The lightweight `duration` feature adds
//! Duration units, text codecs, and Serde field adapters. Optional `chrono`,
//! `big-integer`, `big-decimal`, `url`, and `json` features add mappings for
//! external types; `big-number` enables both big-number families. The
//! `converter` feature includes `duration` and enables core scalar, string, and
//! Duration conversions. Combine it with a rich-type feature to enable
//! conversions for that family, or use `all`.
//!
//! # Conversion contract
//!
//! With `converter`, strings can target fixed-width numeric, boolean,
//! character, and Duration values. Fixed-width integers can target numeric,
//! boolean, and Duration values; floats can target fixed-width numeric values;
//! Duration can target fixed-width integers and String. `chrono`,
//! `big-integer`, `big-decimal`, `url`, and `json` add their corresponding
//! source and target conversions when combined with `converter`; JSON also
//! enables StringMap parsing and formatting. Other type pairs return
//! `DataConversionErrorKind::Unsupported`.
//!
//! `NumericConversionOptions` configures fractional-to-integer conversion,
//! existing-numeric-to-float rounding, text-to-float rounding, and resource
//! limits independently. The strict/default profile rejects truncation and
//! precision loss. `DataConversionOptions::lossy` permits finite
//! decimal/float truncation toward zero, IEEE nearest-even float rounding, and
//! Duration half-up rounding. Duration-to-integer and Duration-to-String
//! require exact divisibility unless `DurationRoundingPolicy::HalfUp` is
//! selected explicitly.
//!
//! Strings are not trimmed by default and are normalized exactly once.
//! Boolean text defaults to `true` and `false`; numeric 0/1 handling is
//! controlled independently by `BooleanNumericPolicy`. By default, Duration
//! text rejects omitted suffixes and accepts
//! `[0-9]+(ns|us|µs|μs|ms|s|min|h|d)`. Strict mode accepts all three
//! microsecond spellings; lenient mode additionally accepts `m` for minutes.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "converter")]
//! # {
//! use qubit_datatype::{
//!     DataConversionErrorKind, InvalidValueReason, DataConversionOptions,
//!     DataConverter,
//! };
//!
//! let error = DataConverter::from("3.9").to::<i32>().unwrap_err();
//! assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
//! assert_eq!(error.reason(), Some(&InvalidValueReason::PrecisionLoss));
//!
//! let lossy = DataConversionOptions::lossy();
//! assert_eq!(DataConverter::from(" 3.9 ").to_with::<i32>(&lossy), Ok(3));
//! # }
//! ```

#![deny(missing_docs)]

/// Data type descriptors and compile-time type mappings.
pub mod datatype;

/// Policy-driven numeric comparison primitives.
pub mod numeric;

/// Lightweight Duration units and text codecs.
#[cfg(feature = "duration")]
pub mod duration;

#[cfg(feature = "duration")]
#[path = "serde/mod.rs"]
mod serde_impl;

/// Serde adapters for Duration interchange formats.
///
/// Use these modules with `#[serde(with = "...")]` when a field requires a
/// stable numeric or unit-suffixed Duration representation.
#[cfg(feature = "duration")]
pub mod serde {
    pub use super::serde_impl::{
        duration_millis,
        duration_millis_with_unit,
        duration_with_unit,
    };
}

/// Runtime value conversion utilities.
#[cfg(feature = "converter")]
pub mod converter;

#[cfg(feature = "converter")]
pub use converter::{
    BlankStringPolicy,
    BooleanConversionOptions,
    BooleanLiteralConflictError,
    BooleanNumericPolicy,
    CollectionConversionOptions,
    ConversionLimit,
    DataConversionError,
    DataConversionErrorKind,
    DataConversionOptions,
    DataConversionTarget,
    DataConverter,
    DataConverters,
    DataFormat,
    DataListConversionError,
    DurationConversionOptions,
    DurationRoundingPolicy,
    EmptyItemPolicy,
    FloatRoundingPolicy,
    FractionalToIntegerPolicy,
    InvalidValueReason,
    NumericConversionLimits,
    NumericConversionOptions,
    ScalarItem,
    ScalarItemError,
    ScalarItems,
    ScalarStringDataConverters,
    StringConversionOptions,
    StringNormalizationError,
    StructuredConversionLimits,
};
pub use datatype::{
    DataType,
    DataTypeOf,
    DataTypeParseError,
};
#[cfg(feature = "duration")]
pub use duration::{
    DurationOverflowError,
    DurationParseError,
    DurationTextOptions,
    DurationUnit,
    DurationUnitParseMode,
    SuffixlessDurationPolicy,
    format_duration_exact,
    parse_duration_text,
};
pub use numeric::{
    NumberRef,
    NumericComparisonPolicy,
};
