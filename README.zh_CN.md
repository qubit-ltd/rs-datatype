# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-datatype/coverage-badge.json)](https://qubit-ltd.github.io/rs-datatype/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

`qubit-datatype` 面向需要处理运行时类型值的 Rust 服务，可将配置、协议和数据接入的值转换为
强类型结果。它提供稳定的类型描述、跨数值表示的精确比较和策略化转换，让精度损失、可接受的
文本格式与资源消耗都保持显式可控。逐项 API 请查阅 [docs.rs](https://docs.rs/qubit-datatype)。

## 安装

默认构建不引入可选依赖，提供 `DataType`、`DataTypeOf`、`NumberRef` 和数值比较：

```toml
[dependencies]
qubit-datatype = "0.12"
```

需要运行时转换时，按服务实际使用的富类型启用 feature：

```toml
[dependencies]
qubit-datatype = { version = "0.12", default-features = false, features = ["converter", "chrono"] }
```

| Feature | 提供的能力 |
| --- | --- |
| `duration` | Duration 单位、带检查的运算、文本编解码、格式化和 Serde 适配器 |
| `converter` | 运行时标量、字符串、Duration、映射和批量转换；包含 `duration` |
| `chrono`、`big-integer`、`big-decimal`、`url`、`json` | 对应富类型的映射；需要转换时与 `converter` 组合启用 |
| `big-number` | `big-integer` 与 `big-decimal` 的兼容别名 |
| `all` | `converter` 与全部富类型 feature |

富类型 feature 本身不会启用 `converter`。

## 快速开始：读取部署配置

假设服务收到 `PORT=8080`、`ADMIN_ENABLED=yes` 和 `RETRY_DELAY=1500ms`，可以明确选择
适合环境变量的策略，再把每个值转换为目标 Rust 类型：

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

结果可直接验证：三个值都成为声明的目标类型，而且 trim、布尔字面量、舍入和 Duration
解释规则都不是隐式选定的。

## 为什么需要它

运行时值经常跨越信任边界和表示边界。临时拼装的解析逻辑可能悄悄舍入数值、接受意外拼写，
或让超大输入消耗无边界的资源。本 crate 将三类问题分开处理：

- `DataType` 与 `DataTypeOf` 为 schema 和元数据提供稳定的类型词汇。
- `NumberRef` 不经由 `f64` 就能比较不同数值表示；验证、存储、确定性排序和键应使用 `Exact`。
- `DataConverter`、`ConversionPolicy` 与 `ConversionLimits` 分别描述转换语义和资源边界；
  只有多次调用必须共享累计操作预算时才需要复用 `ConversionSession`。

## 能力边界

本 crate 支持已文档化的来源/目标组合，对不支持的组合返回
`DataConversionErrorKind::Unsupported`。它不是通用序列化框架，也不会让任意类型对自动变得可转换。

严格转换是默认行为：保留空白，并拒绝小数转整数截断、数值转浮点舍入、文本转浮点舍入和
不精确的 Duration 输出。只有领域规则确实需要时，才选择 `ConversionPolicy::lossy()` 或
`ConversionPolicy::env_friendly()`。错误会保留稳定分类与类型上下文，但不保存来源值，因此应用
可以报告失败而不会由本 crate 意外暴露环境变量等机密内容。

`DataType::as_str` 返回、Serde 接受并且 `DataType::ALL` 列出的全小写拼写属于兼容性接口。
`isize` 和 `usize` 依赖目标平台的表示，因而被有意排除在外。

## 延伸阅读

完整的配置场景、会话、诊断、Duration 文本和排障请阅读
[English user guide](doc/user_guide.md) 或[中文用户手册](doc/user_guide.zh_CN.md)。
更多设计细节请参阅 [English design note](doc/design.md)、[中文设计说明](doc/design.zh_CN.md)
和 [API 文档](https://docs.rs/qubit-datatype)。

## 测试

```bash
# 使用默认 feature 集运行测试
cargo test

# 使用项目声明的全部 feature 运行测试
cargo test --all-features

# 运行项目 CI 检查
./ci-check.sh

# 检查代码覆盖率
./coverage.sh
```

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

本项目基于 Apache License 2.0 授权。完整许可证文本请参阅
[LICENSE](LICENSE)。

## 贡献

欢迎贡献。请遵循 Rust API 指南，及时更新公共 API 文档与测试，并在提交
Pull Request 前运行 `./align-ci.sh`格式化代码，运行`./ci-check.sh`对齐CI要求。

## 作者

**Haixing Hu** - *Qubit Co. Ltd.*

仓库地址：[https://github.com/qubit-ltd/rs-datatype](https://github.com/qubit-ltd/rs-datatype)
