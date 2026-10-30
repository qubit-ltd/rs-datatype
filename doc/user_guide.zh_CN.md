# Qubit Datatype 用户手册

[English user guide](user_guide.md) · 适用于 `qubit-datatype` 0.15.0

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
qubit-datatype = { version = "0.15", default-features = false, features = ["converter"] }
```

`converter` 已包含 `duration`。如果只需要 `DataType` 和 `NumberRef`，使用不带可选
feature 的 `qubit-datatype = "0.15"` 即可。

### 核心工作流

严格策略是默认值：它保留空白，只接受默认的布尔字面量，并拒绝会损失精度的数值和
Duration 转换。处理环境变量风格的输入时，通常应明确选用 `env_friendly()`。布尔字面量
匹配优先于数值文本字节数限制。未知布尔文本若超过 `numeric.max_text_bytes`，会返回
`NumericTextBytes` 限额错误，即使它不是合法整数语法；未超限的未知文本仍返回
`InvalidBoolean`。错误包含类型和资源上下文，不会保留来源值。

```rust
use std::time::Duration;

use qubit_datatype::{ConversionLimits, ConversionPolicy, DataConversionError, DataConverter};

fn main() -> Result<(), DataConversionError> {
    let policy = ConversionPolicy::env_friendly();
    let limits = ConversionLimits::default();

    let port = DataConverter::from("8080").to_with::<u16>(&policy, &limits)?;
    let admin = DataConverter::from(" yes ").to_with::<bool>(&policy, &limits)?;
    let retry = DataConverter::from("1500ms").to_with::<Duration>(&policy, &limits)?;

    assert_eq!(port, 8080);
    assert!(admin);
    assert_eq!(retry, Duration::from_millis(1500));
    Ok(())
}
```

执行结果可以直接观察：三个值都已成为目标 Rust 类型，而且没有隐式选择转换规则。各次
调用彼此独立时，使用 `to_with` 即可；该方法每次都会创建新的会话。

### Profile 与数值文本语法

转换 profile 用于选择文本规范化和转换行为，例如是否 trim，或是否允许精度损失；它们不会
让所有目标类型共用同一套数值词法规则。`BigDecimal` 在三种 profile 下都会接受其解析器原本
支持的下划线形式：`"12_34"`、`"1__2"`、`"12_"` 和 `"1_2.5"`。固定宽度整数和 `BigInt`
目标会把这些文本视为无效整数语法，并返回 `InvalidSyntax`。

这种按目标类型区分的行为保留了 `BigDecimal` 解析器的既有兼容性，同时明确了整数语法。
选择 `lossy()` 或 `env_friendly()` 不会改变各目标解析器接受的数值拼写。

### 为数值转换设置资源上限

`NumericConversionLimits` 的四项限制分别作用于以下转换阶段：

| 限制项 | 生效阶段 |
| --- | --- |
| `max_text_bytes` | 解析数值文本时生效，统计配置的字符串规范化之后的 UTF-8 字节数。 |
| `max_big_integer_digits` | 物化 `BigInt` 结果时生效；前导零和零结果不占用位数额度。 |
| `max_big_decimal_coefficient_digits` | 目标类型为 `BigDecimal` 时生效，包括把已有值复制到该目标。 |
| `max_big_decimal_scale_magnitude` | 目标类型为 `BigDecimal` 时生效，包括把已有值复制到该目标。 |

例如，应用要把整数文本转换为 `BigInt`，需要启用 `converter` 和 `big-integer`，并分别
配置输入文本与结果位数上限：

```toml
[dependencies]
qubit-datatype = { version = "0.15", default-features = false, features = ["converter", "big-integer"] }
num-bigint = "0.4"
```

下面的完整示例接受三位结果 `999`，并拒绝 `1000`，因为生成的 `BigInt` 有四位。文本上限
高于这两个输入的长度，因此拒绝原因是结果位数超限：

```rust
use num_bigint::BigInt;
use qubit_datatype::{
    ConversionLimits, ConversionPolicy, ConversionResource, DataConverter,
    NumericConversionLimits,
};

