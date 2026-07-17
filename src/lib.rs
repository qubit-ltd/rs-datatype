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
//! vocabulary plus [`DataTypeOf`]. Optional `chrono`, `big-number`, `url`, and
//! `json` features add mappings for external types. The `converter` feature
//! enables core scalar, string, and Duration conversions. Combine it with a
//! rich-type feature to enable conversions for that family, or use `all`.
//!
//! # Conversion contract
//!
//! With `converter`, strings can target fixed-width numeric, boolean,
//! character, and Duration values. Fixed-width integers can target numeric,
//! boolean, and Duration values; floats can target fixed-width numeric values;
//! Duration can target fixed-width integers and String. `chrono`,
//! `big-number`, `url`, and `json` add their corresponding source and target
//! conversions when combined with `converter`; JSON also enables StringMap
//! parsing and formatting. Other type pairs return
//! `DataConversionErrorKind::Unsupported`.
//!
//! Numeric conversion defaults to `NumericConversionPolicy::Exact`, which
//! rejects truncation, rounding, and precision loss. Explicit `Lossy` mode
//! permits finite decimal/float truncation toward zero, integer-to-float IEEE
//! rounding, and Duration half-up rounding. Duration-to-integer and
//! Duration-to-String require exact divisibility in Exact mode.
//!
//! Strings are not trimmed by default and are normalized exactly once.
//! Boolean text defaults to `true` and `false`; numeric 0/1 handling is
//! controlled independently by `BooleanNumericPolicy`. Duration text uses
//! `[0-9]+(ns|us|µs|μs|ms|s|m|h|d)?`.
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
    DataConversionError,
    DataConversionErrorKind,
    DataConversionOptions,
    DataConversionTarget,
    DataConverter,
    DataConverters,
    DataFormat,
    DataListConversionError,
    DurationConversionOptions,
    DurationOverflowError,
    DurationUnit,
    EmptyItemPolicy,
    InvalidValueReason,
    NumericConversionPolicy,
    ScalarItem,
    ScalarItemError,
    ScalarItems,
    ScalarStringDataConverters,
    StringConversionOptions,
    StringNormalizationError,
    SuffixlessDurationPolicy,
};
pub use datatype::{
    DataType,
    DataTypeOf,
    DataTypeParseError,
};
pub use numeric::{
    NumericComparisonPolicy,
    NumericValueRef,
    compare_numeric,
};
