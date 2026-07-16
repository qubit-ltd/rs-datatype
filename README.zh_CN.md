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
qubit-datatype = "0.6"
```

需要不引入富类型依赖的基础转换引擎时：

```toml
[dependencies]
qubit-datatype = { version = "0.6", features = ["converter"] }
```

转换器可以只组合实际使用的富类型：

```toml
[dependencies]
qubit-datatype = { version = "0.6", features = ["converter", "chrono"] }
```

## Features

| Feature | 内容 |
| --- | --- |
| default | 不启用任何可选依赖 |
| chrono | Chrono 类型映射；与 `converter` 组合时提供 Chrono 转换 |
| big-number | `BigInt`、`BigDecimal` 类型映射；与 `converter` 组合时提供大数转换 |
| url | `Url` 类型映射；与 `converter` 组合时提供 URL 转换 |
| json | JSON 类型映射；与 `converter` 组合时提供 JSON 和 StringMap 转换 |
| converter | 基础标量、字符串和 Duration 转换 API |
| all | 转换器及全部富类型 feature |

## 典型用法

以下示例涉及转换 API 时，需要启用 `converter` feature。

### 运行时类型词汇

`DataType` 提供 25 个稳定类型名、完整的 `ALL` 数组、数值分类方法、
Serde 支持和大小写不敏感解析；`DataTypeOf` 把 Rust 类型映射为运行时描述。
平台相关的 `isize` 和 `usize` 不提供运行时描述。

```rust
use qubit_datatype::{DataType, DataTypeOf};

assert_eq!(i32::DATA_TYPE, DataType::Int32);
assert!(DataType::Int32.is_signed_integer());
assert_eq!(DataType::ALL.len(), 25);
```

### 单值与批量转换

`DataConverter` 对一个来源值执行一次转换；`DataConverters` 对每个元素应用相同
契约，并在元素失败时报告其原始索引。默认采用精确转换，必须显式选择才能启用有损
行为。

```rust
use qubit_datatype::{
    DataConversionOptions, DataConverter, DataConverters,
};

assert_eq!(DataConverter::from("8080").to::<u16>(), Ok(8080));

let ports: Vec<u16> = DataConverters::from(vec!["8080", "8081"])
    .to_vec()
    .unwrap();
assert_eq!(ports, vec![8080, 8081]);

let lossy = DataConversionOptions::lossy();
assert_eq!(DataConverter::from("3.9").to_with::<i32>(&lossy), Ok(3));
```

### 下游目标类型

为下游自有类型实现 `DataConversionTarget`，并委托给内置目标类型。之后泛型转换
API 就可以直接使用该类型。

```rust
use qubit_datatype::{
    DataConversionError, DataConversionOptions, DataConversionTarget,
    DataConverter, DataType, DataTypeOf,
};

#[derive(Debug, PartialEq, Eq)]
struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        u16::convert_from(source, options).map(Self)
    }
}

let port = DataConverter::from("8080").to::<Port>().unwrap();
assert_eq!(port, Port(8080));
```

### 精确与近似数值排序

跨表示比较需要显式策略。精确模式不会让整数经过 `f64` 舍入；近似模式则会在至少
一侧是浮点表示时，有意使用有限的 `f64` 投影。

```rust
use std::cmp::Ordering;
use qubit_datatype::{
    NumericComparisonPolicy, NumericValueRef, compare_numeric,
};

let integer = NumericValueRef::UInt64((1_u64 << 53) + 1);
let float = NumericValueRef::Float64((1_u64 << 53) as f64);

assert_eq!(
    compare_numeric(integer, float, NumericComparisonPolicy::Exact),
    Some(Ordering::Greater),
);
assert_eq!(
    compare_numeric(integer, float, NumericComparisonPolicy::Approximate),
    Some(Ordering::Equal),
);
```

## 转换契约

启用 `converter` 后，`DataConverter` 负责单值转换，
`DataConverters` 负责迭代器转换，`ScalarStringDataConverters` 惰性拆分
标量字符串并保留原始索引。

转换 API 以目标类型为扩展点：泛型接口只要求 `T: DataConversionTarget`。
下游可以为自己的 newtype 实现该 trait，并委托给已有目标类型，无需再为
`DataConverter` 编写覆盖所有生命周期的约束。

默认 `NumericConversionPolicy::Exact` 禁止截断、舍入和精度损失。
只有显式选择 `Lossy`，才允许有限浮点/十进制向零截断、整数到浮点的 IEEE
舍入，以及 Duration 的 half-up 舍入。

```rust
# #[cfg(feature = "converter")]
# {
use qubit_datatype::{
    DataConversionError, InvalidValueReason, DataConversionOptions,
    DataConverter,
};

assert!(matches!(
    DataConverter::from("3.9").to::<i32>(),
    Err(DataConversionError::InvalidValue {
        reason: InvalidValueReason::PrecisionLoss,
        ..
    }),
));

let lossy = DataConversionOptions::lossy();
assert_eq!(DataConverter::from(" 3.9 ").to_with::<i32>(&lossy), Ok(3));
# }
```

## 数值比较

`compare_numeric` 在显式 `NumericComparisonPolicy` 下比较借用的数值表示。
`Exact` 直接解码 IEEE 有效数与指数，不会经由 `f64` 舍入；大数使用精确有理数，
极端十进制 scale 则委托给 `BigDecimal` 的有界内存精确排序。
`Approximate` 仅在至少一侧是浮点变体时投影到 `f64`，有限投影不可用时回退到
精确比较。NaN 无序，无穷值正常排序，正负零相等。

### 转换矩阵

下表中的“数值”包括 primitive 整数/浮点数和任意精度数。内容无效返回
`InvalidValue`，表外类型组合返回 `Unsupported`，typed empty 返回 `Missing`。

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

Duration 文本格式为 `[0-9]+(ns|us|µs|μs|ms|s|m|h|d)?`。数值输入、
无后缀字符串和输出格式分别配置单位；默认 profile 三者均使用毫秒。拒绝空白、
符号和小数。大整数先拆成秒和纳秒，再判断最终 `Duration` 是否越界。

Duration 转整数和字符串同样遵循数值策略：Exact 要求按输出单位整除，Lossy
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

`DataConversionError` 只有 `Missing`、`EmptyCollection`、`Unsupported` 和
`InvalidValue { reason }` 四个变体。`Missing`、`Unsupported` 和 `InvalidValue`
保存来源/目标 `DataType`；`EmptyCollection` 只保存目标类型。错误不保存或显示原始值。

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

## 许可证

采用 Apache License 2.0，详见 [LICENSE](LICENSE)。

## 作者

**胡海星** — Qubit Co. Ltd.
