# Qubit Datatype User Guide

[中文用户手册](user_guide.zh_CN.md) · Applies to `qubit-datatype` 0.11.0

This guide is for Rust application developers who must turn configuration,
protocol, or ingestion values into typed values while retaining explicit rules
for precision, accepted text, and resource use. It covers the crate's current
public API; see the [README](../README.md) for a capability overview and
[docs.rs](https://docs.rs/qubit-datatype) for API-level details.

## Purpose and audience

Use this crate when the source type is available only at runtime, when a schema
needs a stable type vocabulary, or when unlike numeric representations must be
compared without an implicit `f64` conversion. It is not a general-purpose
serialization framework, and it does not make unsupported source/target pairs
convertible.

The default feature set provides `DataType`, `DataTypeOf`, `NumberRef`, and
numeric comparison. Enable `duration` for Duration text and Serde adapters;
enable `converter` for runtime conversion. Rich-type features such as `chrono`,
`big-integer`, `big-decimal`, `url`, and `json` add their mappings, but require
`converter` to participate in conversion.

## Conceptual model

`DataType` is the crate's stable runtime vocabulary, and `DataTypeOf` maps
supported Rust types to it. `DataConverter` holds a borrowed or owned runtime
source. A conversion combines that source with a `ConversionPolicy` (meaning)
and `ConversionLimits` (resource bounds). `ConversionSession` is optional: use
one only when several calls must share the same cumulative budget.

```text
runtime source ── DataConverter ── policy + limits ──> typed result
                                      │
                                      └─ optional shared ConversionSession
```

`DataConversionError` exposes a stable kind and type context, but deliberately
does not retain the source value. That makes it suitable to report for sources
such as environment variables without accidentally exposing their contents.

## Scenario: reading deployment settings

Assume a service receives `PORT=8080`, `ADMIN_ENABLED=yes`, and
`RETRY_DELAY=1500ms`. Success means it obtains a `u16`, a `bool`, and a
`Duration`; an invalid value must be rejected with a stable error classification
rather than being coerced invisibly.

### Installation and minimal configuration

Add the conversion and Duration APIs:

```toml
[dependencies]
qubit-datatype = { version = "0.11", default-features = false, features = ["converter"] }
```

`converter` includes `duration`. For only `DataType` and `NumberRef`, use
`qubit-datatype = "0.11"` without optional features.

### Core workflow

The strict policy is the default. It preserves whitespace, accepts only the
default Boolean literals, and rejects precision-losing numeric and Duration
conversions. An environment-style input normally benefits from the explicit
`env_friendly()` profile instead.

```rust
use std::time::Duration;

use qubit_datatype::{ConversionLimits, ConversionPolicy, DataConverter};

let policy = ConversionPolicy::env_friendly();
let limits = ConversionLimits::default();

let port = DataConverter::from("8080").to_with::<u16>(&policy, &limits)?;
let admin = DataConverter::from(" yes ").to_with::<bool>(&policy, &limits)?;
let retry = DataConverter::from("1500ms").to_with::<Duration>(&policy, &limits)?;

assert_eq!(port, 8080);
assert!(admin);
assert_eq!(retry, Duration::from_millis(1500));
# Ok::<(), qubit_datatype::DataConversionError>(())
```

The result is observable: the values have their requested Rust types, and no
conversion rule was selected implicitly. For repeated independent conversions,
calling `to_with` is sufficient because each call creates a fresh session.

## Advanced usage

### Share a budget across a logical operation

Build a `ConversionSession` when a whole request, rather than each field,
needs one cumulative item or byte budget. The next example permits two admitted
items and rejects the third.

```rust
use qubit_datatype::{
    ConversionLimits, ConversionOperationLimits, ConversionPolicy,
    ConversionSession, DataConverter,
};

let policy = ConversionPolicy::strict();
let limits = ConversionLimits::builder()
    .operation_limits(ConversionOperationLimits::builder().max_items(2).build())
    .build();
let mut session = ConversionSession::new(&policy, &limits);

assert_eq!(DataConverter::from("1").to_in::<u16>(&mut session)?, 1);
assert_eq!(DataConverter::from("2").to_in::<u16>(&mut session)?, 2);
assert!(DataConverter::from("3").to_in::<u16>(&mut session).is_err());
# Ok::<(), qubit_datatype::DataConversionError>(())
```

An admitted conversion that later fails still retains its item and input
charges. A failed admission does not consume an additional item. Use the
session's `*_used`, `*_remaining`, and `*_limit` accessors when you need to
observe the budget.

### Compare mixed numeric values

Choose `NumericComparisonPolicy::Exact` for validation, storage, deterministic
ordering, and keys. `Approximate` is only for pairwise IEEE-style comparison:
across mixed representations it can be non-transitive, so do not use it to
implement `Ord`, sort or group values, or create ordered-map/set keys.

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

NaN is unordered and signed zeroes compare equal.

### Parse standalone Duration text

When runtime conversion is unnecessary, enable `duration` and call the
lightweight text API. Default options reject a suffixless input and use strict
unit spellings.

```rust
use std::time::Duration;

use qubit_datatype::{parse_duration_text, DurationTextOptions};

let value = parse_duration_text("1500ms", &DurationTextOptions::default())?;
assert_eq!(value, Duration::from_millis(1500));
assert!(parse_duration_text("1500", &DurationTextOptions::default()).is_err());
# Ok::<(), qubit_datatype::DurationParseError>(())
```

Strict mode accepts `us`, `µs`, and `μs` for microseconds. It rejects the
lenient-only minute alias `m`; canonical output uses `µs` and `min`.

## Errors and diagnostics

Handle `DataConversionError` by its stable `DataConversionErrorKind`, not by
formatting an error message. The main kinds are `Missing`, `EmptyCollection`,
`Unsupported`, `InvalidValue`, and `LimitExceeded`; `Quantity` reports that a
native measurement could not fit the configured resource quantity.

```rust
use qubit_datatype::{DataConversionErrorKind, DataConverter};

let error = DataConverter::from("3.9").to::<i32>().unwrap_err();
assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
```

For an invalid value, inspect `reason()` when the caller needs a stable reason.
For a limit failure, inspect `resource()`, `configured_limit()`,
`observed_lower_bound()`, `used()`, `remaining()`, and `requested()` as
applicable. Batch APIs return `DataListConversionError`, which retains the
original `source_index` alongside the conversion error.

## Troubleshooting

| Symptom | Check | Resolution |
| --- | --- | --- |
| `Unsupported` | The active feature set and the source/target pair | Enable `converter` plus the matching rich-type feature, or choose a supported target. |
| `InvalidValue` for `" 42 "` | The strict profile does not trim | Use an explicit policy such as `ConversionPolicy::env_friendly()` only when trimming is intended. |
| Integer conversion rejects `"3.9"` | Strict numeric rules preserve precision | Keep the rejection or select `ConversionPolicy::lossy()` deliberately. |
| `LimitExceeded` | Per-value and session limits | Increase the relevant documented limit only after deciding the input is trustworthy. |
| Duration text is rejected | Suffix and parse mode | Supply a canonical suffix such as `ms`; use a configured policy only for a known external format. |

## Limitations and best practices

- Do not treat `DataType` spellings as disposable: lowercase strings exposed by
  `as_str`, accepted by Serde, and listed by `DataType::ALL` are a compatibility
  surface.
- `isize` and `usize` have no `DataTypeOf` mapping because their representation
  depends on the target platform.
- Keep semantic policy separate from resource limits. Policies decide what is
  acceptable; limits bound text, collections, structured input, and sessions.
- Errors retain type context but not source text. Log source values only under
  the calling application's own secret-handling policy.

## Further reading

- [README](../README.md)
- [中文用户手册](user_guide.zh_CN.md)
- [API documentation on docs.rs](https://docs.rs/qubit-datatype)
