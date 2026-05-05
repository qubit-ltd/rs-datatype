/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0
 *
 ******************************************************************************/
use qubit_datatype::converter::{
    DurationConversionOptions,
    DurationUnit,
};

/// Test duration option defaults and builder methods.
#[test]
fn test_duration_conversion_options_builders_update_fields() {
    let defaults = DurationConversionOptions::default();
    assert_eq!(defaults.unit, DurationUnit::Milliseconds);
    assert!(defaults.append_unit_suffix);

    let options = defaults
        .with_unit(DurationUnit::Seconds)
        .with_append_unit_suffix(false);

    assert_eq!(options.unit, DurationUnit::Seconds);
    assert!(!options.append_unit_suffix);
}
