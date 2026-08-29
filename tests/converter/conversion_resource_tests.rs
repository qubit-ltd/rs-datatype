// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion resource identity tests.

use qubit_datatype::ConversionResource;

#[test]
fn test_conversion_resource_output_bytes_is_distinct() {
    assert_ne!(ConversionResource::OutputBytes, ConversionResource::InputBytes);
}
