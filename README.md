# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文](https://img.shields.io/badge/文档-中文-blue.svg)](README.zh_CN.md)

Runtime data type descriptors, exact cross-representation numeric comparison,
and policy-driven value conversion for Rust.

## Documentation

- [English user guide](doc/user_guide.md)
- [中文用户手册](doc/user_guide.zh_CN.md)
- [API documentation](https://docs.rs/qubit-datatype)

## Quick start

```toml
[dependencies]
qubit-datatype = { version = "0.7", features = ["converter"] }
```

```rust
use qubit_datatype::{DataConverter, DataType, DataTypeOf};

assert_eq!(i32::DATA_TYPE, DataType::Int32);
assert_eq!(DataConverter::from("8080").to::<u16>(), Ok(8080));
```

The default feature set contains only the lightweight type vocabulary. Enable
`converter` for conversion APIs, and add `chrono`, `big-number`, `url`, or
`json` only for the rich types you use. See the user guide for the complete
feature table, conversion matrix, options, errors, collection tools, and
extension examples.

## License

Apache License 2.0. See [LICENSE](LICENSE).
