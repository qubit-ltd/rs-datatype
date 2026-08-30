# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-datatype/coverage-badge.json)](https://qubit-ltd.github.io/rs-datatype/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

For Rust services that accept configuration, protocol, or ingestion values whose
types are known only at runtime, `qubit-datatype` supplies stable type
descriptors, exact cross-representation numeric comparison, and policy-driven
conversion without silently discarding precision. API documentation is
available on [docs.rs](https://docs.rs/qubit-datatype).

## 1. What the crate provides

`qubit-datatype` has four complementary tool families:

- `DataType` and `DataTypeOf` provide stable runtime type descriptors.
- `NumberRef` compares unlike numeric representations without silently losing
  precision and exposes common numeric properties.
- The lightweight `duration` feature provides Duration units, checked unit
  arithmetic, configurable text parsing, exact canonical formatting, and
  Serde field adapters.
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
qubit-datatype = "0.11"
```

Enable conversion and only the rich families you need:

```toml
[dependencies]
qubit-datatype = { version = "0.11", default-features = false, features = ["converter", "chrono"] }
```

| Feature | Capability |
| --- | --- |
| `duration` | Duration units, checked arithmetic, text parsing, exact formatting, and Serde adapters |
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
methods, uniform `DataTypeInfo` metadata, and the exhaustive `DataType::ALL`
slice. `DataTypeOf` maps supported Rust types to that vocabulary.
Platform-sized `isize` and `usize` are omitted because their representation is
target-dependent.

The lowercase spellings returned by `DataType::as_str`, accepted by Serde, and
listed by `DataType::ALL` form a compatibility surface. Existing spellings are
not changed or reused for a different meaning in a non-breaking release.

```rust
use qubit_datatype::{ConversionCapabilities, DataType, DataTypeCategory, DataTypeOf};

assert_eq!(u64::DATA_TYPE, DataType::UInt64);
assert!(DataType::Float64.is_numeric());
assert_eq!(DataType::Float64.info().category(), DataTypeCategory::FloatingPoint);
assert_eq!("INT32".parse::<DataType>(), Ok(DataType::Int32));

let capabilities = ConversionCapabilities::current();
assert!(capabilities.supports(DataType::String, DataType::UInt64));
```

Capability queries describe conversion paths compiled by the active feature
set; they do not predict whether a particular value or policy will accept a
conversion.

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
`to_with` accepts an immutable `ConversionPolicy` and `ConversionLimits`.
`into_target` and `into_target_with` consume an owned source when it is no
longer needed, allowing compatible targets to reuse owned storage.

```rust
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::DataConverter;

assert_eq!(DataConverter::from("42").to::<u16>(), Ok(42));
assert!(DataConverter::from("3.9").to::<i32>().is_err());

let policy = ConversionPolicy::lossy();
let limits = ConversionLimits::default();
assert_eq!(
    DataConverter::from(" 3.9 ").to_with::<i32>(&policy, &limits),
    Ok(3),
);
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

## 7. Policies, limits, and sessions

`ConversionPolicy` groups semantic choices that do not consume resources:

- `numeric`: fractional-to-integer, numeric-to-float, and text-to-float
  policies.
- `string`: trimming and blank-string handling.
- `boolean`: accepted literals, case sensitivity, and numeric policy.
- `collection`: scalar splitting, delimiters, trimming, and empty-item policy.
- `duration`: numeric input unit, suffixless input policy, unit parse mode,
  output unit, suffix formatting, and rounding.

`ConversionLimits` separately groups numeric, collection, duration, structured,
and operation resource limits. `ConversionOperationLimits` configures
cumulative items, input bytes, String output bytes, structured
nodes, and structured payload bytes for one session.

`strict()` is the default. `env_friendly()` trims strings, accepts common
Boolean literals, enables comma-separated scalar collections, and relaxes only
text-to-float conversion to nearest-even rounding. It does not enable
fractional-to-integer truncation or numeric-to-float rounding. Serde input uses
defaults for omitted fields and rejects unknown fields, so misspelled
configuration keys fail early.

```rust
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::DataConverter;

let policy = ConversionPolicy::env_friendly();
let limits = ConversionLimits::default();
assert_eq!(
    DataConverter::from(" yes ").to_with::<bool>(&policy, &limits),
    Ok(true),
);
```

Numeric resource caps are part of `ConversionLimits` and remain enabled with
every policy. They apply after string normalization and before expensive
parsing or `BigInt` materialization:

```rust
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::NumericConversionLimits;
use qubit_datatype::NumericConversionPolicy;

let policy = ConversionPolicy::builder()
    .numeric_policy(NumericConversionPolicy::strict())
    .build();
let limits = ConversionLimits::builder()
    .numeric_limits(
        NumericConversionLimits::builder()
            .max_text_bytes(4096)
            .max_big_integer_digits(10_000)
            .build(),
    )
    .build();
```

Structured text limits are checked after string normalization and before URL,
JSON, or StringMap parsing:

```rust
use qubit_datatype::ConversionLimits;
use qubit_datatype::StructuredConversionLimits;

let limits = ConversionLimits::builder()
    .structured_limits(
        StructuredConversionLimits::builder()
            .max_text_bytes(64 * 1024)
            .build(),
    )
    .build();
```

Each `to_with`, `into_target_with`, or batch `*_with` call creates an independent
session. Internally, `ConversionOperationLimits` creates a private
`ConversionBudget`, and `ConversionSession` carries that mutable accounting
through nested work. Reuse one explicitly constructed session with `to_in`,
`into_target_in`, or `to_vec_in` only when limits must accumulate across calls:

```rust
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionOperationLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::ConversionSession;
use qubit_datatype::DataConverter;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let policy = ConversionPolicy::strict();
let operation = ConversionOperationLimits::builder().max_items(2).build();
let limits = ConversionLimits::builder()
    .operation_limits(operation)
    .build();
let mut session = ConversionSession::new(&policy, &limits);

assert_eq!(DataConverter::from("1").to_in::<u16>(&mut session)?, 1);
assert_eq!(DataConverter::from("2").to_in::<u16>(&mut session)?, 2);
assert!(DataConverter::from("3").to_in::<u16>(&mut session).is_err());
# Ok(())
# }
```

The rejected third request does not consume another item because its budget
check fails before admission. A conversion that is admitted and later fails
still retains its item and input charges; only transactional String output is
committed after the complete result succeeds. Convenience calls do not share
that state because each constructs a fresh session.

Limit errors expose stable resource, configured-limit, observation, used,
remaining, and requested facts directly through `DataConversionError`
accessors. The underlying accounting and JSON-library error types remain an
implementation detail. Output-byte budgets count the cumulative UTF-8 payload
of successful built-in `String` targets across a session, regardless of
whether the source is borrowed or owned; non-`String` targets do not consume
this output budget.

Boolean literal builders are fallible because true and false sets must remain
disjoint under the selected case-sensitivity rule.

## 8. Strings, duration, and rich formats

Strings are not trimmed by default and each conversion normalizes a String
source exactly once. Blank values can be preserved, treated as missing, or
rejected. The default Duration policy rejects suffixless text and
uses Strict parsing: `[0-9]+(ns|us|µs|μs|ms|s|min|h|d)`. Strict accepts the
ASCII `us`, micro-sign `µs`, and Greek-mu `μs` microsecond spellings. Lenient
parsing additionally accepts the non-canonical minute alias `m`:
`[0-9]+(ns|us|µs|μs|ms|s|min|m|h|d)`. `DurationConversionPolicy::env_friendly()`
uses Lenient parsing and interprets suffixless integers as milliseconds.
Input and output units are configured independently. Exact Duration output
requires divisibility by the output unit; half-up rounding must be selected
explicitly. Output always uses `µs` for microseconds and `min` for minutes.

With only the `duration` feature, `DurationTextOptions` selects the suffixless
policy and unit parse mode and bounds input to 256 bytes by default.
`parse_duration_text` enforces that byte limit before suffix processing,
performs checked parsing without implicit trimming, and `format_duration_exact`
selects the largest exact preferred unit.
Duration parse errors retain only stable categories and static canonical unit
symbols; they neither echo nor own arbitrary input suffixes.

The `duration` feature also exposes four `#[serde(with = "...")]` modules:
`duration_millis` stores a half-up rounded `u64` millisecond count,
`duration_millis_with_unit` stores half-up rounded `<integer>ms` text, and
`duration_millis_with_unit_exact` stores `<integer>ms` text only when no
submillisecond precision would be lost. `duration_with_unit` stores an exact
preferred-unit string.

```rust
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Timeout {
    #[serde(with = "qubit_datatype::serde::duration_with_unit")]
    value: Duration,
}
```

Canonical rich strings are: `YYYY-MM-DD` for dates,
`HH:MM:SS[.fraction]` for times, RFC 3339 for instants, absolute URLs, standard
JSON, and JSON objects with unique keys and string values for StringMap. Date,
date-time, and instant formatting accepts only years `0000` through `9999`;
values outside that canonical four-digit domain are rejected.
StringMap JSON objects emit members in lexicographic key order, including when
the dependency graph enables `serde_json`'s `preserve_order` feature.
Date and time fields are zero-padded ASCII digits with separators in the exact
positions shown; internal whitespace, signed years, alternate separators, and
Unicode digits are rejected before Chrono performs range validation.

## 9. Batch and scalar-string collections

`DataConverters` converts an existing iterator and reports the original
`source_index` on failure. `ScalarStringDataConverters` optionally splits one
string lazily; skipped empty items do not renumber later items.

Scalar-string collection conversion retains at most 65,536 items by default.
The limit is checked after trimming and empty-item filtering, so skipped items
do not consume the budget. A zero limit permits only an empty retained result;
the first additional retained item returns `LimitExceeded` with its original
source index. Use
`CollectionConversionLimits::builder().max_items(limit).build()` to select a
different bound. This per-string splitting limit is separate from the session
budget, which also applies to ordinary iterator batches.

```rust
use qubit_datatype::ConversionLimits;
use qubit_datatype::ConversionPolicy;
use qubit_datatype::DataConverters;
use qubit_datatype::ScalarStringDataConverters;

let ports: Vec<u16> = DataConverters::from(vec!["8080", "8081"])
    .to_vec()
    .unwrap();
assert_eq!(ports, [8080, 8081]);

let policy = ConversionPolicy::env_friendly();
let limits = ConversionLimits::default();
let values: Vec<u16> = ScalarStringDataConverters::new("1, 2, 3")
    .to_vec_with(&policy, &limits)
    .unwrap();
assert_eq!(values, [1, 2, 3]);
```

Use `to_vec` when all retained items are required and `to_first` when only the
first retained value matters.

## 10. Downstream target types

Downstream crates can implement `DataConversionTarget` for their own newtypes
and delegate through the caller's `ConversionSession`. Delegation shares the
active item, input, output, and structured budgets without charging the nested
conversion as a second top-level item.

The outer `to_in`/`into_target_in` call has already charged one item and its
source input. Custom targets should use `session.delegate` or
`session.delegate_owned` for nested conversions. Extensions with their own
work units can use the domain-level admission, JSON, and transactional String
methods on `ConversionSession`; these return `DataConversionError` rather than
backend-specific errors.

For scalar collection adapters, `admit_scalar_item` returns a one-use proof
that owns both the exact source and its session borrow. Consume that proof with
`convert` or `convert_with_session`; it cannot be paired with another source or
reused.

```rust
use qubit_datatype::{ConversionSession, DataConversionError,
    DataConversionTarget, DataConverter, DataType, DataTypeOf};

#[derive(Debug, PartialEq, Eq)]
struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert_from(
        source: &DataConverter<'_>,
        session: &mut ConversionSession<'_>,
    )
        -> Result<Self, DataConversionError>
    {
        session.delegate::<u16>(source).map(Self)
    }
}

assert_eq!(DataConverter::from("8080").to::<Port>(), Ok(Port(8080)));
```

Prefer delegation so downstream types inherit the same normalization,
precision, error, and feature contracts as built-in targets.

## Learn more

The [English user guide](doc/user_guide.md) walks through a configuration-input
scenario, policy selection, resource limits, diagnostics, and Duration text.
Read the [中文用户手册](doc/user_guide.zh_CN.md) for the same material in
Simplified Chinese.

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
