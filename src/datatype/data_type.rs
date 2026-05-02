/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Data Type Definitions (Language Layer)
//!
//! Provides cross-module reusable common data type enum `DataType` and type mapping `DataTypeOf`.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::str::FromStr;

use super::data_type_parse_error::DataTypeParseError;

use serde::{Deserialize, Serialize};

/// Universal data type enumeration for cross-module type representation
///
/// Defines all basic data types and composite types supported by the system.
/// This enum provides a unified way to represent and work with different data types
/// across various modules and components.
///
/// `DataType` serves as a bridge between Rust's type system and runtime type
/// information, enabling dynamic type handling, serialization, validation,
/// and other type-aware operations.
///
/// # Features
///
/// - **Comprehensive Coverage**: Supports all basic Rust types plus common third-party types
/// - **String Representation**: Each variant has a consistent string representation
/// - **Serialization Support**: Implements `Serialize` and `Deserialize` for JSON/YAML support
/// - **Type Mapping**: Works with `DataTypeOf` trait for compile-time type mapping
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
///
/// # Author
///
/// Haixing Hu
///
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
    /// Platform-dependent signed integer (isize)
    IntSize,
    /// Platform-dependent unsigned integer (usize)
    UIntSize,
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

impl FromStr for DataType {
    type Err = DataTypeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "bool" => Ok(DataType::Bool),
            "char" => Ok(DataType::Char),
            "int8" => Ok(DataType::Int8),
            "int16" => Ok(DataType::Int16),
            "int32" => Ok(DataType::Int32),
            "int64" => Ok(DataType::Int64),
            "int128" => Ok(DataType::Int128),
            "uint8" => Ok(DataType::UInt8),
            "uint16" => Ok(DataType::UInt16),
            "uint32" => Ok(DataType::UInt32),
            "uint64" => Ok(DataType::UInt64),
            "uint128" => Ok(DataType::UInt128),
            "float32" => Ok(DataType::Float32),
            "float64" => Ok(DataType::Float64),
            "string" => Ok(DataType::String),
            "date" => Ok(DataType::Date),
            "time" => Ok(DataType::Time),
            "datetime" => Ok(DataType::DateTime),
            "instant" => Ok(DataType::Instant),
            "biginteger" => Ok(DataType::BigInteger),
            "bigdecimal" => Ok(DataType::BigDecimal),
            "intsize" => Ok(DataType::IntSize),
            "uintsize" => Ok(DataType::UIntSize),
            "duration" => Ok(DataType::Duration),
            "url" => Ok(DataType::Url),
            "stringmap" => Ok(DataType::StringMap),
            "json" => Ok(DataType::Json),
            _ => Err(DataTypeParseError::new(value)),
        }
    }
}

impl DataType {
    // Keep explicit conversion helper for existing call-sites and API parity with
    // current serialized/configuration text values.
    /// Get the string representation of the data type.
    ///
    /// # Returns
    ///
    /// Returns the name string of the data type.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            DataType::Bool => "bool",
            DataType::Char => "char",
            DataType::Int8 => "int8",
            DataType::Int16 => "int16",
            DataType::Int32 => "int32",
            DataType::Int64 => "int64",
            DataType::Int128 => "int128",
            DataType::UInt8 => "uint8",
            DataType::UInt16 => "uint16",
            DataType::UInt32 => "uint32",
            DataType::UInt64 => "uint64",
            DataType::UInt128 => "uint128",
            DataType::Float32 => "float32",
            DataType::Float64 => "float64",
            DataType::String => "string",
            DataType::Date => "date",
            DataType::Time => "time",
            DataType::DateTime => "datetime",
            DataType::Instant => "instant",
            DataType::BigInteger => "biginteger",
            DataType::BigDecimal => "bigdecimal",
            DataType::IntSize => "intsize",
            DataType::UIntSize => "uintsize",
            DataType::Duration => "duration",
            DataType::Url => "url",
            DataType::StringMap => "stringmap",
            DataType::Json => "json",
        }
    }
}
