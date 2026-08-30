// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private helpers shared by the conversion implementation.

mod conversion_budget;
mod conversion_string_sink;
#[cfg(feature = "json")]
mod json_accounting_visitor;

pub(super) use conversion_budget::ConversionBudget;
pub(super) use conversion_string_sink::ConversionStringSink;
#[cfg(feature = "json")]
pub(super) use json_accounting_visitor::JsonAccountingVisitor;
