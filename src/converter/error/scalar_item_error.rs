// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Scalar item iteration error.

use crate::datatype::DataType;

use super::conversion_limit::ConversionLimit;
use super::data_conversion_error::DataConversionError;
use super::data_list_conversion_error::DataListConversionError;
use super::invalid_value_reason::InvalidValueReason;

/// Target-independent error discovered while iterating scalar items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ScalarItemError {
    /// An empty item was rejected by collection policy.
    #[error("blank scalar item rejected at source index {source_index}")]
    BlankRejected {
        /// Zero-based index before empty-item filtering.
        source_index: usize,
    },
    /// Retaining another item would exceed the configured limit.
    #[error(
        "scalar collection exceeds the {maximum}-item limit at source index {source_index}"
    )]
    ItemLimitExceeded {
        /// Zero-based index before empty-item filtering.
        source_index: usize,
        /// Configured maximum number of retained items.
        maximum: usize,
    },
}

impl ScalarItemError {
    /// Creates an empty-item rejection for the original source position.
    ///
    /// # Parameters
    ///
    /// * `source_index` - Zero-based position before empty-item filtering.
    ///
    /// # Returns
    ///
    /// An empty-item rejection containing the rejected source position.
    #[inline(always)]
    pub const fn new(source_index: usize) -> Self {
        Self::BlankRejected { source_index }
    }

    /// Creates an item-limit error for the original source position.
    ///
    /// # Parameters
    ///
    /// * `source_index` - Zero-based position before empty-item filtering.
    /// * `maximum` - Configured maximum number of retained items.
    ///
    /// # Returns
    ///
    /// An item-limit error containing the source position and limit.
    #[inline(always)]
    pub(crate) const fn item_limit_exceeded(
        source_index: usize,
        maximum: usize,
    ) -> Self {
        Self::ItemLimitExceeded {
            source_index,
            maximum,
        }
    }

    /// Returns the original source position before filtering.
    ///
    /// # Returns
    ///
    /// The original source position of the rejected or excess scalar item.
    #[must_use]
    #[inline(always)]
    pub const fn source_index(&self) -> usize {
        match *self {
            Self::BlankRejected { source_index }
            | Self::ItemLimitExceeded { source_index, .. } => source_index,
        }
    }

    /// Returns the retained-item limit carried by this error.
    ///
    /// # Returns
    ///
    /// `Some` with the configured limit for an item-limit error, or `None`
    /// for an empty-item rejection.
    #[must_use]
    #[inline(always)]
    pub const fn maximum_items(&self) -> Option<usize> {
        match *self {
            Self::BlankRejected { .. } => None,
            Self::ItemLimitExceeded { maximum, .. } => Some(maximum),
        }
    }

    /// Converts this item error into a target-aware conversion error.
    ///
    /// # Parameters
    ///
    /// * `to` - Requested target data type.
    ///
    /// # Returns
    ///
    /// A rejected-blank or resource-limit conversion error from
    /// [`DataType::String`] to `to`.
    #[inline(always)]
    pub const fn into_data_conversion_error(
        self,
        to: DataType,
    ) -> DataConversionError {
        match self {
            Self::BlankRejected { .. } => DataConversionError::invalid(
                DataType::String,
                to,
                InvalidValueReason::BlankRejected,
            ),
            Self::ItemLimitExceeded { maximum, .. } => {
                DataConversionError::limit_exceeded(
                    DataType::String,
                    to,
                    ConversionLimit::CollectionItems { maximum },
                )
            }
        }
    }

    /// Converts this item error into an indexed list conversion error.
    ///
    /// # Parameters
    ///
    /// * `to` - Requested target data type.
    ///
    /// # Returns
    ///
    /// A list conversion error preserving the original source index and
    /// carrying the target-aware rejected-blank or resource-limit error.
    #[inline(always)]
    pub const fn into_list_conversion_error(
        self,
        to: DataType,
    ) -> DataListConversionError {
        let source_index = self.source_index();
        DataListConversionError::new(
            source_index,
            self.into_data_conversion_error(to),
        )
    }
}
