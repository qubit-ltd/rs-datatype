# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的运行时类型描述、跨表示精确数值比较和策略驱动值转换工具。

## 文档

- [中文用户手册](doc/user_guide.zh_CN.md)
- [English user guide](doc/user_guide.md)
- [API 文档](https://docs.rs/qubit-datatype)

## 快速开始

```toml
[dependencies]
qubit-datatype = { version = "0.7", features = ["converter"] }
```

```rust
use qubit_datatype::{DataConverter, DataType, DataTypeOf};

assert_eq!(i32::DATA_TYPE, DataType::Int32);
assert_eq!(DataConverter::from("8080").to::<u16>(), Ok(8080));
```

默认 feature 只包含轻量类型词汇。需要转换 API 时启用 `converter`，再按需组合
`chrono`、`big-number`、`url` 或 `json`。完整 feature 表、转换矩阵、配置策略、
错误模型、集合工具和扩展示例请阅读用户手册。

## 许可证

Apache License 2.0，详见 [LICENSE](LICENSE)。
