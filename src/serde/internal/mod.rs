// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private visitor implementations for Serde adapters.

mod duration_millis_with_unit_visitor;
mod duration_with_unit_visitor;

pub(super) use duration_millis_with_unit_visitor::DurationMillisWithUnitVisitor;
pub(super) use duration_with_unit_visitor::DurationWithUnitVisitor;
