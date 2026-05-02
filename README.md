# Qubit Datatype

Runtime data type descriptors and conversion utilities for Qubit Rust projects.

`qubit-datatype` provides a shared `DataType` enum, compile-time type mapping
through `DataTypeOf`, and reusable conversion helpers such as `DataConverter`
and `DataConverters`.

## Features

- Runtime data type descriptors for supported scalar and structured values.
- Compile-time Rust type to `DataType` mapping with `DataTypeOf`.
- Single-value and batch conversion helpers.
- Conversion options for strings, booleans, and collection-like string inputs.
- Structured conversion errors carrying source and target `DataType` values.

## Installation

```toml
[dependencies]
qubit-datatype = "0.1.0"
```

## Quick Start

```rust
use qubit_datatype::{DataConverter, DataType, DataTypeOf};

assert_eq!(i32::DATA_TYPE, DataType::Int32);

let port: u16 = DataConverter::from("8080")
    .to()
    .expect("valid port should convert");
assert_eq!(port, 8080);
```

## Module Layout

- `datatype`: `DataType`, `DataTypeOf`, and parse errors.
- `converter`: `DataConverter`, `DataConverters`, conversion options, and
  conversion errors.

Top-level re-exports are provided for commonly used types.
