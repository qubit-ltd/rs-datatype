// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lazy scalar items bound to one source already charged to a session.

use super::AdmittedScalarItem;
use super::ConversionSession;
use super::DataConversionError;
use super::DataConverter;
use super::DataListConversionError;
use super::ScalarItems;
use crate::DataType;

/// Borrows a charged scalar source and lends one admitted item at a time.
///
/// Only [`ConversionSession::admit_scalar_string_source`] constructs this
/// capability. Items always borrow that exact source, retain their original
/// indices, and charge the session's item budget before target conversion.
/// Dropping the source leaves unconsumed items unscanned and uncharged; the
/// complete source's input bytes remain charged. No item strings are allocated.
///
/// # Type Parameters
///
/// * `'session` - Exclusive session borrow retained during iteration.
/// * `'policy` - Lifetime of the session's immutable policy and limits.
/// * `'source` - Lifetime shared by the source text and splitting policy.
///
/// # Examples
///
/// ```
/// use qubit_datatype::{ConversionLimits, ConversionPolicy, ConversionSession};
/// let policy = ConversionPolicy::env_friendly();
/// let limits = ConversionLimits::default();
/// let mut session = ConversionSession::new(&policy, &limits);
/// let mut source = session.admit_scalar_string_source("1,,3")?;
/// let item = source.next_item().expect("first item")?;
/// assert_eq!(item.source_index(), 0);
/// assert_eq!(item.convert::<u8>()?, 1);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// An adapter cannot create an item token without a charged source:
///
/// ```compile_fail
/// use qubit_datatype::{ConversionLimits, ConversionPolicy, ConversionSession, DataConverter};
/// let policy = ConversionPolicy::default();
/// let limits = ConversionLimits::default();
/// let mut session = ConversionSession::new(&policy, &limits);
/// let _ = DataConverter::from("uncharged");
/// compile_error!("item tokens are created only by AdmittedScalarSource");
/// ```
///
/// Outstanding items retain the exclusive session borrow:
///
/// ```compile_fail
/// use qubit_datatype::{ConversionLimits, ConversionPolicy, ConversionSession};
/// let policy = ConversionPolicy::env_friendly();
/// let limits = ConversionLimits::default();
/// let mut session = ConversionSession::new(&policy, &limits);
/// let mut source = session.admit_scalar_string_source("1,2").unwrap();
/// let first = source.next_item().unwrap().unwrap();
/// let second = source.next_item();
/// first.convert::<u8>().unwrap();
/// ```
#[must_use]
#[derive(Debug)]
pub struct AdmittedScalarSource<'session, 'policy, 'source> {
    /// Session that admitted the entire source's input bytes.
    session: &'session mut ConversionSession<'policy>,
    /// Allocation-free item slices retaining their pre-filter source indices.
    items: ScalarItems<'source>,
    /// Prevents continuing after an admission or splitting failure.
    exhausted: bool,
}

impl<'session, 'policy: 'source, 'source> AdmittedScalarSource<'session, 'policy, 'source> {
    /// Binds splitting to a source whose byte admission has already succeeded.
    #[inline]
    pub(crate) fn new(session: &'session mut ConversionSession<'policy>, source: &'source str) -> Self {
        let items = session
            .policy()
            .collection()
            .scalar_items(session.limits().collection(), source);
        Self {
            session,
            items,
            exhausted: false,
        }
    }

    /// Admits the next retained item, borrowing the session until it is
    /// consumed.
    ///
    /// # Returns
    ///
    /// `Some(Ok(item))` for the next admitted slice, `Some(Err(error))` for a
    /// splitting or item-budget failure, and `None` after exhaustion. Admission
    /// errors carry the original source index and String-to-String context;
    /// conversion errors use the target selected by the item consumer.
    ///
    /// # Errors
    ///
    /// Returns a list error for a rejected empty item, collection item limit,
    /// or exhausted session item budget. A failure exhausts this source.
    #[inline(always)]
    pub fn next_item(&mut self) -> Option<Result<AdmittedScalarItem<'_, 'policy, 'source>, DataListConversionError>> {
        self.next_item_for(DataType::String)
    }

    /// Admits an item with the target context known by built-in batch callers.
    pub(crate) fn next_item_for(
        &mut self,
        target: DataType,
    ) -> Option<Result<AdmittedScalarItem<'_, 'policy, 'source>, DataListConversionError>> {
        if self.exhausted {
            return None;
        }
        let item = match self.items.next()? {
            Ok(item) => item,
            Err(error) => {
                self.exhausted = true;
                return Some(Err(error.into_list_conversion_error(target)));
            }
        };
        if let Err(error) = self.session.consume_item() {
            self.exhausted = true;
            let error = DataConversionError::limit_exceeded(DataType::String, target, error);
            return Some(Err(DataListConversionError::new(item.source_index, error)));
        }
        Some(Ok(AdmittedScalarItem::new(
            self.session,
            item.source_index,
            DataConverter::from(item.value),
        )))
    }
}