fn main() {
    let numeric = NumericConversionLimits::builder()
        .max_text_bytes(64)
        .max_big_integer_digits(3)
        .build();
    let limits = ConversionLimits::builder().numeric_limits(numeric).build();
    let policy = ConversionPolicy::strict();
    assert_eq!(
        DataConverter::from("999").to_with::<BigInt>(&policy, &limits),
        Ok(BigInt::from(999_u16)),
    );
    let error = DataConverter::from("1000")
        .to_with::<BigInt>(&policy, &limits)
        .expect_err("four result digits exceed the configured maximum");
    assert_eq!(error.resource(), Some(ConversionResource::BigIntegerDigits));
    assert_eq!(error.configured_limit(), Some(3));
}
```

这些限制不覆盖已有数值转换过程中的所有临时分配。例如，把已构造的 `BigDecimal` 转为
浮点数时，转换器可能先把完整系数格式化为解析器输入；系数位数和 scale 上限只在目标为
`BigDecimal` 时生效，不会自动限制已有 `BigDecimal` 转整数或浮点数的全部工作量。需要更强的
资源边界时，应在应用的输入或反序列化边界限制不可信数值表示的规模。`NumberRef` 的比较也
独立于转换会话，不受其中的转换预算计量；详见[比较不同数值表示](#比较不同数值表示)。

各 builder 方法的具体约定及生效阶段见
[`NumericConversionLimits` API 文档](https://docs.rs/qubit-datatype/0.15.0/qubit_datatype/struct.NumericConversionLimits.html)。

## 进阶用法

### 读取部署端口列表

对于 `PORTS=8080,, 8443;9000` 这样的标量配置，应显式设置拆分、分隔符、trim 和空项策略。
[可运行的 `deployment_lists` 示例](../examples/deployment_lists.rs)可通过
`cargo run --example deployment_lists --features converter` 执行，下面展示相同的核心调用：

```rust
use qubit_datatype::{
    CollectionConversionPolicy, ConversionLimits, ConversionPolicy,
    DataConversionErrorKind, EmptyItemPolicy, ScalarStringDataConverters,
};

let collection = CollectionConversionPolicy::builder()
    .split_scalar_strings(true)
    .delimiters([',', ';'])
    .trim_items(true)
    .empty_item_policy(EmptyItemPolicy::Skip)
    .build();
let policy = ConversionPolicy::builder().collection_policy(collection).build();
let limits = ConversionLimits::default();
let ports = ScalarStringDataConverters::from("8080,, 8443;9000")
    .to_vec_with::<u16>(&policy, &limits)
    .expect("所有端口都应有效");
assert_eq!(ports, vec![8080, 8443, 9000]);

let error = ScalarStringDataConverters::from("8080,,bad")
    .to_vec_with::<u16>(&policy, &limits)
    .expect_err("第三项无效");
assert_eq!(error.source_index(), 2);
assert_eq!(error.conversion_error().kind(), DataConversionErrorKind::InvalidValue);
assert_eq!(
    ScalarStringDataConverters::from("8080,,bad").to_first_with::<u16>(&policy, &limits)
        .expect("第一项有效"),
    8080,
);
```

`source_index()` 是原始输入中的零起始索引。跳过空项不会重新编号，因此 `bad` 仍是索引 `2`。
`to_first_with` 只转换首项，不验证尚未访问的尾部内容；若必须确认整个配置有效后才能应用，
请使用 `to_vec_with`。对于已经物化的列表，应改用 `DataConverters`，不需要再次拆分每个字符串元素。

如果多次转换属于同一操作，应复用一个 `ConversionSession`。示例设置两条目的操作上限，
尝试转换三个端口，并在原始索引 `2` 观察到 `ConversionResource::Items`。整个标量来源先准入
并计入字节预算，随后按访问顺序累计条目预算；转换失败不会退还已经扣除的来源字节或条目。
只有全量转换成功后才应用部署配置；拒绝时可报告索引和错误分类，不要记录可能包含机密的原始配置值。

在更底层的迭代接口中，`ScalarItems` 遇到被拒绝的空项后仍可继续读取，但条目上限错误会耗尽该迭代器。
`AdmittedScalarSource::next_item` 遇到拆分或条目准入错误后会耗尽来源；之后的目标转换错误本身不会耗尽来源。
普通应用可使用 `ScalarStringDataConverters`，由适配器处理这些迭代细节。

### 在一个逻辑操作内共享预算

当整个请求而非单个字段需要共享条目或字节预算时，构造 `ConversionSession`。下面的配置
只允许两个已准入条目，第三次转换会失败。

```rust
use qubit_datatype::{
    ConversionLimits, ConversionOperationLimits, ConversionPolicy,
    ConversionSession, DataConversionError, DataConverter,
};

