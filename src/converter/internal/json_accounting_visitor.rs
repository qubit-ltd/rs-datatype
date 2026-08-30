// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Internal visitor used to account an already materialized JSON tree.

use qubit_json::value::traverse::JsonTreeContext;
use qubit_json::value::traverse::JsonTreeVisitor;
use serde_json::Value;

/// Accepts every node after the traversal budget admits it.
pub(crate) struct JsonAccountingVisitor;

impl JsonTreeVisitor for JsonAccountingVisitor {
    type Error = std::convert::Infallible;

    #[inline(always)]
    fn enter(&mut self, _value: &Value, _context: JsonTreeContext<'_>) -> Result<(), Self::Error> {
        Ok(())
    }
}
