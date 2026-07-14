# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-datatype/coverage-badge.json)](https://qubit-ltd.github.io/rs-datatype/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Runtime data type descriptors and policy-driven conversion utilities for Rust.

## Installation

The default build contains the lightweight type vocabulary only:

```toml
[dependencies]
qubit-datatype = "0.3"
```

Enable individual external type mappings as needed, or enable the complete
conversion engine:

```toml
[dependencies]
qubit-datatype = { version = "0.3", features = ["converter"] }
```

## Features

| Feature | Adds |
| --- | --- |
| default | No optional dependencies |
| chrono | `DataTypeOf` for Chrono date/time types |
| big-number | `DataTypeOf` for `BigInt` and `BigDecimal` |
| url | `DataTypeOf` for `Url` |
| json | `DataTypeOf` for JSON and string maps |
| converter | The conversion API and all rich-type features |

## Type vocabulary

`DataType` provides 27 stable runtime type names, an exhaustive `ALL` array,
numeric classification methods, Serde support, and case-insensitive parsing.
`DataTypeOf` maps Rust types to their runtime descriptors.

```rust
use qubit_datatype::{DataType, DataTypeOf};

assert_eq!(i32::DATA_TYPE, DataType::Int32);
assert!(DataType::Int32.is_signed_integer());
assert_eq!(DataType::ALL.len(), 27);
```

## Conversion contract

With the `converter` feature, `DataConverter` converts a single value,
`DataConverters` converts an iterator, and `ScalarStringDataConverters`
lazily splits a scalar string while preserving original source indices.

The default `NumericConversionPolicy::Exact` rejects truncation, rounding, and
precision loss. Select `Lossy` explicitly to allow finite decimal/float to
integer truncation toward zero, integer-to-float IEEE rounding, and Duration
half-up rounding.

```rust
# #[cfg(feature = "converter")]
# {
use qubit_datatype::{
    DataConversionError, DataConversionErrorKind, DataConversionOptions,
    DataConverter, NumericConversionPolicy,
};

assert!(matches!(
    DataConverter::from("3.9").to::<i32>(),
    Err(DataConversionError::Invalid {
        kind: DataConversionErrorKind::PrecisionLoss,
        ..
    }),
));

let lossy = DataConversionOptions::default()
    .with_numeric_policy(NumericConversionPolicy::Lossy);
assert_eq!(DataConverter::from("3.9").to_with::<i32>(&lossy), Ok(3));
# }
```

### Conversion matrix

“Numeric” below includes primitive integers/floats and arbitrary-precision
numbers. Invalid values return `Invalid`; type pairs outside this matrix return
`Unsupported`; typed empty values return `Missing`.

| Source family | Supported targets |
| --- | --- |
| Any concrete source | Its own type; `String` |
| `String` | Numeric, bool, char, Chrono types, Duration, URL, JSON, StringMap |
| Bool / char | Primitive numeric targets |
| Integer / BigInt | Numeric targets, bool, Duration |
| Float / BigDecimal | Numeric targets |
| Duration | Integer targets and String |
| StringMap | JSON and String |
| JSON | String |

### Strings and booleans

Strings are not trimmed by default. Every string conversion calls
`StringConversionOptions::normalize` once; enable `trim` explicitly.
Blank strings can be preserved, treated as missing, or rejected.

Boolean text defaults to only `true` and `false` (ASCII
case-insensitive). Numeric 0/1 handling is controlled separately by
`BooleanNumericPolicy::ZeroOrOne`; `NonZero` and `Reject` are explicit
alternatives. Literal builders are fallible, so true/false sets cannot overlap.

```rust
# #[cfg(feature = "converter")]
# {
use qubit_datatype::{
    DataConversionOptions, DataConverter, StringConversionOptions,
};

assert!(DataConverter::from(" true ").to::<bool>().is_err());
let options = DataConversionOptions::default().with_string_options(
    StringConversionOptions::default().with_trim(true),
);
assert_eq!(DataConverter::from(" true ").to_with::<bool>(&options), Ok(true));
# }
```

### Duration

Duration text uses `[0-9]+(ns|us|ms|s|m|h|d)?`; a missing suffix uses the
configured unit. Whitespace, signs, decimals, and non-ASCII suffixes are
rejected. Large integer counts are decomposed into seconds and nanoseconds
before range checking.

Duration-to-integer and Duration-to-String follow the numeric policy: Exact
requires divisibility by the configured unit; Lossy rounds half-up.

### Rich text formats

- char: exactly one Unicode scalar value
- date: `YYYY-MM-DD`
- time: `HH:MM:SS[.fraction]`, 1–9 fractional digits
- local date-time: `YYYY-MM-DDTHH:MM:SS[.fraction]`
- UTC instant: RFC 3339 with `Z` or an offset
- BigInt: signed decimal integer
- BigDecimal: decimal with optional exponent
- URL: absolute URL
- JSON: any valid JSON value
- StringMap: JSON object with unique keys and string values

## Structured errors and collections

`DataConversionError` has exactly three categories: `Missing`,
`Unsupported`, and `Invalid { kind }`. Errors store source and target
`DataType` but never retain or display the original value.

List failures use `DataListConversionError::source_index`. Empty-item
`Skip` does not renumber later items, and `to_first` stops after the first
retained item without validating the tail.

## Development

```bash
cargo +1.94.0 test --no-default-features
cargo +1.94.0 test --all-features
./coverage.sh text
./align-ci.sh
./ci-check.sh
```

See [COVERAGE.md](COVERAGE.md) for coverage commands and thresholds.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).

## Author

**Haixing Hu** — Qubit Co. Ltd.
