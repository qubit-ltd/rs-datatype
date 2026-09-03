// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Legacy metadata compatibility wrapper for one runtime data type.

use super::DataType;
use super::DataTypeCategory;

/// Provides stable descriptive metadata for a [`DataType`].
///
/// This value does not expose a numeric ordinal. Protocols that require an
/// ordinal must define that ordering locally instead of coupling it to the
/// runtime type vocabulary. New code should call [`DataType::as_str`] and
/// [`DataType::category`] directly.
///
/// # Examples
///
/// ```
/// use qubit_datatype::DataType;
/// use qubit_datatype::DataTypeCategory;
///
/// assert_eq!(DataType::Json.as_str(), "json");
/// assert_eq!(DataType::Json.category(), DataTypeCategory::Structured);
/// ```
#[deprecated(since = "0.12.0", note = "use DataType::as_str and DataType::category directly")]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataTypeInfo {
    /// Runtime data type described by this metadata value.
    data_type: DataType,
}

#[allow(deprecated)]
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
    #[must_use = "the data type is returned"]
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
    #[must_use = "the data type category is returned"]
    #[inline(always)]
    pub const fn category(self) -> DataTypeCategory {
        self.data_type.category()
    }
}
