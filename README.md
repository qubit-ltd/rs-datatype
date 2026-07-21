# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-datatype/coverage-badge.json)](https://qubit-ltd.github.io/rs-datatype/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Runtime data type descriptors, exact cross-representation numeric comparison,
and policy-driven value conversion for Rust. API documentation is available on
[docs.rs](https://docs.rs/qubit-datatype).

## 1. What the crate provides

`qubit-datatype` has four complementary tool families:

- `DataType` and `DataTypeOf` provide stable runtime type descriptors.
- `NumberRef` compares unlike numeric representations without silently losing
  precision and exposes common numeric properties.
- The lightweight `duration` feature provides Duration units, checked unit
  arithmetic, configurable text parsing, and exact canonical formatting.
- The `converter` feature provides single-value, batch, and scalar-string
  collection conversion with explicit policies and structured errors.

Use the first family for schemas and metadata, the second for heterogeneous
numeric ordering, the third for Duration protocols that do not need runtime
value conversion, and the fourth at configuration, protocol, or ingestion
boundaries where source types are known only at runtime.

## 2. Installation and features

The minimum build has no optional dependencies:

```toml
[dependencies]
qubit-datatype = "0.8"
```

Enable conversion and only the rich families you need:

```toml
[dependencies]
qubit-datatype = { version = "0.8", default-features = false, features = ["converter", "chrono"] }
```

| Feature | Capability |
| --- | --- |
| `duration` | Duration units, checked arithmetic, text parsing, and exact formatting |
| `converter` | Scalar, string, Duration, map, batch, and option APIs; includes `duration` |
| `chrono` | Chrono type mappings; conversion support when combined with `converter` |
| `big-integer` | `BigInt` mappings; conversion support when combined with `converter` |
| `big-decimal` | `BigDecimal` mappings; conversion support when combined with `converter` |
| `big-number` | Compatibility alias for `big-integer` and `big-decimal` |
| `url` | `Url` mapping; conversion support when combined with `converter` |
| `json` | `serde_json::Value` mapping; JSON text and StringMap conversion when combined with `converter` |
| `all` | `converter` plus every rich-type feature |

Rich-type features do not enable `converter` by themselves.

`HashMap<String, String>` identity conversion is part of `converter`; parsing
or formatting that map as JSON additionally needs `json`.

## 3. Runtime type descriptors

`DataType` is a stable vocabulary with parsing, display, Serde, classification
methods, and the exhaustive `DataType::ALL` slice. `DataTypeOf` maps supported
Rust types to that vocabulary. Platform-sized `isize` and `usize` are omitted
because their representation is target-dependent.

The lowercase spellings returned by `DataType::as_str`, accepted by Serde, and
listed by `DataType::ALL` form a compatibility surface. Existing spellings are
not changed or reused for a different meaning in a non-breaking release.

```rust
use qubit_datatype::{DataType, DataTypeOf};

assert_eq!(u64::DATA_TYPE, DataType::UInt64);
assert!(DataType::Float64.is_numeric());
assert_eq!("INT32".parse::<DataType>(), Ok(DataType::Int32));
```

## 4. Numeric comparison

Wrap borrowed values in `NumberRef`, then choose a policy explicitly.
`Exact` compares mathematical values without routing integers through `f64`.
When a finite primitive float participates, `Approximate` attempts to project
both operands to finite `f64` values. Infinities are ordered separately, and if
either operand cannot be projected that way, comparison falls back to the exact
path. NaN is unordered, and signed zeros compare equal.

```rust
use std::cmp::Ordering;
use qubit_datatype::{NumberRef, NumericComparisonPolicy};

let integer = NumberRef::from((1_u64 << 53) + 1);
let float = NumberRef::from((1_u64 << 53) as f64);
assert_eq!(
    integer.compare(float, NumericComparisonPolicy::Exact),
    Some(Ordering::Greater),
);
```

Use `Exact` for validation, storage, and deterministic ordering. `Approximate`
is pair-dependent and non-transitive across mixed representations. It must not
be used to implement `Ord`, sort or group values, or construct keys for ordered
maps and sets. Use it only when IEEE-style pairwise comparison is the intended
domain rule.

## 5. Single-value conversion

`DataConverter` borrows or owns a runtime source. `to` uses strict defaults;
`to_with` accepts a `DataConversionOptions` profile.

```rust
use qubit_datatype::{DataConversionOptions, DataConverter};

assert_eq!(DataConverter::from("42").to::<u16>(), Ok(42));
assert!(DataConverter::from("3.9").to::<i32>().is_err());

let lossy = DataConversionOptions::lossy();
assert_eq!(DataConverter::from(" 3.9 ").to_with::<i32>(&lossy), Ok(3));
```

The strict profile independently rejects fractional-to-integer truncation,
numeric-to-float rounding, text-to-float rounding, and inexact Duration output.
The lossy profile permits finite float/decimal truncation toward zero,
nearest-even float rounding, and Duration half-up rounding. Decimal and
scientific-notation strings share the configured fractional-to-integer rule for
fixed-width integer and `BigInt` targets.

## 6. Conversion matrix

Rich targets require their matching feature.

| Source family | Supported targets |
| --- | --- |
| Concrete value | Its own type and `String` |
| `String` | Numeric, bool, char, Chrono, Duration, URL, JSON, StringMap |
| Bool / char | Primitive numeric targets |
| Integer / BigInt | Numeric, bool, Duration |
| Float / BigDecimal | Numeric |
| Duration | Fixed-width integers and `String` |
| StringMap | StringMap; JSON and `String` with `json` |
| JSON | `String` |

Inspect `DataConversionError::kind()` for the stable classification:
`DataConversionErrorKind::Unsupported` identifies unsupported pairs,
`Missing` identifies typed unset sources, `EmptyCollection` identifies an
empty collection where a first value was requested, `InvalidValue` identifies
malformed or policy-rejected values, and `LimitExceeded` identifies configured
resource caps. Errors retain type context but never retain the source value.

## 7. Options and input profiles

`DataConversionOptions` groups independent policies:

- `numeric`: fractional-to-integer, numeric-to-float, and text-to-float
  policies, plus resource limits.
- `string`: trimming and blank-string handling.
- `boolean`: accepted literals, case sensitivity, and numeric policy.
- `collection`: scalar splitting, delimiters, trimming, empty items, and the
  maximum number of retained items.
- `duration`: numeric input unit, suffixless input, accepted suffix set, output
  unit, suffix formatting, rounding, and source-text byte limit.

`strict()` is the default. `env_friendly()` trims strings, accepts common
Boolean literals, enables comma-separated scalar collections, and relaxes only
text-to-float conversion to nearest-even rounding. It does not enable
fractional-to-integer truncation or numeric-to-float rounding. Serde input uses
defaults for omitted fields and rejects unknown fields, so misspelled
configuration keys fail early.

```rust
use qubit_datatype::{DataConversionOptions, DataConverter};

let options = DataConversionOptions::env_friendly();
assert_eq!(DataConverter::from(" yes ").to_with::<bool>(&options), Ok(true));
```

Numeric resource caps are part of the options and remain enabled in every
profile. They apply after string normalization and before expensive parsing or
`BigInt` materialization:

```rust
use qubit_datatype::{
    DataConversionOptions, NumericConversionLimits, NumericConversionOptions,
};

let limits = NumericConversionLimits::default()
    .with_max_text_bytes(4096)
    .with_max_big_integer_digits(10_000);
let options = DataConversionOptions::strict().with_numeric_options(
    NumericConversionOptions::strict().with_limits(limits),
);
```

Boolean literal builders are fallible because true and false sets must remain
disjoint under the selected case-sensitivity rule.

## 8. Strings, duration, and rich formats

Strings are not trimmed by default. Blank values can be preserved, treated as
missing, or rejected. The default extended Duration suffix set accepts
`[0-9]+(ns|us|µs|μs|ms|s|m|h|d)?`; the ASCII suffix set excludes `µs` and
`μs`. Input and output units are configured independently. Exact Duration
output requires divisibility by the output unit; half-up rounding must be
selected explicitly.

With only the `duration` feature, `DurationTextOptions` selects suffixless and
ASCII-versus-extended suffix policies and bounds input to 1 MiB by default.
`parse_duration_text` enforces that byte limit before suffix processing,
performs checked parsing without implicit trimming, and `format_duration_exact`
selects the largest exact canonical unit.

Canonical rich strings are: `YYYY-MM-DD` for dates,
`HH:MM:SS[.fraction]` for times, RFC 3339 for instants, absolute URLs, standard
JSON, and JSON objects with unique keys and string values for StringMap. Date,
date-time, and instant formatting accepts only years `0000` through `9999`;
values outside that canonical four-digit domain are rejected.

## 9. Batch and scalar-string collections

`DataConverters` converts an existing iterator and reports the original
`source_index` on failure. `ScalarStringDataConverters` optionally splits one
string lazily; skipped empty items do not renumber later items.

Scalar-string collection conversion retains at most 65,536 items by default.
The limit is checked after trimming and empty-item filtering, so skipped items
do not consume the budget. A zero limit permits only an empty retained result;
the first additional retained item returns `LimitExceeded` with its original
source index. Use `CollectionConversionOptions::with_max_items` to select a
different bound.

```rust
use qubit_datatype::{DataConversionOptions, DataConverters, ScalarStringDataConverters};

let ports: Vec<u16> = DataConverters::from(vec!["8080", "8081"])
    .to_vec()
    .unwrap();
assert_eq!(ports, [8080, 8081]);

let options = DataConversionOptions::env_friendly();
let values: Vec<u16> = ScalarStringDataConverters::new("1, 2, 3")
    .to_vec_with(&options)
    .unwrap();
assert_eq!(values, [1, 2, 3]);
```

Use `to_vec` when all retained items are required and `to_first` when only the
first retained value matters.

## 10. Downstream target types

Downstream crates can implement `DataConversionTarget` for their own newtypes
and delegate to a built-in target.

```rust
use qubit_datatype::{DataConversionError, DataConversionOptions,
    DataConversionTarget, DataConverter, DataType, DataTypeOf};

#[derive(Debug, PartialEq, Eq)]
struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert_from(source: &DataConverter<'_>, options: &DataConversionOptions)
        -> Result<Self, DataConversionError>
    {
        u16::convert_from(source, options).map(Self)
    }
}

assert_eq!(DataConverter::from("8080").to::<Port>(), Ok(Port(8080)));
```

Prefer delegation so downstream types inherit the same normalization,
precision, error, and feature contracts as built-in targets.

## Testing

```bash
# Run tests with the default feature set
cargo test

# Run tests with all declared features
cargo test --all-features

# Project CI checks
./ci-check.sh

# Check code coverage
./coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./align-ci.sh` to format code and
`./ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-datatype](https://github.com/qubit-ltd/rs-datatype)
