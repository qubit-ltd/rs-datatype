// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Feature-aware discovery of available conversion paths.

use super::internal::supports_conversion;
use crate::DataType;

/// Describes conversion paths compiled into the current crate build.
///
/// Capability checks describe whether an implementation path exists. They do
/// not guarantee that a concrete value satisfies the active conversion policy
/// or lies within the target value range.
///
/// # Examples
///
/// ```
/// use qubit_datatype::ConversionCapabilities;
/// use qubit_datatype::DataType;
///
/// let capabilities = ConversionCapabilities::current();
/// assert!(capabilities.supports(DataType::String, DataType::Int32));
/// assert!(!capabilities.supports(DataType::StringMap, DataType::Bool));
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionCapabilities {
    /// Prevents downstream construction of build states other than current.
    private: (),
}

impl ConversionCapabilities {
    /// Returns the capabilities compiled into the current crate build.
    ///
    /// # Returns
    ///
    /// A zero-sized value that queries the active Cargo feature combination.
    #[must_use = "the active conversion capabilities are returned"]
    #[inline(always)]
    pub const fn current() -> Self {
        Self { private: () }
    }

    /// Reports whether a concrete Rust mapping and converter implementation
    /// exist for `data_type` in the current build.
    ///
    /// # Parameters
    ///
    /// * `data_type` - Stable runtime type to inspect.
    ///
    /// # Returns
    ///
    /// `true` when the current feature combination includes the type.
    #[must_use]
    #[inline]
    pub const fn is_available(self, data_type: DataType) -> bool {
        match data_type {
            DataType::Date | DataType::Time | DataType::DateTime | DataType::Instant => {
                cfg!(feature = "chrono")
            }
            DataType::BigInteger => cfg!(feature = "big-integer"),
            DataType::BigDecimal => cfg!(feature = "big-decimal"),
            DataType::Url => cfg!(feature = "url"),
            DataType::Json => cfg!(feature = "json"),
            DataType::Bool
            | DataType::Char
            | DataType::Int8
            | DataType::Int16
            | DataType::Int32
            | DataType::Int64
            | DataType::Int128
            | DataType::UInt8
            | DataType::UInt16
            | DataType::UInt32
            | DataType::UInt64
            | DataType::UInt128
            | DataType::Float32
            | DataType::Float64
            | DataType::String
            | DataType::Duration
            | DataType::StringMap => true,
        }
    }

    /// Reports whether the current build contains a conversion path between
    /// two concrete runtime types.
    ///
    /// This check is independent of [`crate::ConversionPolicy`] and source
    /// values. A supported path can still reject malformed, out-of-range, or
    /// policy-disallowed input.
    ///
    /// # Parameters
    ///
    /// * `from` - Concrete source data type.
    /// * `to` - Requested target data type.
    ///
    /// # Returns
    ///
    /// `true` when both types are available and a conversion implementation
    /// exists.
    #[must_use]
    pub fn supports(self, from: DataType, to: DataType) -> bool {
        if !self.is_available(from) || !self.is_available(to) {
            return false;
        }
        supports_conversion(from, to)
    }

    /// Iterates over targets supported for one source in stable declaration
    /// order.
    ///
    /// The iterator borrows only the static [`DataType::ALL`] catalog and does
    /// not allocate.
    ///
    /// # Parameters
    ///
    /// * `from` - Concrete source data type.
    ///
    /// # Returns
    ///
    /// Every supported target in [`DataType::ALL`] order, or an empty iterator
    /// when the source type is unavailable.
    #[must_use = "the supported target iterator must be consumed"]
    #[inline]
    pub fn supported_targets(self, from: DataType) -> impl Iterator<Item = DataType> {
        DataType::ALL.iter().copied().filter(move |to| self.supports(from, *to))
    }
}