fn main() -> Result<(), DataConversionError> {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .operation_limits(ConversionOperationLimits::builder().max_items(2).build())
        .build();
    let mut session = ConversionSession::new(&policy, &limits);

    assert_eq!(DataConverter::from("1").to_in::<u16>(&mut session)?, 1);
    assert_eq!(DataConverter::from("2").to_in::<u16>(&mut session)?, 2);
    assert!(DataConverter::from("3").to_in::<u16>(&mut session).is_err());
    Ok(())
}
```

条目预算检查通过后，即使后续输入检查或目标转换失败，已扣除的条目额度仍会保留；
已成功扣除的输入字节也不会退回。条目预算检查自身失败时，不会新增条目消耗。
单次输入字节扣减失败时不会新增字节消耗，但不会撤销此前已经扣除的条目额度。
内置 `String` 转换，以及自定义目标通过 `AdmittedConversion::write_string` 进行的渲染，均按 UTF-8 字节计入输出预算。
启用 `json` feature 后，`AdmittedConversion::encode_json` 也会计入实际编码后的 JSON 字节，包括格式字符和转义产生的字节。
每个输出操作仅在自身成功时提交其字节；自定义目标之后失败，不会撤销此前已成功输出操作的计费。
自定义目标自行产生的其他输出不会自动计量，此预算也不限制所有临时分配。其他输入、条目和结构化预算仍遵循原有规则。
需要观察预算时，可使用会话的 `*_used`、`*_remaining` 和 `*_limit` 访问器。`ConversionSession` 是操作级所有者；自定义目标不会直接获得它。

### 安全扩展目标类型

为下游 newtype 实现 `DataConversionTarget`。转换器会传入一个
`AdmittedConversion`，其中包含框架已经选定的原始值，以及绑定好的策略、限制、会话和错误上下文。
这个对象由框架创建，因此目标类型不需要自己实现输入包装，也不能再次执行顶层准入。

```rust
use qubit_datatype::{
    AdmittedConversion, DataConversionError, DataConversionTarget, DataConverter,
    DataType, DataTypeOf,
};

struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert(input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        input.convert::<u16>().map(Self)
    }
}

fn main() -> Result<(), DataConversionError> {
    assert_eq!(DataConverter::from("8080").to::<Port>()?.0, 8080);
    Ok(())
}
```

`input.convert::<u16>()` 会消费已准入封装，并把同一个来源交给内置目标，不会产生第二次顶层转换。
封装还提供 `from_type()`、`to_type()`、`source()`，以及 JSON 辅助方法（启用 `json` 时）和事务式
`write_string`。旧的不安全委托方法已不再属于 API，原始准入和记账操作仍保持内部可见。

委托目标的 `DataType` 必须与这个 context 绑定的外层目标一致。描述符不同，框架会在调用被委托目标前返回 `Unsupported`；错误仍保留原始来源类型和外层目标类型。顶层条目及已成功扣除的输入字节继续计入会话；这次拒绝不会增加其他扣减，也不会退回此前用量。

这会收紧此前可能成功的跨描述符自定义委托，调用结果可能改为 `Unsupported`。如果 newtype 只是给内置类型增加领域约束，应声明与被包装类型相同的 `DataType`，委托转换后再检查领域规则。若语义确实不同，可在自定义目标中通过 `source()` 检查并映射来源值；生成文本或结构化输出时使用现有预算辅助方法。不要另建会话来绕开累计限制。

### 比较不同数值表示

验证、存储、确定性排序和键应选用 `NumericComparisonPolicy::Exact`。`Approximate` 仅适合
成对的 IEEE 风格比较：混合表示时它可能不具备传递性，因此不能用于实现 `Ord`、排序、
分组或构造有序映射和有序集合的键。

```rust
use std::cmp::Ordering;

