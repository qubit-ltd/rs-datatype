// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Type Definitions (Language Layer)
//!
//! Provides cross-module reusable common data type enum `DataType` and type
//! mapping `DataTypeOf`.

use super::data_type_parse_error::DataTypeParseError;

use serde::{
    Deserialize,
    Serialize,
};
use strum::{
    Display,
    EnumString,
    IntoStaticStr,
    VariantArray,
};

/// Universal data type enumeration for cross-module type representation
///
/// Defines the fixed runtime data types supported by the system.
/// This enum provides a unified way to represent and work with different data
/// types across various modules and components.
///
/// `DataType` serves as a bridge between Rust's type system and runtime type
/// information, enabling dynamic type handling, serialization, validation,
/// and other type-aware operations.
///
/// # Features
///
/// - **Stable Coverage**: Supports fixed-width Rust primitives and common
///   third-party types while intentionally omitting `isize` and `usize`
/// - **String Representation**: Each variant has a consistent string
///   representation
/// - **Serialization Support**: Implements `Serialize` and `Deserialize` for
///   JSON/YAML support
/// - **Type Mapping**: Works with `DataTypeOf` trait for compile-time type
///   mapping
///
/// # Use Cases
///
/// - **Dynamic Type Handling**: Runtime type checking and conversion
/// - **Serialization/Deserialization**: Type-aware data format conversion
/// - **Validation Systems**: Type-based input validation
/// - **Generic Programming**: Type-safe generic operations
/// - **API Documentation**: Automatic type information generation
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use qubit_datatype::DataType;
///
/// let data_type = DataType::Int32;
/// assert_eq!(data_type.to_string(), "int32");
/// assert_eq!(data_type.as_str(), "int32");
/// ```
///
/// ## Type Checking
///
/// ```rust
/// use qubit_datatype::DataType;
///
/// fn is_numeric(data_type: DataType) -> bool {
///     matches!(data_type,
///         DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 | DataType::Int128 |
///         DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64 | DataType::UInt128 |
///         DataType::Float32 | DataType::Float64 | DataType::BigInteger | DataType::BigDecimal
///     )
/// }
///
/// assert!(is_numeric(DataType::Int32));
/// assert!(!is_numeric(DataType::String));
/// ```
///
/// ## Serialization
///
/// ```rust
/// use qubit_datatype::DataType;
/// use serde_json;
///
/// let data_type = DataType::Float64;
/// let json = serde_json::to_string(&data_type).unwrap();
/// assert_eq!(json, "\"float64\"");
///
/// let deserialized: DataType = serde_json::from_str(&json).unwrap();
/// assert_eq!(deserialized, DataType::Float64);
/// ```
#[must_use]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    IntoStaticStr,
    VariantArray,
)]
#[serde(rename_all = "lowercase")]
#[strum(
    serialize_all = "lowercase",
    ascii_case_insensitive,
    parse_err_ty = DataTypeParseError,
    parse_err_fn = DataTypeParseError::new
)]
pub enum DataType {
    /// Boolean type
    Bool,
    /// Character type
    Char,
    /// 8-bit signed integer
    Int8,
    /// 16-bit signed integer
    Int16,
    /// 32-bit signed integer
    Int32,
    /// 64-bit signed integer
    Int64,
    /// 128-bit signed integer
    Int128,
    /// 8-bit unsigned integer
    UInt8,
    /// 16-bit unsigned integer
    UInt16,
    /// 32-bit unsigned integer
    UInt32,
    /// 64-bit unsigned integer
    UInt64,
    /// 128-bit unsigned integer
    UInt128,
    /// 32-bit floating point number
    Float32,
    /// 64-bit floating point number
    Float64,
    /// String type
    String,
    /// Date type (NaiveDate)
    Date,
    /// Time type (NaiveTime)
    Time,
    /// DateTime type (NaiveDateTime)
    DateTime,
    /// UTC time point (equivalent to Java Instant) (`DateTime<Utc>`)
    Instant,
    /// Big integer type (BigInt)
    BigInteger,
    /// Big decimal type (BigDecimal)
    BigDecimal,
    /// Duration type (std::time::Duration)
    Duration,
    /// URL type (url::Url)
    Url,
    /// String map type (HashMap<String, String>)
    StringMap,
    /// JSON value type (serde_json::Value)
    Json,
}

impl DataType {
    /// All data type variants in their stable declaration order.
    pub const ALL: &'static [DataType] = <DataType as VariantArray>::VARIANTS;

    /// Returns the stable lowercase name of this data type.
    ///
    /// # Returns
    ///
    /// The stable lowercase serialization and display name.
    #[must_use]
    #[inline(always)]
    pub fn as_str(self) -> &'static str {
        <&'static str>::from(self)
    }

    /// Tests whether this type belongs to the numeric family.
    ///
    /// # Returns
    ///
    /// `true` for fixed-width integers, primitive floats, and big numbers.
    #[must_use]
    #[inline(always)]
    pub const fn is_numeric(self) -> bool {
        self.is_integer() || self.is_float() || self.is_big_number()
    }

    /// Tests whether this type is a fixed-width integer.
    ///
    /// # Returns
    ///
    /// `true` for signed or unsigned fixed-width integer variants.
    #[must_use]
    #[inline(always)]
    pub const fn is_integer(self) -> bool {
        self.is_signed_integer() || self.is_unsigned_integer()
    }

    /// Tests whether this type is a signed integer.
    ///
    /// # Returns
    ///
    /// `true` for `Int8` through `Int128`.
    #[must_use]
    #[inline(always)]
    pub const fn is_signed_integer(self) -> bool {
        matches!(
            self,
            DataType::Int8
                | DataType::Int16
                | DataType::Int32
                | DataType::Int64
                | DataType::Int128
        )
    }

    /// Tests whether this type is an unsigned integer.
    ///
    /// # Returns
    ///
    /// `true` for `UInt8` through `UInt128`.
    #[must_use]
    #[inline(always)]
    pub const fn is_unsigned_integer(self) -> bool {
        matches!(
            self,
            DataType::UInt8
                | DataType::UInt16
                | DataType::UInt32
                | DataType::UInt64
                | DataType::UInt128
        )
    }

    /// Tests whether this type is a primitive floating-point type.
    ///
    /// # Returns
    ///
    /// `true` for [`Self::Float32`] or [`Self::Float64`].
    #[must_use]
    #[inline(always)]
    pub const fn is_float(self) -> bool {
        matches!(self, DataType::Float32 | DataType::Float64)
    }

    /// Tests whether this type is an arbitrary-precision number.
    ///
    /// # Returns
    ///
    /// `true` for [`Self::BigInteger`] or [`Self::BigDecimal`].
    #[must_use]
    #[inline(always)]
    pub const fn is_big_number(self) -> bool {
        matches!(self, DataType::BigInteger | DataType::BigDecimal)
    }
}
