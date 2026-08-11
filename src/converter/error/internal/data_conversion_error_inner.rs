// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Data Conversion Error Inner
//!
//! Defines the private variant representation of data conversion errors.

use qubit_budget::BudgetError;
use qubit_budget::QuantityConversionError;

use super::super::invalid_value_reason::InvalidValueReason;
use crate::converter::ConversionResource;
use crate::datatype::DataType;

/// Variant-specific details stored by a public conversion error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(in crate::converter::error) enum DataConversionErrorInner {
    /// The source has no concrete value.
    #[error("Missing value for conversion from {from} to {to}")]
    Missing {
        /// Declared source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
    },
    /// A first-value conversion was requested from an empty collection.
    #[error("Cannot convert the first value of an empty collection to {to}")]
    EmptyCollection {
        /// Requested target data type.
        to: DataType,
    },
    /// The source and target type pair is unsupported.
    #[error("Unsupported conversion from {from} to {to}")]
    Unsupported {
        /// Declared source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
    },
    /// The type pair is supported but the source value is invalid.
    #[error("Invalid conversion from {from} to {to}: {reason}")]
    InvalidValue {
        /// Declared source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
        /// Stable, value-free rejection reason.
        reason: InvalidValueReason,
    },
    /// The conversion would exceed a configured resource limit.
    #[error("Conversion limit exceeded from {from} to {to}: {source}")]
    LimitExceeded {
        /// Declared source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
        /// Complete point-limit or cumulative-budget facts.
        #[source]
        source: BudgetError<ConversionResource, u64>,
    },
    /// A native measurement cannot be represented by the configured quantity.
    #[error(
        "Conversion measurement cannot be represented from {from} to {to}: {source}"
    )]
    Quantity {
        /// Declared source data type.
        from: DataType,
        /// Requested target data type.
        to: DataType,
        /// Resource whose measurement could not be represented.
        resource: ConversionResource,
        /// Exact native quantity conversion failure.
        #[source]
        source: QuantityConversionError,
    },
}
