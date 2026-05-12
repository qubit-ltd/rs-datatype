# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-datatype/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-datatype?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的运行时数据类型描述与类型转换工具库。

## 概述

Qubit Datatype 提供统一的 `DataType` 枚举、通过 `DataTypeOf` 实现的编译期类型映射，以及用于在受支持 Rust 数据类型之间转换值的可复用工具。它适用于需要运行时类型元数据、typed empty、配置解析、值容器或结构化转换诊断的库。

## 设计目标

- **聚焦范围**：只建模受支持的数据类型和转换，不做泛泛的数据处理。
- **统一类型词汇**：在 value、config、metadata 等 crate 中一致使用 `DataType`。
- **结构化错误**：不支持的转换保留来源和目标 `DataType`。
- **优先借用**：尽可能接受借用值，只在必要时拥有数据。
- **可组合选项**：集中管理字符串、布尔值和集合解析策略。

## 特性

### 数据类型系统

- **运行时类型枚举**：`DataType` 覆盖基础 Rust 类型和部分常见生态类型。
- **编译期类型映射**：`DataTypeOf` 将 Rust 类型映射到 `DataType`。
- **Typed Empty**：`DataConverter::Empty(DataType)` 保留缺失值的预期类型。
- **Serde 支持**：`DataType` 使用稳定的小写名称序列化，例如 `int32` 和 `stringmap`。

### 数据转换

- **单值转换**：`DataConverter` 将一个源值转换为目标 Rust 类型。
- **批量转换**：`DataConverters` 按源顺序转换切片、向量或迭代器。
- **标量字符串拆分**：`ScalarStringDataConverters` 支持逗号或自定义分隔符输入。
- **转换选项**：配置空白字符串、布尔字面量、trim、分隔符和空元素策略。
- **详细错误**：不支持的转换报告 `from` 和 `to` 数据类型；无效内容保留上下文。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-datatype = "0.2.0"
```

## 快速开始

### 数据类型使用

```rust
use qubit_datatype::{DataType, DataTypeOf};

let data_type = DataType::Int32;
assert_eq!(data_type.as_str(), "int32");

assert_eq!(i32::DATA_TYPE, DataType::Int32);
assert_eq!(String::DATA_TYPE, DataType::String);
```

### 数据转换

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

### 转换选项

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

## 支持的数据类型

完整变体见 [`DataType`](https://docs.rs/qubit-datatype/latest/qubit_datatype/enum.DataType.html)。
字符串形式由 `as_str()` 给出。

### 基础类型

- **整数**：`i8`、`i16`、`i32`、`i64`、`i128`、`u8`、`u16`、`u32`、`u64`、`u128`
- **平台相关整数**：`isize`、`usize`
- **浮点数**：`f32`、`f64`
- **其他**：`bool`、`char`、`String`

### 日期、时间和结构化类型

- **Chrono**：`NaiveDate`、`NaiveTime`、`NaiveDateTime`、`DateTime<Utc>`
- **大数**：`BigInt`、`BigDecimal`
- **时长**：`std::time::Duration`
- **字符串映射**：`HashMap<String, String>`
- **JSON**：`serde_json::Value`
- **URL**：`url::Url`

## API 参考

### 数据类型

- [`DataType`](https://docs.rs/qubit-datatype/latest/qubit_datatype/enum.DataType.html) - 运行时数据类型描述。
- [`DataTypeOf`](https://docs.rs/qubit-datatype/latest/qubit_datatype/trait.DataTypeOf.html) - 编译期类型映射 trait。
- [`DataTypeParseError`](https://docs.rs/qubit-datatype/latest/qubit_datatype/struct.DataTypeParseError.html) - 未知类型名称的解析错误。

### 转换

- [`DataConverter`](https://docs.rs/qubit-datatype/latest/qubit_datatype/enum.DataConverter.html) - 单值转换包装器。
- [`DataConverters`](https://docs.rs/qubit-datatype/latest/qubit_datatype/struct.DataConverters.html) - 批量转换适配器。
- [`ScalarStringDataConverters`](https://docs.rs/qubit-datatype/latest/qubit_datatype/struct.ScalarStringDataConverters.html) - 支持分隔符的字符串转换适配器。
- [`DataConversionError`](https://docs.rs/qubit-datatype/latest/qubit_datatype/enum.DataConversionError.html) - 转换失败详情。
- [`DataListConversionError`](https://docs.rs/qubit-datatype/latest/qubit_datatype/struct.DataListConversionError.html) - 带失败索引的批量转换错误。
- [`DataConversionOptions`](https://docs.rs/qubit-datatype/latest/qubit_datatype/struct.DataConversionOptions.html) - 组合转换选项。

## 测试与代码覆盖率

本项目对数据类型解析、类型映射、转换成功路径、转换错误和边界条件保持全面测试覆盖。

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行覆盖率报告
./coverage.sh

# 生成文本格式报告
./coverage.sh text

# 运行 CI 检查（格式化、clippy、测试、覆盖率、审计）
./ci-check.sh
```

### 覆盖率指标

详细的覆盖率统计请参见 [COVERAGE.zh_CN.md](COVERAGE.zh_CN.md)。

## 依赖项

运行时依赖：

- `bigdecimal`：支持任意精度十进制数。
- `chrono`：支持日期和时间类型。
- `num-bigint` 和 `num-traits`：支持任意精度整数和数值转换。
- `serde` 和 `serde_json`：支持序列化和 JSON 值。
- `url`：支持 URL 类型。

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

根据 Apache 许可证 2.0 版（"许可证"）授权；
除非遵守许可证，否则您不得使用此文件。
您可以在以下位置获取许可证副本：

    http://www.apache.org/licenses/LICENSE-2.0

除非适用法律要求或书面同意，否则根据许可证分发的软件
按"原样"分发，不附带任何明示或暗示的担保或条件。
有关许可证下的特定语言管理权限和限制，请参阅许可证。

完整的许可证文本请参阅 [LICENSE](LICENSE)。

## 贡献

欢迎贡献！请随时提交 Pull Request。

### 开发指南

- 遵循 Rust API 指南。
- 保持全面的测试覆盖。
- 在文档能帮助理解时，为公共 API 提供示例。
- 提交 PR 前运行 `./ci-check.sh`。

## 作者

**胡海星** - *Qubit Co. Ltd.*

## 相关项目

Qubit 旗下的更多 Rust 库发布在 GitHub 组织 [qubit-ltd](https://github.com/qubit-ltd)。

---

仓库地址：[https://github.com/qubit-ltd/rs-datatype](https://github.com/qubit-ltd/rs-datatype)
