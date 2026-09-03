// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private helpers shared by the conversion implementation.

mod conversion_budget;
mod conversion_capability_matrix;
mod conversion_string_sink;

pub(super) use conversion_budget::ConversionBudget;
pub(super) use conversion_capability_matrix::supports_conversion;
pub(super) use conversion_string_sink::ConversionStringSink;
