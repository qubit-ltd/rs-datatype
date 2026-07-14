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
//! enables the complete conversion engine and all rich-type features.
//!
//! # Conversion contract
//!
//! With `converter`, strings can target numeric, boolean, character, temporal,
//! Duration, URL, JSON, and StringMap values. Integers and BigInt can target
//! numeric, boolean, and Duration values; floats and BigDecimal can target
//! numeric values; Duration can target integers and String; StringMap can
//! target JSON and String. Other type pairs return
//! `DataConversionError::Unsupported`.
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
//! `[0-9]+(ns|us|ms|s|m|h|d)?`.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "converter")]
//! # {
//! use qubit_datatype::{
//!     DataConversionError, InvalidValueReason, DataConversionOptions,
//!     DataConverter, NumericConversionPolicy, StringConversionOptions,
//! };
//!
//! assert!(matches!(
//!     DataConverter::from("3.9").to::<i32>(),
//!     Err(DataConversionError::InvalidValue {
//!         reason: InvalidValueReason::PrecisionLoss,
//!         ..
//!     }),
//! ));
//!
//! let lossy = DataConversionOptions::default()
//!     .with_numeric_policy(NumericConversionPolicy::Lossy)
//!     .with_string_options(StringConversionOptions::default().with_trim(true));
//! assert_eq!(DataConverter::from(" 3.9 ").to_with::<i32>(&lossy), Ok(3));
//! # }
//! ```

#![deny(missing_docs)]

/// Data type descriptors and compile-time type mappings.
pub mod datatype;

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
    DataConversionOptions,
    DataConvertTo,
    DataConverter,
    DataConverters,
    DataFormat,
    DataListConversionError,
    DurationConversionOptions,
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
};
pub use datatype::{
    DataType,
    DataTypeOf,
    DataTypeParseError,
};
