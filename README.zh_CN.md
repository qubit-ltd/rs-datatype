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
- `NumberRef`：比较不同数值表示，避免隐式精度损失，并提供通用数值属性查询。
- 轻量 `duration` feature：提供 Duration 单位、带溢出检查的单位运算、可配置文本
  解析和精确规范化格式。
- `converter` feature：按显式策略执行单值、批量和标量字符串集合转换，并返回
  结构化错误，适合配置、协议和数据接入边界。

## 2. 安装与 features

最小构建不启用可选依赖：

```toml
[dependencies]
qubit-datatype = "0.8"
```

按需启用转换器和富类型：

```toml
[dependencies]
qubit-datatype = { version = "0.8", default-features = false, features = ["converter", "chrono"] }
```

| Feature | 能力 |
| --- | --- |
| `duration` | Duration 单位、带检查运算、文本解析和精确格式化 |
| `converter` | 标量、字符串、Duration、StringMap、批量和配置 API；包含 `duration` |
| `chrono` | Chrono 类型映射；与 `converter` 组合时支持转换 |
| `big-integer` | `BigInt` 映射；与 `converter` 组合时支持转换 |
| `big-decimal` | `BigDecimal` 映射；与 `converter` 组合时支持转换 |
| `big-number` | `big-integer` 与 `big-decimal` 的兼容别名 |
| `url` | `Url` 映射；与 `converter` 组合时支持转换 |
| `json` | `serde_json::Value` 映射；与 `converter` 组合时支持 JSON 文本及 StringMap 转换 |
| `all` | `converter` 与全部富类型 feature |

富类型 feature 本身不会启用 `converter`。

`HashMap<String, String>` 的恒等转换只需要 `converter`；把它解析或格式化为 JSON
还需要 `json`。

## 3. 运行时类型描述

`DataType` 提供稳定类型词汇、解析、显示、Serde、分类方法和完整的
`DataType::ALL`。`DataTypeOf` 把 Rust 类型映射为该词汇。平台相关的 `isize`、
`usize` 不提供映射，以免数据表示随目标平台变化。

`DataType::as_str` 返回、Serde 接受且 `DataType::ALL` 列出的全小写拼写属于兼容性
接口；非破坏性版本不会修改既有拼写，也不会把既有拼写复用于其他含义。

```rust
use qubit_datatype::{DataType, DataTypeOf};

assert_eq!(u64::DATA_TYPE, DataType::UInt64);
assert!(DataType::Float64.is_numeric());
assert_eq!("INT32".parse::<DataType>(), Ok(DataType::Int32));
```

## 4. 数值比较

先把借用值包装成 `NumberRef`，再显式选择策略。`Exact` 比较数学值，不让整数
经过 `f64`。有限原生浮点数参与时，`Approximate` 会尝试把两个操作数投影为有限
`f64`；无穷值单独排序，任一操作数无法完成这种投影时回退到精确比较。NaN 无序，
正负零相等。

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

严格 profile 分别拒绝小数转整数截断、已有数值转浮点舍入、文本转浮点舍入以及
不精确的 Duration 输出。有损 profile 允许有限浮点/十进制向零截断、nearest-even
浮点舍入以及 Duration 的 half-up 舍入。十进制与科学计数法字符串转换到定长整数
或 `BigInt` 时共享配置的小数转整数规则。

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

通过 `DataConversionError::kind()` 获取稳定分类：
`DataConversionErrorKind::Unsupported` 表示不支持的类型组合，`Missing` 表示 typed
unset，`EmptyCollection` 表示请求首值时集合为空，`InvalidValue` 表示格式或策略
不合法，`LimitExceeded` 表示触及配置的资源上限。错误只保留类型上下文，不保留
原始值。

## 7. 配置与输入 profile

`DataConversionOptions` 包含相互独立的策略：

- `numeric`：小数转整数、已有数值转浮点、文本转浮点三组策略，以及资源上限。
- `string`：trim 和空白字符串处理。
- `boolean`：文字集合、大小写和数值布尔策略。
- `collection`：标量拆分、分隔符、trim、空项策略和最终保留项数上限。
- `duration`：数值输入单位、无后缀输入、可接受后缀集、输出单位、后缀格式、舍入和源文本字节上限。

`strict()` 是默认值。`env_friendly()` 会 trim 字符串、接受常见布尔文字，并开启
逗号分隔的标量集合；它只把文本转浮点放宽为 nearest-even，不会开启小数转整数
截断或已有数值转浮点舍入。Serde 对省略字段使用默认值，同时拒绝未知字段，因此
配置键拼写错误会立即失败。

```rust
use qubit_datatype::{DataConversionOptions, DataConverter};

let options = DataConversionOptions::env_friendly();
assert_eq!(DataConverter::from(" yes ").to_with::<bool>(&options), Ok(true));
```

数值资源上限属于 options，并在所有 profile 中保持启用。它们在字符串规范化之后、
昂贵解析或 `BigInt` 物化之前生效：

```rust
use qubit_datatype::{
    DataConversionOptions, NumericConversionLimits, NumericConversionOptions,
};

let limits = NumericConversionLimits::default()
    .with_max_text_bytes(4096)
    .with_max_big_integer_digits(10_000);
let options = DataConversionOptions::strict().with_numeric_options(
    NumericConversionOptions::strict().with_limits(limits),
);
```

布尔文字 builder 返回 `Result`，保证在选定大小写规则下 true/false 集合互不重叠。

## 8. 字符串、Duration 与富格式

默认不 trim 字符串。空白值可保留、视为缺失或拒绝。默认扩展 Duration 后缀集接受
`[0-9]+(ns|us|µs|μs|ms|s|m|h|d)?`；ASCII 后缀集不接受 `µs` 和 `μs`。
输入和输出单位分别配置；精确输出要求按输出单位整除，half-up 舍入必须显式开启。

仅启用 `duration` feature 时，`DurationTextOptions` 可选择无后缀策略、ASCII
或扩展后缀集合，并默认把输入限制为 1 MiB；`parse_duration_text` 在处理后缀前
先执行该字节上限，再在不隐式 trim 的情况下执行带范围检查的解析，
`format_duration_exact` 自动选择最大的精确规范单位。

富文本的规范格式包括：日期 `YYYY-MM-DD`、时间 `HH:MM:SS[.fraction]`、
instant 的 RFC 3339、绝对 URL、标准 JSON，以及 key 唯一且 value 全为字符串的
StringMap JSON object。日期、date-time 与 instant 仅格式化 `0000` 至 `9999`
年份；超出四位规范年份范围的值会被拒绝。

## 9. 批量与标量字符串集合

`DataConverters` 转换现有迭代器，失败时报告原始 `source_index`。
`ScalarStringDataConverters` 可惰性拆分一个标量字符串；跳过空元素不会重排后续索引。

标量字符串集合转换默认最多保留 65,536 项。上限在 trim 和空项过滤后检查，
因此被跳过的项不占配额。上限为零时只允许空结果；第一个额外保留项返回
`LimitExceeded`，并保留其原始源索引。可通过
`CollectionConversionOptions::with_max_items` 调整上限。

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
