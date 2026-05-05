/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0
 *
 ******************************************************************************/
use std::time::Duration;

use qubit_datatype::converter::DurationUnit;

/// Test suffix parsing and duration conversion for duration units.
#[test]
fn test_duration_unit_parses_suffixes_and_converts_values() {
    assert_eq!(
        DurationUnit::from_suffix("ms"),
        Some(DurationUnit::Milliseconds)
    );
    assert_eq!(
        DurationUnit::from_suffix("µs"),
        Some(DurationUnit::Microseconds)
    );
    assert_eq!(DurationUnit::from_suffix("unknown"), None);

    assert_eq!(
        DurationUnit::Minutes
            .duration_from_u64(2)
            .expect("minutes should convert"),
        Duration::from_secs(120),
    );
    assert_eq!(
        DurationUnit::Milliseconds.rounded_units(Duration::from_micros(1_500)),
        2,
    );
}
