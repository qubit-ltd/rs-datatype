# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-datatype/coverage-badge.json)](https://qubit-ltd.github.io/rs-datatype/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的运行时类型描述、跨表示精确数值比较和策略驱动值转换工具。API 文档
发布于 [docs.rs](https://docs.rs/qubit-datatype)。

## 1. 工具概览

`qubit-datatype` 提供四组互补工具：

- `DataType`、`DataTypeOf`：稳定的运行时类型描述，适合 schema 和元数据。
- `NumericValueRef`、`compare_numeric`：比较不同数值表示，避免隐式精度损失。
- 轻量 `duration` feature：提供 Duration 单位、带溢出检查的单位运算、可配置文本
  解析和精确规范化格式。
- `converter` feature：按显式策略执行单值、批量和标量字符串集合转换，并返回
  结构化错误，适合配置、协议和数据接入边界。

## 2. 安装与 features

最小构建不启用可选依赖：

```toml
[dependencies]
qubit-datatype = "0.7"
```

按需启用转换器和富类型：

```toml
[dependencies]
qubit-datatype = { version = "0.7", features = ["converter", "chrono"] }
```

| Feature | 能力 |
| --- | --- |
| `duration` | Duration 单位、带检查运算、文本解析和精确格式化 |
| `converter` | 标量、字符串、Duration、StringMap、批量和配置 API；包含 `duration` |
| `chrono` | Chrono 类型映射及转换 |
| `big-integer` | `BigInt` 映射及转换 |
| `big-decimal` | `BigDecimal` 映射及转换 |
| `big-number` | `big-integer` 与 `big-decimal` 的兼容别名 |
| `url` | `Url` 映射及转换 |
| `json` | `serde_json::Value`、JSON 文本和 StringMap JSON 转换 |
| `all` | `converter` 与全部富类型 feature |

`HashMap<String, String>` 的恒等转换只需要 `converter`；把它解析或格式化为 JSON
还需要 `json`。

## 3. 运行时类型描述

`DataType` 提供稳定类型词汇、解析、显示、Serde、分类方法和完整的
`DataType::ALL`。`DataTypeOf` 把 Rust 类型映射为该词汇。平台相关的 `isize`、
`usize` 不提供映射，以免数据表示随目标平台变化。

```rust
use qubit_datatype::{DataType, DataTypeOf};

assert_eq!(u64::DATA_TYPE, DataType::UInt64);
assert!(DataType::Float64.is_numeric());
assert_eq!("INT32".parse::<DataType>(), Ok(DataType::Int32));
```

## 4. 数值比较

先把借用值包装成 `NumericValueRef`，再显式选择策略。`Exact` 比较数学值，不让整数
经过 `f64`。有限原生浮点数参与时，`Approximate` 会尝试把两个操作数投影为有限
`f64`；无穷值单独排序，任一操作数无法完成这种投影时回退到精确比较。NaN 无序，
正负零相等。

```rust
use std::cmp::Ordering;
use qubit_datatype::{compare_numeric, NumericComparisonPolicy, NumericValueRef};

let integer = NumericValueRef::UInt64((1_u64 << 53) + 1);
let float = NumericValueRef::Float64((1_u64 << 53) as f64);
assert_eq!(
    compare_numeric(integer, float, NumericComparisonPolicy::Exact),
    Some(Ordering::Greater),
);
```

校验、存储和确定性排序应使用 `Exact`。`Approximate` 的投影取决于当前操作数对，
在混合表示之间不满足传递性，因此不得用于实现 `Ord`、排序、分组，也不得用于有序
映射或有序集合的键；只有领域规则明确要求 IEEE 风格的成对近似比较时才使用它。

## 5. 单值转换

`DataConverter` 借用或持有一个运行时来源值。`to` 使用严格默认配置，`to_with`
接收显式的 `DataConversionOptions`。

```rust
use qubit_datatype::{DataConversionOptions, DataConverter};

assert_eq!(DataConverter::from("42").to::<u16>(), Ok(42));
assert!(DataConverter::from("3.9").to::<i32>().is_err());

let lossy = DataConversionOptions::lossy();
assert_eq!(DataConverter::from(" 3.9 ").to_with::<i32>(&lossy), Ok(3));
```

严格数值策略拒绝截断、舍入和精度损失。有损模式允许有限浮点/十进制向零截断、
整数到浮点的 IEEE 舍入以及 Duration 的 half-up 舍入。十进制与科学计数法字符串
转换到定长整数或 `BigInt` 时遵循同一数值策略。

## 6. 转换矩阵

富类型目标需要对应 feature。

| 来源族 | 支持的目标 |
| --- | --- |
| 具体值 | 自身类型和 `String` |
| `String` | 数值、bool、char、Chrono、Duration、URL、JSON、StringMap |
| Bool / char | primitive 数值 |
| 整数 / BigInt | 数值、bool、Duration |
| 浮点 / BigDecimal | 数值 |
| Duration | 定长整数和 `String` |
| StringMap | StringMap；启用 `json` 后支持 JSON 和 `String` |
| JSON | `String` |

表外组合返回 `DataConversionError::Unsupported`，typed empty 返回 `Missing`，
格式或策略不合法返回 `InvalidValue`。错误只保留类型上下文，不保留原始值。

## 7. 配置与输入 profile

`DataConversionOptions` 包含相互独立的策略：

- `numeric_policy`：精确或有损数值转换。
- `string`：trim 和空白字符串处理。
- `boolean`：文字集合、大小写和数值布尔策略。
- `collection`：标量拆分、分隔符、trim 和空元素处理。
- `duration`：数值输入单位、无后缀输入、输出单位和后缀。

`strict()` 是默认值。`env_friendly()` 会 trim 字符串、接受常见布尔文字，并开启
逗号分隔的标量集合。Serde 对省略字段使用默认值，同时拒绝未知字段，因此配置键
拼写错误会立即失败。

```rust
use qubit_datatype::{DataConversionOptions, DataConverter};

let options = DataConversionOptions::env_friendly();
assert_eq!(DataConverter::from(" yes ").to_with::<bool>(&options), Ok(true));
```

布尔文字 builder 返回 `Result`，保证在选定大小写规则下 true/false 集合互不重叠。

## 8. 字符串、Duration 与富格式

默认不 trim 字符串。空白值可保留、视为缺失或拒绝。Duration 输入格式为
`[0-9]+(ns|us|µs|μs|ms|s|m|h|d)?`，输入和输出单位分别配置；精确输出要求按
输出单位整除。

仅启用 `duration` feature 时，`DurationTextOptions` 可选择无后缀策略以及 ASCII
或扩展后缀集合；`parse_duration_text` 在不隐式 trim 的情况下执行带范围检查的
解析，`format_duration_exact` 自动选择最大的精确规范单位。

富文本的规范格式包括：日期 `YYYY-MM-DD`、时间 `HH:MM:SS[.fraction]`、
instant 的 RFC 3339、绝对 URL、标准 JSON，以及 key 唯一且 value 全为字符串的
StringMap JSON object。

## 9. 批量与标量字符串集合

`DataConverters` 转换现有迭代器，失败时报告原始 `source_index`。
`ScalarStringDataConverters` 可惰性拆分一个标量字符串；跳过空元素不会重排后续索引。

```rust
use qubit_datatype::{DataConversionOptions, DataConverters, ScalarStringDataConverters};

let ports: Vec<u16> = DataConverters::from(vec!["8080", "8081"])
    .to_vec()
    .unwrap();
assert_eq!(ports, [8080, 8081]);

let options = DataConversionOptions::env_friendly();
let values: Vec<u16> = ScalarStringDataConverters::new("1, 2, 3")
    .to_vec_with(&options)
    .unwrap();
assert_eq!(values, [1, 2, 3]);
```

需要全部保留项时使用 `to_vec`；只关心第一个保留值时使用 `to_first`。

## 10. 扩展下游目标类型

下游 crate 可以为自己的 newtype 实现 `DataConversionTarget`，并委托给内置目标。

```rust
use qubit_datatype::{DataConversionError, DataConversionOptions,
    DataConversionTarget, DataConverter, DataType, DataTypeOf};

#[derive(Debug, PartialEq, Eq)]
struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert_from(source: &DataConverter<'_>, options: &DataConversionOptions)
        -> Result<Self, DataConversionError>
    {
        u16::convert_from(source, options).map(Self)
    }
}

assert_eq!(DataConverter::from("8080").to::<Port>(), Ok(Port(8080)));
```

优先采用委托，以继承内置目标的规范化、精度、错误和 feature 契约。

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
