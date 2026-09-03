# Qubit Datatype 用户手册

[English user guide](user_guide.md) · 适用于 `qubit-datatype` 0.12.0

本手册面向需要把配置、协议或数据接入值转换为 Rust 强类型的应用开发者。它说明如何
显式控制精度、可接受的文本格式和资源消耗；项目能力概览请看
[README](../README.zh_CN.md)，逐项 API 请查阅
[docs.rs](https://docs.rs/qubit-datatype)。

## 手册目标与读者

当来源类型只能在运行时确定、需要稳定的类型词汇，或必须在不同数值表示之间比较而不
隐式经过 `f64` 时，可以使用本 crate。它不是通用序列化框架，也不会让不受支持的来源与
目标类型组合自动变得可转换。

默认 feature 提供 `DataType`、`DataTypeOf`、`NumberRef` 和数值比较。启用 `duration`
可使用 Duration 文本与 Serde 适配器；启用 `converter` 可进行运行时转换。`chrono`、
`big-integer`、`big-decimal`、`url`、`json` 等富类型 feature 会增加相应映射，但只有与
`converter` 一同启用时才参与转换。

## 概念模型

`DataType` 是 crate 的稳定运行时类型词汇，`DataTypeOf` 将受支持的 Rust 类型映射到该
词汇。`DataConverter` 持有或借用运行时来源值；一次转换由 `ConversionPolicy` 决定语义，
由 `ConversionLimits` 限制资源。只有多次调用必须共用累计预算时，才需要
`ConversionSession`。

```text
运行时来源值 ── DataConverter ── 策略 + 限制 ──> 强类型结果
                                      │
                                      └─ 可选的共享 ConversionSession
```

`DataConversionError` 提供稳定的错误分类和类型上下文，但刻意不保留来源值。因此，即使
来源是环境变量，也可在不意外泄露内容的情况下报告错误。

## 实战场景：读取部署配置

假设服务收到 `PORT=8080`、`ADMIN_ENABLED=yes` 和 `RETRY_DELAY=1500ms`。成功标准是
得到 `u16`、`bool` 和 `Duration`；不合法值必须以稳定的错误分类被拒绝，不能悄悄转换。

### 安装与最小配置

添加转换和 Duration API：

```toml
[dependencies]
qubit-datatype = { version = "0.12", default-features = false, features = ["converter"] }
```

`converter` 已包含 `duration`。如果只需要 `DataType` 和 `NumberRef`，使用不带可选
feature 的 `qubit-datatype = "0.12"` 即可。

### 核心工作流

严格策略是默认值：它保留空白，只接受默认的布尔字面量，并拒绝会损失精度的数值和
Duration 转换。处理环境变量风格的输入时，通常应明确选用 `env_friendly()`。

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

执行结果可以直接观察：三个值都已成为目标 Rust 类型，而且没有隐式选择转换规则。各次
调用彼此独立时，使用 `to_with` 即可；该方法每次都会创建新的会话。

## 进阶用法

### 在一个逻辑操作内共享预算

当整个请求而非单个字段需要共享条目或字节预算时，构造 `ConversionSession`。下面的配置
只允许两个已准入条目，第三次转换会失败。

```rust
use qubit_datatype::{
    ConversionLimits, ConversionOperationLimits, ConversionPolicy,
    ConversionSession, DataConverter,
};

let policy = ConversionPolicy::strict();
let limits = ConversionLimits::builder()
    .operation_limits(ConversionOperationLimits::builder().max_items(2).build())
    .build();
let mut session = ConversionSession::new(&policy, &limits);

assert_eq!(DataConverter::from("1").to_in::<u16>(&mut session)?, 1);
assert_eq!(DataConverter::from("2").to_in::<u16>(&mut session)?, 2);
assert!(DataConverter::from("3").to_in::<u16>(&mut session).is_err());
# Ok::<(), qubit_datatype::DataConversionError>(())
```

已经准入但随后失败的转换仍会保留条目和输入字节消耗；准入失败不会再消耗一个条目。
需要观察预算时，可使用会话的 `*_used`、`*_remaining` 和 `*_limit` 访问器。
`ConversionSession` 是操作级所有者；自定义目标不会直接获得它。

### 安全扩展目标类型

为下游 newtype 实现 `DataConversionTarget`。转换器会传入已绑定当前来源/目标类型的
`ConversionContext`；用它委托或渲染输出，不要再次准入顶层来源。

```rust
use qubit_datatype::{
    ConversionContext, DataConversionError, DataConversionTarget, DataConverter,
    DataType, DataTypeOf,
};

struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert_from(
        source: &DataConverter<'_>,
        context: &mut ConversionContext<'_, '_>,
    ) -> Result<Self, DataConversionError> {
        // SAFETY: `source` 是外层转换已经准入的原始值。
        unsafe { context.delegate::<u16>(source) }.map(Self)
    }
}
```

启用 `json` 后，`ConversionContext` 还提供 JSON 解码/编码和事务式 `write_string`。
它们共享同一预算与错误类型上下文；原始准入和字节记账操作有意不属于扩展 API。

### 比较不同数值表示

验证、存储、确定性排序和键应选用 `NumericComparisonPolicy::Exact`。`Approximate` 仅适合
成对的 IEEE 风格比较：混合表示时它可能不具备传递性，因此不能用于实现 `Ord`、排序、
分组或构造有序映射和有序集合的键。

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

NaN 没有顺序；正零和负零相等。

### 单独解析 Duration 文本

不需要运行时转换时，可启用 `duration` 后使用轻量文本 API。默认选项拒绝没有单位后缀的
输入，并要求严格单位拼写。

```rust
use std::time::Duration;

use qubit_datatype::{parse_duration_text, DurationTextOptions};

let value = parse_duration_text("1500ms", &DurationTextOptions::default())?;
assert_eq!(value, Duration::from_millis(1500));
assert!(parse_duration_text("1500", &DurationTextOptions::default()).is_err());
# Ok::<(), qubit_datatype::DurationParseError>(())
```

严格模式接受 `us`、`µs` 和 `μs` 三种微秒拼写，但拒绝仅在宽松模式中接受的分钟别名
`m`；规范输出固定使用 `µs` 和 `min`。

Serde 字段应按 wire contract 选择适配器：

| 适配器 | 表示 | 精度 |
| --- | --- | --- |
| `duration_millis` | JSON 数字 | 向毫秒 half-up 舍入 |
| `duration_millis_with_unit` | `"<integer>ms"` | 向毫秒 half-up 舍入 |
| `duration_millis_with_unit_exact` | `"<integer>ms"` | 拒绝亚毫秒值 |
| `duration_with_unit` | 规范化单位字符串 | 精确 |

## 错误与诊断

应按稳定的 `DataConversionErrorKind` 处理 `DataConversionError`，不要依赖格式化后的错误
文本。主要类别为 `Missing`、`EmptyCollection`、`Unsupported`、`InvalidValue` 和
`LimitExceeded`；`Quantity` 表示原生计量值无法容纳在配置的资源数量中。

```rust
use qubit_datatype::{DataConversionErrorKind, DataConverter};

let error = DataConverter::from("3.9").to::<i32>().unwrap_err();
assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
```

处理无效值时，如需稳定的拒绝原因，可检查 `reason()`。资源限制失败时，可按需检查
`resource()`、`configured_limit()`、`observed_lower_bound()`、`used()`、`remaining()` 和
`requested()`。批量 API 返回 `DataListConversionError`，其中保留原始 `source_index` 和
转换错误。

## 排障

| 现象 | 先检查什么 | 处理方式 |
| --- | --- | --- |
| `Unsupported` | 当前 feature 和来源/目标类型组合 | 启用 `converter` 及匹配的富类型 feature，或改用受支持的目标类型。 |
| `" 42 "` 返回 `InvalidValue` | 严格策略不会 trim | 只有确实希望忽略空白时才显式使用 `ConversionPolicy::env_friendly()`。 |
| `"3.9"` 不能转整数 | 严格数值规则会保留精度 | 保持拒绝，或明确选择 `ConversionPolicy::lossy()`。 |
| `LimitExceeded` | 单值限制与会话限制 | 先确认输入可信，再有针对性地提高对应限制。 |
| Duration 文本被拒绝 | 后缀与解析模式 | 使用如 `ms` 的规范后缀；只有外部格式明确时才配置不同策略。 |

## 限制与最佳实践

- 不要把 `DataType` 拼写当作可随意变更的内部文本：`as_str` 返回、Serde 接受且
  `DataType::ALL` 列出的全小写字符串是兼容性接口。
- `isize` 和 `usize` 没有 `DataTypeOf` 映射，因为其表示依赖目标平台。
- 语义策略和资源限制应分开管理：策略决定哪些值可接受，限制约束文本、集合、结构化输入
  和会话。
- 错误保留类型上下文但不保留来源文本。是否记录来源值，应由调用应用自身的机密信息处理
  规则决定。

## 延伸阅读

- [README](../README.zh_CN.md)
- [English user guide](user_guide.md)
- [docs.rs API 文档](https://docs.rs/qubit-datatype)
