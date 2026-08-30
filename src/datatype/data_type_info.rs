// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Stable metadata for one runtime data type.

use super::DataType;
use super::DataTypeCategory;

/// Provides stable descriptive metadata for a [`DataType`].
///
/// This value does not expose a numeric ordinal. Protocols that require an
/// ordinal must define that ordering locally instead of coupling it to the
/// runtime type vocabulary.
///
/// # Examples
///
/// ```
/// use qubit_datatype::DataType;
/// use qubit_datatype::DataTypeCategory;
///
/// let info = DataType::Json.info();
/// assert_eq!(info.data_type(), DataType::Json);
/// assert_eq!(info.name(), "json");
/// assert_eq!(info.category(), DataTypeCategory::Structured);
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataTypeInfo {
    /// Runtime data type described by this metadata value.
    data_type: DataType,
}

impl DataTypeInfo {
    /// Creates metadata for one runtime data type.
    ///
    /// # Parameters
    ///
    /// * `data_type` - Runtime data type to describe.
    ///
    /// # Returns
    ///
    /// A lightweight metadata value for `data_type`.
    #[inline(always)]
    pub(crate) const fn new(data_type: DataType) -> Self {
        Self { data_type }
    }

    /// Returns the runtime data type described by this metadata value.
    ///
    /// # Returns
    ///
    /// The original [`DataType`].
    #[inline(always)]
    pub const fn data_type(self) -> DataType {
        self.data_type
    }

    /// Returns the stable lowercase protocol name.
    ///
    /// # Returns
    ///
    /// The same name returned by [`DataType::as_str`].
    #[must_use]
    #[inline(always)]
    pub const fn name(self) -> &'static str {
        self.data_type.as_str()
    }

    /// Returns the broad semantic category of this data type.
    ///
    /// # Returns
    ///
    /// A feature-independent category for the stable runtime type.
    #[inline]
    pub const fn category(self) -> DataTypeCategory {
        match self.data_type {
            DataType::Bool => DataTypeCategory::Boolean,
            DataType::Char => DataTypeCategory::Character,
            DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 | DataType::Int128 => {
                DataTypeCategory::SignedInteger
            }
            DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64 | DataType::UInt128 => {
                DataTypeCategory::UnsignedInteger
            }
            DataType::Float32 | DataType::Float64 => DataTypeCategory::FloatingPoint,
            DataType::String => DataTypeCategory::Text,
            DataType::Date | DataType::Time | DataType::DateTime | DataType::Instant => DataTypeCategory::Temporal,
            DataType::BigInteger | DataType::BigDecimal => DataTypeCategory::BigNumber,
            DataType::Duration => DataTypeCategory::Duration,
            DataType::Url => DataTypeCategory::Locator,
            DataType::StringMap | DataType::Json => DataTypeCategory::Structured,
        }
    }
}
