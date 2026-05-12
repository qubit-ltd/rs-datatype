# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-datatype/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-datatype?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Runtime data type descriptors and conversion utilities for Rust.

## Overview

Qubit Datatype provides a shared `DataType` enum, compile-time type mapping
through `DataTypeOf`, and reusable conversion utilities for moving values between
supported Rust data types. It is intended for libraries that need runtime type
metadata, typed empty values, configuration parsing, value containers, or
structured conversion diagnostics.

## Design Goals

- **Focused scope**: model supported data types and conversions, not general data processing.
- **Shared type vocabulary**: use `DataType` consistently across value, config, and metadata crates.
- **Structured errors**: preserve source and target `DataType` in unsupported conversions.
- **Borrow-first conversion**: accept borrowed values where possible and own only when needed.
- **Composable options**: centralize string, boolean, and collection parsing policies.

## Features

### Data Type System

- **Runtime Type Enum**: `DataType` covers primitive Rust types and selected common ecosystem types.
- **Compile-time Type Mapping**: `DataTypeOf` maps Rust types to `DataType`.
- **Typed Empty Values**: `DataConverter::Empty(DataType)` preserves the intended missing value type.
- **Serde Support**: `DataType` serializes using stable lowercase names such as `int32` and `stringmap`.

### Data Conversion

- **Single-value Conversion**: `DataConverter` converts one source value to a target Rust type.
- **Batch Conversion**: `DataConverters` converts slices, vectors, or iterators in source order.
- **Scalar String Splitting**: `ScalarStringDataConverters` supports comma- or delimiter-separated inputs.
- **Conversion Options**: configure blank strings, boolean literals, trimming, delimiters, and empty items.
- **Detailed Errors**: unsupported conversions report `from` and `to` data types; invalid content carries context.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-datatype = "0.2.0"
```

## Quick Start

### Data Type Usage

```rust
use qubit_datatype::{DataType, DataTypeOf};

let data_type = DataType::Int32;
assert_eq!(data_type.as_str(), "int32");

assert_eq!(i32::DATA_TYPE, DataType::Int32);
assert_eq!(String::DATA_TYPE, DataType::String);
```

### Data Conversion

```rust
use std::time::Duration;

use qubit_datatype::{
    DataConversionResult,
    DataConverter,
    DataConverters,
    DataListConversionResult,
};

fn read_settings() -> DataConversionResult<(u16, bool, Duration)> {
    let port = DataConverter::from("8080").to::<u16>()?;
    let enabled = DataConverter::from("true").to::<bool>()?;
    let timeout = DataConverter::from("1500000000ns").to::<Duration>()?;

    Ok((port, enabled, timeout))
}

fn read_ports(values: &[String]) -> DataListConversionResult<Vec<u16>> {
    DataConverters::from(values).to_vec()
}
```

### Conversion Options

```rust
use qubit_datatype::{
    BlankStringPolicy,
    DataConversionOptions,
    DataConverter,
};

let options = DataConversionOptions::default()
    .with_blank_string_policy(BlankStringPolicy::AsNone);

let value = DataConverter::from(" 8080 ")
    .to_with::<u16>(&options)
    .expect("port should convert");

assert_eq!(value, 8080);
```

## Supported Data Types

The [`DataType`](https://docs.rs/qubit-datatype/latest/qubit_datatype/enum.DataType.html)
enum lists every variant. String forms use `as_str()`.

### Basic Types

- **Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
- **Platform integers**: `isize`, `usize`
- **Floats**: `f32`, `f64`
- **Other**: `bool`, `char`, `String`

### Date, Time, and Structured Types

- **Chrono**: `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `DateTime<Utc>`
- **Big numbers**: `BigInt`, `BigDecimal`
- **Duration**: `std::time::Duration`
- **String maps**: `HashMap<String, String>`
- **JSON**: `serde_json::Value`
- **URL**: `url::Url`

## API Reference

### Data Types

- [`DataType`](https://docs.rs/qubit-datatype/latest/qubit_datatype/enum.DataType.html) - runtime data type descriptor.
- [`DataTypeOf`](https://docs.rs/qubit-datatype/latest/qubit_datatype/trait.DataTypeOf.html) - compile-time type mapping trait.
- [`DataTypeParseError`](https://docs.rs/qubit-datatype/latest/qubit_datatype/struct.DataTypeParseError.html) - parse error for unknown type names.

### Conversion

- [`DataConverter`](https://docs.rs/qubit-datatype/latest/qubit_datatype/enum.DataConverter.html) - single-value conversion wrapper.
- [`DataConverters`](https://docs.rs/qubit-datatype/latest/qubit_datatype/struct.DataConverters.html) - batch conversion adapter.
- [`ScalarStringDataConverters`](https://docs.rs/qubit-datatype/latest/qubit_datatype/struct.ScalarStringDataConverters.html) - delimiter-aware string conversion adapter.
- [`DataConversionError`](https://docs.rs/qubit-datatype/latest/qubit_datatype/enum.DataConversionError.html) - conversion failure details.
- [`DataListConversionError`](https://docs.rs/qubit-datatype/latest/qubit_datatype/struct.DataListConversionError.html) - batch conversion error with failing index.
- [`DataConversionOptions`](https://docs.rs/qubit-datatype/latest/qubit_datatype/struct.DataConversionOptions.html) - combined conversion options.

## Testing & Code Coverage

This project maintains comprehensive test coverage for data type parsing,
mapping, conversion success paths, conversion errors, and boundary conditions.

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage report
./coverage.sh

# Generate text format report
./coverage.sh text

# Run CI checks (format, clippy, test, coverage, audit)
./ci-check.sh
```

### Coverage Metrics

See [COVERAGE.md](COVERAGE.md) for detailed coverage statistics.

## Dependencies

Runtime dependencies:

- `bigdecimal` for arbitrary precision decimal support.
- `chrono` for date and time types.
- `num-bigint` and `num-traits` for arbitrary precision integer support and numeric conversions.
- `serde` and `serde_json` for serialization and JSON values.
- `url` for URL type support.

## License

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

See [LICENSE](LICENSE) for the full license text.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

- Follow the Rust API guidelines.
- Maintain comprehensive test coverage.
- Document all public APIs with examples when they clarify usage.
- Run `./ci-check.sh` before submitting PRs.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

## Related Projects

More Rust libraries from Qubit are published under the [qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

---

Repository: [https://github.com/qubit-ltd/rs-datatype](https://github.com/qubit-ltd/rs-datatype)
