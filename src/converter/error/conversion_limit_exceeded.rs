// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Structured facts for conversion resource failures.

use qubit_budget::BudgetError;

use super::conversion_limit::ConversionLimit;
use crate::converter::ConversionResource;

/// A conversion-specific wrapper retaining all facts from a budget failure.
#[must_use]
#[derive(Debug, Clone, thiserror::Error)]
#[error("{source}")]
pub struct ConversionLimitExceeded {
    /// Original point-limit or cumulative-budget failure.
    #[source]
    source: BudgetError<ConversionResource, usize>,
}

impl ConversionLimitExceeded {
    /// Wraps a budget failure produced by the conversion accounting layer.
    ///
    /// `InvalidRelease` is retained for completeness but is never generated
    /// by conversion sessions, which use non-releasable budgets.
    #[inline]
    pub fn from_budget_error(source: BudgetError<ConversionResource, usize>) -> Self {
        Self { source }
    }

    /// Returns the resource associated with the failure.
    #[inline(always)]
    pub const fn resource(&self) -> ConversionResource {
        match &self.source {
            BudgetError::LimitExceeded { resource, .. }
            | BudgetError::Insufficient { resource, .. }
            | BudgetError::InvalidRelease { resource, .. } => *resource,
        }
    }

    /// Returns the observed point measurement, if this was a point failure.
    #[inline(always)]
    pub const fn actual(&self) -> Option<usize> {
        self.source.actual()
    }

    /// Returns the point maximum, if this was a point failure.
    #[inline(always)]
    pub const fn maximum(&self) -> Option<usize> {
        self.source.maximum()
    }

    /// Returns the cumulative configured limit, if this was a budget failure.
    #[inline(always)]
    pub const fn limit(&self) -> Option<usize> {
        self.source.limit()
    }

    /// Returns the remaining cumulative capacity before failure.
    #[inline(always)]
    pub const fn remaining(&self) -> Option<usize> {
        self.source.remaining()
    }

    /// Returns the rejected cumulative request, if any.
    #[inline(always)]
    pub const fn requested(&self) -> Option<usize> {
        self.source.requested()
    }

    /// Returns cumulative usage before the failed request.
    #[inline(always)]
    pub const fn used(&self) -> Option<usize> {
        match (self.source.limit(), self.source.remaining()) {
            (Some(limit), Some(remaining)) => Some(limit - remaining),
            _ => None,
        }
    }

    /// Returns the underlying budget failure.
    #[inline(always)]
    pub const fn source_error(&self) -> &BudgetError<ConversionResource, usize> {
        &self.source
    }
}

impl From<ConversionLimit> for ConversionLimitExceeded {
    /// Converts the legacy point-limit description into structured facts.
    fn from(limit: ConversionLimit) -> Self {
        let (resource, maximum) = match limit {
            ConversionLimit::DurationTextBytes { maximum } => {
                (ConversionResource::DurationTextBytes, maximum)
            }
            ConversionLimit::NumericTextBytes { maximum } => {
                (ConversionResource::NumericTextBytes, maximum)
            }
            ConversionLimit::StructuredTextBytes { maximum } => {
                (ConversionResource::StructuredTextBytes, maximum)
            }
            ConversionLimit::BigIntegerDigits { maximum } => {
                (ConversionResource::BigIntegerDigits, maximum)
            }
            ConversionLimit::CollectionItems { maximum } => (ConversionResource::Items, maximum),
            ConversionLimit::InputBytes { maximum } => (ConversionResource::InputBytes, maximum),
            ConversionLimit::OutputBytes { maximum } => (ConversionResource::OutputBytes, maximum),
            ConversionLimit::StructuredDepth { maximum } => {
                (ConversionResource::StructuredDepth, maximum)
            }
            ConversionLimit::StructuredNodes { maximum } => {
                (ConversionResource::StructuredNodes, maximum)
            }
            ConversionLimit::SequenceItems { maximum } => {
                (ConversionResource::SequenceItems, maximum)
            }
            ConversionLimit::MapEntries { maximum } => (ConversionResource::MapEntries, maximum),
        };
        Self::from_budget_error(BudgetError::LimitExceeded {
            resource,
            actual: maximum.saturating_add(1),
            maximum,
        })
    }
}

impl PartialEq<ConversionLimit> for ConversionLimitExceeded {
    /// Compares structured facts with the legacy maximum-only view.
    fn eq(&self, other: &ConversionLimit) -> bool {
        let expected = ConversionLimitExceeded::from(*other);
        self.resource() == expected.resource() && self.maximum() == expected.maximum()
    }
}

impl PartialEq for ConversionLimitExceeded {
    /// Compares the resource dimension and configured capacity.
    fn eq(&self, other: &Self) -> bool {
        self.resource() == other.resource()
            && self.maximum() == other.maximum()
            && self.limit() == other.limit()
    }
}

impl Eq for ConversionLimitExceeded {}
