# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-datatype/coverage-badge.json)](https://qubit-ltd.github.io/rs-datatype/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

`qubit-datatype` helps Rust services turn configuration, protocol, and ingestion
values into typed values when their source type is known only at run time. It
provides stable type descriptors, exact mixed-numeric comparison, and
policy-driven conversion so precision loss, accepted text, and resource use
remain explicit. API-level details are available on [docs.rs](https://docs.rs/qubit-datatype).

## Installation

The default build has no optional dependencies and provides `DataType`,
`DataTypeOf`, `NumberRef`, and numeric comparison:

```toml
[dependencies]
qubit-datatype = "0.12"
```

Enable runtime conversion only with the rich type families your service needs:

```toml
[dependencies]
qubit-datatype = { version = "0.12", default-features = false, features = ["converter", "chrono"] }
```

| Feature | Adds |
| --- | --- |
| `duration` | Duration units, checked arithmetic, text codecs, formatting, and Serde adapters |
| `converter` | Runtime scalar, string, Duration, map, and batch conversion; includes `duration` |
| `chrono`, `big-integer`, `big-decimal`, `url`, `json` | Corresponding type mappings; combine with `converter` for conversion |
| `big-number` | Compatibility alias for `big-integer` and `big-decimal` |
| `all` | `converter` and every rich-type feature |

Rich-type features do not enable `converter` themselves.

## Quick start: typed deployment settings

For a service that receives `PORT=8080`, `ADMIN_ENABLED=yes`, and
`RETRY_DELAY=1500ms`, select the environment-oriented policy explicitly and
convert each value to its requested Rust type:

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

The result is observable: each value has its requested type, and no trimming,
Boolean alias, rounding, or Duration interpretation was selected implicitly.

## Why this crate exists

Runtime values cross trust and representation boundaries. Ad hoc parsing can
quietly round a number, accept an unintended spelling, or let a large value
consume unbounded work. This crate keeps three concerns separate:

- `DataType` and `DataTypeOf` provide a stable vocabulary for schemas and metadata.
- `NumberRef` compares unlike numeric representations without routing integers
  through `f64`; use `Exact` for validation, storage, deterministic ordering, and keys.
- `DataConverter`, `ConversionPolicy`, and `ConversionLimits` make conversion
  semantics and resource bounds explicit. Use `ConversionSession` only when
  several calls must share a cumulative operation budget.

## What it provides—and does not provide

The crate supports its documented source/target pairs and reports unsupported
pairs as `DataConversionErrorKind::Unsupported`. It is not a general-purpose
serialization framework and does not make arbitrary pairs convertible.

Strict conversion is the default: it preserves whitespace and rejects
fractional-to-integer truncation, numeric-to-float rounding, text-to-float
rounding, and inexact Duration output. `ConversionPolicy::lossy()` and
`ConversionPolicy::env_friendly()` are deliberate alternatives. Errors retain
stable kind and type context, but not source values, so the crate does not
expose secrets such as environment-variable contents.

`DataType`'s lowercase spellings, returned by `as_str`, accepted by Serde, and
listed in `DataType::ALL`, are a compatibility surface. Platform-sized `isize`
and `usize` are intentionally omitted because their representation is target-dependent.

## Learn more

Read the [English user guide](doc/user_guide.md) or the
[中文用户手册](doc/user_guide.zh_CN.md) for the full configuration scenario,
sessions, diagnostics, Duration text, and troubleshooting. See the
[design note](doc/design.md), [中文设计说明](doc/design.zh_CN.md), and
[API documentation](https://docs.rs/qubit-datatype) for further detail.

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