use qubit_datatype::{NumberRef, NumericComparisonPolicy};

fn main() {
    let integer = NumberRef::from((1_u64 << 53) + 1);
    let float = NumberRef::from((1_u64 << 53) as f64);
    assert_eq!(
        integer.compare(float, NumericComparisonPolicy::Exact),
        Some(Ordering::Greater),
    );
}
```

NaN 没有顺序；正零和负零相等。

固定宽度数值的比较不需要堆分配。部分涉及 `BigInt` 或 `BigDecimal` 的路径可能会克隆系数，
或构造精确有理数、十进制数，成本会随数值表示规模增长；某些大数之间也有直接比较路径。
`ConversionLimits` 和 `ConversionSession` 管理的是转换，不会为这种独立比较计量预算。若数值来自
不可信输入，应在输入边界限制其表示规模。

### 单独解析 Duration 文本

不需要运行时转换时，可启用 `duration` 后使用轻量文本 API。默认选项拒绝没有单位后缀的
输入，并要求严格单位拼写。

```rust
use std::time::Duration;

use qubit_datatype::{parse_duration_text, DurationTextOptions};

fn main() -> Result<(), qubit_datatype::DurationParseError> {
    let value = parse_duration_text("1500ms", &DurationTextOptions::default())?;
    assert_eq!(value, Duration::from_millis(1500));
    assert!(parse_duration_text("1500", &DurationTextOptions::default()).is_err());
    Ok(())
}
```

严格模式接受 `us`、`µs` 和 `μs` 三种微秒拼写，但拒绝仅在宽松模式中接受的分钟别名
`m`；规范输出固定使用 `µs` 和 `min`。

Serde 字段应按 wire contract 选择适配器：

| 适配器 | 表示 | 精度与范围 |
| --- | --- | --- |
| `duration_millis` | JSON 数字 | 向毫秒 half-up 舍入；舍入结果超过 `u64::MAX` 毫秒时返回错误 |
| `duration_millis_with_unit` | `"<integer>ms"` | 向毫秒 half-up 舍入；序列化在 `18446744073709551615999ms` 饱和，反序列化则拒绝超出 `Duration` 范围的值 |
| `duration_millis_with_unit_exact` | `"<integer>ms"` | 拒绝亚毫秒值；整毫秒范围遵循 `Duration`，不受 `u64` 毫秒上限约束 |
| `duration_with_unit` | 规范化单位字符串 | 精确，不进行舍入或饱和 |

以下示例只需启用 `duration`，不需要 `converter`。请直接依赖 Serde derive 和
`serde_json`：

```toml
[dependencies]
qubit-datatype = { version = "0.15", default-features = false, features = ["duration"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

```rust
use std::time::Duration;
use serde::Serialize;

#[derive(Serialize)]
struct NumericDelay {
    #[serde(with = "qubit_datatype::serde::duration_millis")]
    delay: Duration,
}

#[derive(Serialize)]
struct TextDelay {
    #[serde(with = "qubit_datatype::serde::duration_millis_with_unit")]
    delay: Duration,
}

#[derive(Serialize)]
struct ExactDelay {
    #[serde(with = "qubit_datatype::serde::duration_millis_with_unit_exact")]
    delay: Duration,
}

fn main() {
    let rounded = NumericDelay { delay: Duration::from_micros(1_500) };
    assert_eq!(serde_json::to_value(&rounded).unwrap(), serde_json::json!({"delay": 2}));
    let inexact = ExactDelay { delay: Duration::from_micros(1_500) };
    assert!(serde_json::to_string(&inexact).is_err());
    let exact = ExactDelay { delay: Duration::from_millis(2) };
    assert_eq!(serde_json::to_value(&exact).unwrap(), serde_json::json!({"delay": "2ms"}));
    assert!(serde_json::to_string(&NumericDelay { delay: Duration::MAX }).is_err());
    assert_eq!(
        serde_json::to_value(&TextDelay { delay: Duration::MAX }).unwrap(),
        serde_json::json!({"delay": "18446744073709551615999ms"}),
    );
}
```

### 限制结构化输入

读取配置或协议中的嵌套 JSON 时，可用 `StructuredConversionLimits` 在构造目标值前限制
每个值的规模。深度从根值开始计数，根值深度为 1。数组和对象的上限分别按各自的直接
子项统计，不会把整棵树的条目相加。整个会话累计的 `max_structured_nodes` 是另一项预算。

此示例除 `converter` 外还需要启用 `json` feature，并直接依赖 `serde_json`。使用以下依赖配置：

```toml
[dependencies]
qubit-datatype = { version = "0.15", default-features = false, features = ["converter", "json"] }
serde_json = "1.0"
```

```rust
use qubit_datatype::{
    ConversionLimits, ConversionPolicy, ConversionResource, DataConverter,
    StructuredConversionLimits,
};

fn main() {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .structured_limits(
            StructuredConversionLimits::builder()
                .max_depth(3)
                .max_sequence_items(2)
                .max_map_entries(1)
                .build(),
        )
        .build();
    let value = DataConverter::from("[[1],[2]]")
        .to_with::<serde_json::Value>(&policy, &limits)
        .expect("嵌套值的深度不超过 3");
    assert_eq!(value, serde_json::json!([[1], [2]]));

    let zero_items = ConversionLimits::builder()
        .structured_limits(StructuredConversionLimits::builder().max_sequence_items(0).build())
        .build();
    assert!(DataConverter::from("[]")
        .to_with::<serde_json::Value>(&policy, &zero_items)
        .is_ok());
    let error = DataConverter::from("[1]")
        .to_with::<serde_json::Value>(&policy, &zero_items)
        .expect_err("数组上限为零时应拒绝非空数组");
    assert_eq!(error.resource(), Some(ConversionResource::SequenceItems));
}
```

来源已经是 `serde_json::Value` 时也执行相同检查。深度上限为零时，深度为 1 的根标量也会
被拒绝；数组或对象的子项上限为零时，只有空容器可以通过。

`max_text_bytes` 限制受支持文本转换路径中的规范化来源文本，不是已物化 JSON 值的单字符串
限额。可复用的 `value()` 模板也携带相同最大值的字符串字节限额，直接应用该模板的使用者会
执行此检查。内置转换会话从模板中选取结构限额，并叠加累计节点和 payload 预算；不会应用模板
的字符串字节限额。因此，该模板不是会话最终生效预算的快照。

自定义目标调用 `AdmittedConversion::decode_json` 时，helper 还会按传入字节的原始长度检查
`max_text_bytes`，其中包括空白。它不会规范化输入，也不会再次准入输入。若此检查拒绝文本，
已扣除的条目和来源字节仍保留扣减，且不会新增结构或输出字节用量。内置文本转 JSON 路径仍会在
解码前检查规范化后的来源文本。

### 理解策略 builder 的调用顺序

`string_policy` 这样的组 setter 会替换整组策略；`blank_string_policy` 这样的叶 setter 只修改
当前组中的一个字段。设置会按调用顺序生效，因此后调用的组 setter 会重置此前设置的叶字段：

`BooleanConversionPolicy` 的 builder 会收集字面量列表和标量设置，并在 `build()` 时验证最终组合。
因此，先设置 `case_sensitive` 还是先设置字面量不会改变构建结果；同一标量字段重复设置时以后一次
为准。这不改变 `string_policy` 等组 setter 的整组替换行为。

```rust
use qubit_datatype::{
    BlankStringPolicy, CollectionConversionPolicy, ConversionPolicy,
    EmptyItemPolicy, StringConversionPolicy,
};

fn main() {
    let leaf_last = ConversionPolicy::builder()
        .string_policy(StringConversionPolicy::default())
        .blank_string_policy(BlankStringPolicy::Reject)
        .build();
    assert_eq!(leaf_last.string().blank_string_policy(), BlankStringPolicy::Reject);

    let group_last = ConversionPolicy::builder()
        .blank_string_policy(BlankStringPolicy::Reject)
        .string_policy(StringConversionPolicy::default())
        .build();
    assert_eq!(group_last.string().blank_string_policy(), BlankStringPolicy::Preserve);

    let collection = ConversionPolicy::builder()
        .empty_item_policy(EmptyItemPolicy::Reject)
        .collection_policy(CollectionConversionPolicy::default())
        .build();
    assert_eq!(collection.collection().empty_item_policy(), EmptyItemPolicy::Keep);
}
```

### 精确处理 Instant 文本

启用 `chrono` 后，RFC 3339 Instant 文本的小数部分最多允许 9 位。所有转换策略都遵守这一
规则，超出的位数即使全是尾随零也会被拒绝。例如，`.123456789Z` 可以解析，
`.1234567890Z` 则不可以。上游数据若带有更高精度，应先按领域规则明确舍入或截断，再进行
转换；也可以直接传入已经构造好的 `DateTime<Utc>`。

## 错误与诊断

应按稳定的 `DataConversionErrorKind` 处理 `DataConversionError`，不要依赖格式化后的错误
文本。主要类别为 `Missing`、`EmptyCollection`、`Unsupported`、`InvalidValue` 和
`LimitExceeded`；`Quantity` 表示原生计量值无法容纳在配置的资源数量中。

```rust
use qubit_datatype::{DataConversionErrorKind, DataConverter};

fn main() {
    let error = DataConverter::from("3.9").to::<i32>().unwrap_err();
    assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
}
```

处理无效值时，如需稳定的拒绝原因，可检查 `reason()`。资源限制失败时，可按需检查
`resource()`、`configured_limit()`、`observed_lower_bound()`、`used()`、`remaining()` 和
`requested()`。批量 API 返回 `DataListConversionError`，其中保留原始 `source_index` 和
转换错误。

JSON 转换失败时，可通过 `json_diagnostic()` 查看解码器提供的安全元数据；错误的通用分类仍是
`InvalidValue`。`category()` 返回 `Read`、`EmptyInput`、`InvalidUtf8`、`InvalidJson`、
`UnexpectedTopLevel` 或 `Deserialize`。下面示例使用 `converter` 和 `json` 两个 feature，并直接依赖
`serde_json`；依赖配置见[限制结构化输入](#限制结构化输入)：

```rust
use qubit_datatype::{DataConverter, JsonConversionErrorCategory};

let error = DataConverter::from("{\n@}")
    .to::<serde_json::Value>()
    .unwrap_err();
let diagnostic = error.json_diagnostic().expect("JSON diagnostic");
assert_eq!(diagnostic.category(), JsonConversionErrorCategory::InvalidJson);
assert_eq!(diagnostic.line(), Some(2));
assert_eq!(diagnostic.column(), Some(1));
```

行号从 1 开始。位置对应实际交给解码器的文本；该文本可能已经过规范化。`InvalidJson` 的列按
Unicode 字符计数，`Deserialize` 的列按 UTF-8 字节计数。并非每种错误类别或每个调用入口都会提供
行列；解码器未提供的位置由相应访问器返回 `None`。诊断不会保存输入值、对象键或后端错误消息。
预算耗尽仍返回 `LimitExceeded`，且不附带 JSON 诊断。附加诊断不会改变 `DataConversionError` 的
公共相等性比较结果，比较时不考虑诊断。

`requested()` 表示被拒绝的累计请求，可能是保守的最低预留量或格式化器的一次写入量，
而不是最终完整文本的精确长度。资源失败还可通过 `std::error::Error::source` 访问底层原因；
稳定分类仍应优先使用现有访问器。

自定义标量列表适配器应调用 `session.admit_scalar_string_source(text)`，通过返回的
`AdmittedScalarSource::next_item` 逐项借用已准入元素，再用 `convert` 或
`convert_with_session` 消费。该接口保留原始索引，完整来源字节只计费一次；未访问的元素
不会被扫描。分割错误和元素准入错误会终止来源访问。

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
