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

use std::fmt;
use std::str::FromStr;

use super::data_type_parse_error::DataTypeParseError;

use serde::{
    Deserialize,
    Serialize,
};

/// Universal data type enumeration for cross-module type representation
///
/// Defines all basic data types and composite types supported by the system.
/// This enum provides a unified way to represent and work with different data
/// types across various modules and components.
///
/// `DataType` serves as a bridge between Rust's type system and runtime type
/// information, enabling dynamic type handling, serialization, validation,
/// and other type-aware operations.
///
/// # Features
///
/// - **Comprehensive Coverage**: Supports all basic Rust types plus common
///   third-party types
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

impl fmt::Display for DataType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

macro_rules! define_data_type_names {
    ($( $variant:ident => $name:literal ),+ $(,)?) => {
        impl FromStr for DataType {
            type Err = DataTypeParseError;

            /// Parses a case-insensitive data type name.
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value.to_ascii_lowercase().as_str() {
                    $( $name => Ok(DataType::$variant), )+
                    _ => Err(DataTypeParseError::new(value)),
                }
            }
        }

        impl DataType {
            /// All data type variants in their stable declaration order.
            pub const ALL: [DataType; 25] = [$( DataType::$variant, )+];

            /// Returns the stable lowercase name of this data type.
            #[inline]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $( DataType::$variant => $name, )+
                }
            }
        }
    };
}

define_data_type_names! {
    Bool => "bool",
    Char => "char",
    Int8 => "int8",
    Int16 => "int16",
    Int32 => "int32",
    Int64 => "int64",
    Int128 => "int128",
    UInt8 => "uint8",
    UInt16 => "uint16",
    UInt32 => "uint32",
    UInt64 => "uint64",
    UInt128 => "uint128",
    Float32 => "float32",
    Float64 => "float64",
    String => "string",
    Date => "date",
    Time => "time",
    DateTime => "datetime",
    Instant => "instant",
    BigInteger => "biginteger",
    BigDecimal => "bigdecimal",
    Duration => "duration",
    Url => "url",
    StringMap => "stringmap",
    Json => "json",
}

impl DataType {
    /// Tests whether this type belongs to the numeric family.
    #[inline]
    pub const fn is_numeric(self) -> bool {
        self.is_integer() || self.is_float() || self.is_big_number()
    }

    /// Tests whether this type is a fixed-width integer.
    #[inline]
    pub const fn is_integer(self) -> bool {
        self.is_signed_integer() || self.is_unsigned_integer()
    }

    /// Tests whether this type is a signed integer.
    #[inline]
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
    #[inline]
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
    #[inline]
    pub const fn is_float(self) -> bool {
        matches!(self, DataType::Float32 | DataType::Float64)
    }

    /// Tests whether this type is an arbitrary-precision number.
    #[inline]
    pub const fn is_big_number(self) -> bool {
        matches!(self, DataType::BigInteger | DataType::BigDecimal)
    }
}
