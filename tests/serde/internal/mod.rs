// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Internal holder types used by serde adapter tests.

mod borrowed_str_only_deserializer_tests;
mod duration_millis_holder_tests;
mod duration_millis_with_unit_holder_tests;
mod duration_with_unit_holder_tests;

pub(crate) use borrowed_str_only_deserializer_tests::BorrowedStrOnlyDeserializer;
pub(crate) use duration_millis_holder_tests::DurationMillisHolder;
pub(crate) use duration_millis_with_unit_holder_tests::DurationMillisWithUnitHolder;
pub(crate) use duration_with_unit_holder_tests::DurationWithUnitHolder;
