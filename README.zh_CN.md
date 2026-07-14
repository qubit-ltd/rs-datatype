# Qubit Datatype

[![Rust CI](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-datatype/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-datatype/coverage-badge.json)](https://qubit-ltd.github.io/rs-datatype/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-datatype.svg?color=blue)](https://crates.io/crates/qubit-datatype)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的运行时类型描述和策略驱动转换工具。

## 安装

默认构建只包含轻量类型词汇，不引入可选依赖：

```toml
[dependencies]
qubit-datatype = "0.4"
```

需要完整转换引擎时：

```toml
[dependencies]
qubit-datatype = { version = "0.4", features = ["converter"] }
```

## Features

| Feature | 内容 |
| --- | --- |
| default | 不启用任何可选依赖 |
| chrono | Chrono 类型的 `DataTypeOf` |
| big-number | `BigInt`、`BigDecimal` 的 `DataTypeOf` |
| url | `Url` 的 `DataTypeOf` |
| json | JSON 值的 `DataTypeOf` |
| converter | 完整转换 API，并聚合所有 rich-type feature |

## 类型词汇

`DataType` 提供 27 个稳定类型名、完整的 `ALL` 数组、数值分类方法、
Serde 支持和大小写不敏感解析；`DataTypeOf` 把 Rust 类型映射为运行时描述。

```rust
use qubit_datatype::{DataType, DataTypeOf};

assert_eq!(i32::DATA_TYPE, DataType::Int32);
assert!(DataType::Int32.is_signed_integer());
assert_eq!(DataType::ALL.len(), 27);
```

## 转换契约

启用 `converter` 后，`DataConverter` 负责单值转换，
`DataConverters` 负责迭代器转换，`ScalarStringDataConverters` 惰性拆分
标量字符串并保留原始索引。

默认 `NumericConversionPolicy::Exact` 禁止截断、舍入和精度损失。
只有显式选择 `Lossy`，才允许有限浮点/十进制向零截断、整数到浮点的 IEEE
舍入，以及 Duration 的 half-up 舍入。

```rust
# #[cfg(feature = "converter")]
# {
use qubit_datatype::{
    DataConversionError, InvalidValueReason, DataConversionOptions,
    DataConverter, NumericConversionPolicy,
};

assert!(matches!(
    DataConverter::from("3.9").to::<i32>(),
    Err(DataConversionError::InvalidValue {
        reason: InvalidValueReason::PrecisionLoss,
        ..
    }),
));

let lossy = DataConversionOptions::default()
    .with_numeric_policy(NumericConversionPolicy::Lossy);
assert_eq!(DataConverter::from("3.9").to_with::<i32>(&lossy), Ok(3));
# }
```

### 转换矩阵

下表中的“数值”包括 primitive 整数/浮点数和任意精度数。内容无效返回
`Invalid`，表外类型组合返回 `Unsupported`，typed empty 返回 `Missing`。

| 来源族 | 支持的目标 |
| --- | --- |
| 任意具体值 | 自身类型；`String` |
| `String` | 数值、bool、char、Chrono、Duration、URL、JSON、StringMap |
| Bool / char | primitive 数值 |
| 整数 / BigInt | 数值、bool、Duration |
| 浮点 / BigDecimal | 数值 |
| Duration | 整数、String |
| StringMap | JSON、String |
| JSON | String |

### 字符串与布尔值

默认不 trim。每次字符串转换只调用一次
`StringConversionOptions::normalize`；需要时显式开启 `trim`。空白字符串可保留、
视为缺失或拒绝。

默认布尔文字只有 `true`、`false`，匹配时 ASCII 大小写不敏感。数值 0/1
由独立的 `BooleanNumericPolicy::ZeroOrOne` 控制，还可选择 `NonZero` 或
`Reject`。文字 builder 返回 `Result`，因此无法制造 true/false 重叠集合。

```rust
# #[cfg(feature = "converter")]
# {
use qubit_datatype::{
    DataConversionOptions, DataConverter, StringConversionOptions,
};

assert!(DataConverter::from(" true ").to::<bool>().is_err());
let options = DataConversionOptions::default().with_string_options(
    StringConversionOptions::default().with_trim(true),
);
assert_eq!(DataConverter::from(" true ").to_with::<bool>(&options), Ok(true));
# }
```

### Duration

Duration 文本格式为 `[0-9]+(ns|us|ms|s|m|h|d)?`，无后缀时使用配置单位；
拒绝空白、符号、小数和非 ASCII 后缀。大整数先拆成秒和纳秒，再判断最终
`Duration` 是否越界。

Duration 转整数和字符串同样遵循数值策略：Exact 要求按配置单位整除，Lossy
采用 half-up 舍入。

### Rich text 格式

- char：恰好一个 Unicode scalar value
- date：`YYYY-MM-DD`
- time：`HH:MM:SS[.fraction]`，小数秒 1–9 位
- local date-time：`YYYY-MM-DDTHH:MM:SS[.fraction]`
- instant：带 `Z` 或 offset 的 RFC 3339
- BigInt：带可选正负号的十进制整数
- BigDecimal：十进制及可选指数
- URL：绝对 URL
- JSON：任意合法 JSON
- StringMap：key 唯一、value 全为字符串的 JSON object

## 结构化错误与集合

`DataConversionError` 只有 `Missing`、`Unsupported` 和
`InvalidValue { reason }` 三类。错误保存来源/目标 `DataType`，但不保存或显示原始值。

列表错误使用 `DataListConversionError::source_index`。空元素 `Skip` 不会重排
后续索引；`to_first` 找到首个保留项就停止，不验证尾部。

## 开发

```bash
cargo +1.94.0 test --no-default-features
cargo +1.94.0 test --all-features
./coverage.sh text
./align-ci.sh
./ci-check.sh
```

覆盖率命令和阈值见 [COVERAGE.zh_CN.md](COVERAGE.zh_CN.md)。

## 许可证

采用 Apache License 2.0，详见 [LICENSE](LICENSE)。

## 作者

**胡海星** — Qubit Co. Ltd.
