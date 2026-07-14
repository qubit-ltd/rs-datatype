# `rs-datatype` 及下游生态破坏性重构实施计划

> **供自动化执行者使用：** 必须使用 `subagent-driven-development` 或
> `executing-plans` 逐任务执行；每项行为改动都遵循 TDD，先运行并确认失败测试，
> 再修改生产代码。任务状态使用复选框跟踪。

**目标：** 将 `qubit-datatype` 从“类型词汇与隐式宽松转换混合体”重构为具有明确、
可序列化、默认精确且跨来源一致的转换契约，并迁移全部必要下游 crate。

**架构：** `DataType` 保持为轻量、稳定的运行时类型词汇；转换引擎通过 feature
显式启用，并按数值、布尔、文本、Duration、结构化类型拆分实现。所有调用方共享
同一套结构化错误、转换策略和 Serde 表达，不再在 `rs-config` 中复制 DTO，也不再
把 typed value 强制转换为字符串后重新解析。

**技术栈：** Rust 1.94、Edition 2024、Serde、Chrono、BigDecimal、Num BigInt、URL、
Serde JSON、cargo-llvm-cov、项目 `.rs-ci` 工具链。

## 全局约束

- 当前任务明确允许所有破坏性重构，不保留历史 API 或行为兼容性。
- 不执行 `git add`、`git commit`、`git push`；各独立 Git 仓库分别检查差异。
- 所有生产代码改动必须先有一个因目标行为缺失而失败的测试。
- 所有 Rust 测试继续放在 `tests/`，不在源码内增加 `#[cfg(test)] mod tests`。
- 每个新增源码文件必须有镜像的 `{源文件名}_tests.rs`，并通过 `tests/**/mod.rs`
  导出。
- 所有函数和方法，包括私有辅助函数，按项目 Rust 注释规范补充必要文档。
- 最终 Cargo.toml 使用正常版本依赖，不提交 `[patch.crates-io]`；未发布版本通过
  临时 `CARGO_HOME/config.toml` 映射到本地绝对路径进行完整 CI。
- 每个被修改 crate 先运行 `./align-ci.sh`，再运行 `./ci-check.sh`。
- 版本规则：发生公开 API、公开类型身份或公开 wire 行为变化的 crate 升 minor；
  仅内部依赖更新且公开契约不变的 crate 升 patch。
- 保留当前工作区已有未跟踪文档，不覆盖或删除用户文件。

---

## 目标转换契约

### 数值与布尔策略

新增以下公开类型，并为它们实现 `Default`、Serde 和稳定 snake_case wire 名称：

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NumericConversionPolicy {
    #[default]
    Exact,
    Lossy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BooleanNumericPolicy {
    #[default]
    ZeroOrOne,
    NonZero,
    Reject,
}
```

`DataConversionOptions::default()` 使用 `Exact` 和 `ZeroOrOne`。`Exact` 禁止截断、
舍入和精度损失；`Lossy` 允许有限浮点/十进制向零截断、整数到浮点的 IEEE 舍入，
以及 Duration 到整数的 half-up 舍入，但始终拒绝越界、负数转无符号和非有限值转
整数。

数值字符串先解析为内部 `ParsedNumber::{Integer(BigInt), Decimal(BigDecimal),
NaN, PositiveInfinity, NegativeInfinity}`，再与 typed 数值走同一转换核心。

数值转布尔只接受整数族和整数格式字符串；`ZeroOrOne` 只接受 0/1，`NonZero`
采用 0=false、非零=true，`Reject` 禁止数值来源。补齐 `isize/usize` 与 BigInt。

### 文本格式

- 所有字符串入口只调用一次 `StringConversionOptions::normalize()`；目标 parser
  不得再次 trim。
- `normalize<'a>(&self, value: &'a str)` 返回借用的 `&'a str`，不分配 String。
- `char`：恰好一个 Unicode scalar value。
- `NaiveDate`：`YYYY-MM-DD`。
- `NaiveTime`：`HH:MM:SS`，可带 1–9 位小数秒。
- `NaiveDateTime`：`YYYY-MM-DDTHH:MM:SS[.fraction]`。
- `DateTime<Utc>`：RFC 3339，必须带 `Z` 或 offset，统一转 UTC。
- BigInt：十进制 `[+-]?[0-9]+`；BigDecimal：十进制及 `e/E` 指数。
- URL：`Url::parse` 接受的绝对 URL。
- JSON：任意合法 JSON；StringMap：JSON object 且所有 value 必须为 JSON string。
- Duration：`[0-9]+(ns|us|ms|s|m|h|d)?`，无空格、符号或小数。

### 结构化错误

删除通用字符串错误和 JSON 专用字符串错误，使用：

```rust
pub enum DataConversionError {
    Missing { from: DataType, to: DataType },
    Unsupported { from: DataType, to: DataType },
    Invalid {
        from: DataType,
        to: DataType,
        kind: DataConversionErrorKind,
    },
}

pub enum DataConversionErrorKind {
    BlankRejected,
    InvalidSyntax { expected: &'static str },
    OutOfRange,
    PrecisionLoss,
    NonFinite,
    InvalidBoolean,
    NegativeDuration,
    UnsupportedDurationUnit,
    Serialization { format: DataFormat },
    Deserialization { format: DataFormat },
}

pub enum DataFormat {
    Json,
}
```

错误不保存或显示原始配置值。`DataListConversionError.index` 重命名为
`source_index`。

### 集合语义

`CollectionConversionOptions::scalar_items()` 返回惰性 `ScalarItems`；每项携带
原始 `source_index` 和借用 `&str`。Skip 不重排索引；`to_first` 找到首个保留项后
立即停止，不检查尾部元素。

---

### Task 1：建立干净基线和失败测试清单

**文件：**

- 修改：`rs-datatype/tests/converter/data_conversion_options_tests.rs`
- 修改：`rs-datatype/tests/converter/data_converter_tests.rs`
- 修改：`rs-datatype/tests/converter/scalar_string_data_converters_tests.rs`
- 修改：`rs-datatype/tests/converter/string_conversion_options_tests.rs`
- 修改：`rs-datatype/tests/datatype/data_type_tests.rs`

**产出接口：** 目标 API 和行为由失败测试固定；本任务不修改生产代码。

- [x] 运行基线：`cargo +1.94.0 test --all-features`，预期 73 个集成测试和 7 个
  doctest 通过。
- [x] 为 `NumericConversionPolicy::Exact/Lossy`、`BooleanNumericPolicy`、结构化
  错误、统一 trim、rich type 文本解析、原始集合索引和 `to_first` 短路添加测试。
- [x] 增加表驱动 source-target matrix，精确断言 supported、unsupported、invalid、
  missing 分类，不使用“仅 is_finite/非空”的弱断言。
- [x] 运行最小测试并确认因类型、方法或目标行为不存在而失败：
  `cargo +1.94.0 test --test lib_tests converter::data_converter_tests`。
- [x] 记录 RED 输出，确认不是 import、拼写或测试装配错误。

关键断言必须覆盖：Exact 下 typed/text `3.9 -> i32` 均为 PrecisionLoss；Lossy
下均为 3；ZeroOrOne 拒绝 typed/text 2；NonZero 接受二者；默认 trim=false 时
`" true "`、`" 1 "`、`" 1s "` 一致失败；`"1,,bad"` 报 source_index=2；
Reject 下 `to_first("1,,")` 返回 1。

### Task 2：轻量 DataType、分类 API 与 feature 边界

**文件：**

- 修改：`rs-datatype/Cargo.toml`
- 创建：`rs-datatype/.rs-ci-cargo-matrix.json`
- 修改：`rs-datatype/src/lib.rs`
- 修改：`rs-datatype/src/datatype/data_type.rs`
- 修改：`rs-datatype/src/datatype/data_type_of.rs`
- 修改：`rs-datatype/tests/datatype/data_type_tests.rs`
- 修改：`rs-datatype/tests/datatype/data_type_of_tests.rs`

**产出接口：**

```rust
impl DataType {
    pub const ALL: [DataType; 27];
    pub const fn is_numeric(self) -> bool;
    pub const fn is_integer(self) -> bool;
    pub const fn is_signed_integer(self) -> bool;
    pub const fn is_unsigned_integer(self) -> bool;
    pub const fn is_float(self) -> bool;
    pub const fn is_big_number(self) -> bool;
}
```

Features：`default = []`；`chrono`、`big-number`、`url`、`json` 控制外部类型的
`DataTypeOf` 实现；`converter` 聚合全部转换依赖。

- [x] 先运行分类和 `--no-default-features` 测试，确认 feature/API 缺失导致失败。
- [x] 实现 `DataType::ALL` 和分类 const 方法，用一张声明式表生成名称映射，消除
  `as_str`、`FromStr` 和测试列表的重复维护。
- [x] 将外部依赖设为 optional，并给 rich `DataTypeOf` impl 添加准确 feature gate。
- [x] 仅在 `converter` feature 下导出 converter 模块和根级转换 API。
- [x] 配置 feature matrix：minimal 执行 check/test/doc；每个单 feature 执行
  check/doc；converter 执行 check/test/doc/clippy。
- [x] 验证：`cargo +1.94.0 test --no-default-features` 和
  `cargo +1.94.0 test --all-features` 均通过。

### Task 3：转换选项、稳定 Serde 与结构化错误

**文件：**

- 创建：`rs-datatype/src/converter/numeric_conversion_policy.rs`
- 创建：`rs-datatype/src/converter/boolean_numeric_policy.rs`
- 创建：`rs-datatype/src/converter/data_conversion_error_kind.rs`
- 创建：`rs-datatype/src/converter/data_format.rs`
- 创建对应 `rs-datatype/tests/converter/*_tests.rs`
- 修改：`rs-datatype/src/converter/{mod.rs,data_conversion_options.rs,boolean_conversion_options.rs,string_conversion_options.rs,collection_conversion_options.rs,duration_conversion_options.rs,duration_unit.rs,blank_string_policy.rs,empty_item_policy.rs,data_conversion_error.rs}`
- 修改：对应现有测试文件。

**产出接口：** `NumericConversionPolicy`、`BooleanNumericPolicy`、`DataFormat`、
`DataConversionErrorKind`、新的 `DataConversionError`、可直接 Serde round-trip 的
全部 options/policies、`DataConversionOptions::default_ref()`。

- [x] 先添加并运行 Serde wire、literal 冲突、错误脱敏和默认策略失败测试。
- [x] 为所有 policy/options 加 `Serialize`、`Deserialize`、`#[serde(default)]` 和
  snake_case enum 名称。
- [x] Boolean options 提供校验构造器，拒绝 true/false literal 集合在当前大小写
  策略下重叠；默认文字仅为 `true`、`false`。
- [x] `normalize()` 返回借用 `&str`；BlankStringPolicy 映射成结构化错误前的内部
  normalization 结果。
- [x] 用 `LazyLock<DataConversionOptions>` 实现 `default_ref()`，所有无 options 的
  转换入口复用该引用，不再逐次分配默认 literal Vec。
- [x] 精确测试所有错误 Display，不包含测试输入中的 secret marker。

### Task 4：拆分并重写 DataConverter 转换核心

**文件：**

- 修改：`rs-datatype/src/converter/data_converter.rs`
- 创建：`rs-datatype/src/converter/data_converter/{numeric.rs,boolean.rs,text.rs,duration.rs,structured.rs,source.rs}`
- 创建：`rs-datatype/tests/converter/data_converter/{numeric_tests.rs,boolean_tests.rs,text_tests.rs,duration_tests.rs,structured_tests.rs,source_tests.rs,mod.rs}`
- 修改：`rs-datatype/tests/converter/mod.rs`
- 精简：`rs-datatype/tests/converter/data_converter_tests.rs`

**产出接口：** `DataConverter` 和 `DataConvertTo<T>` 名称保留，但所有行为遵守目标
契约；`data_converter.rs` 仅保留公开 enum、固有方法和子模块声明。

- [x] 从 Task 1 中选择一个数值 RED 测试，实现内部 ParsedNumber 和 Exact/Lossy
  转换核心，使该测试通过，再逐项扩展整数、浮点、BigInt、BigDecimal 和 Duration。
- [x] Exact 实现双向等价检查；Lossy 实现确定性截断/舍入；错误区分 OutOfRange、
  PrecisionLoss、NonFinite。
- [x] 让 String 数值先进入 ParsedNumber，禁止按目标 primitive 直接 parse。
- [x] 实现所有整数族到 bool，并应用 ZeroOrOne/NonZero/Reject。
- [x] 实现固定格式的 char、chrono、BigInt、BigDecimal、Url、Json、StringMap、
  Duration 文本解析。
- [x] StringMap 反序列化只接受 object + string values，并用自定义 visitor 拒绝
  重复 key。
- [x] 所有 parser 只消费 normalize 结果；删除 bool/Duration 内部 trim。
- [x] 每完成一个目标族，运行对应镜像测试，再运行整个 rs-datatype 测试。

### Task 5：流式集合转换与 lifetime 简化

**文件：**

- 修改：`rs-datatype/src/converter/collection_conversion_options.rs`
- 修改：`rs-datatype/src/converter/scalar_string_data_converters.rs`
- 修改：`rs-datatype/src/converter/data_converters.rs`
- 修改：`rs-datatype/src/converter/data_list_conversion_error.rs`
- 修改：对应四个测试文件。

**产出接口：**

```rust
pub struct ScalarItem<'a> {
    pub source_index: usize,
    pub value: &'a str,
}

pub struct ScalarItems<'a> { /* fields private */ }

impl CollectionConversionOptions {
    pub fn scalar_items<'a>(&'a self, value: &'a str) -> ScalarItems<'a>;
}
```

- [x] 确认原始索引和短路 RED 测试失败。
- [x] 实现惰性 ScalarItems；Reject 在迭代到空项时返回错误，Skip 保留后续原始
  下标。
- [x] `to_vec_with` 直接迭代 ScalarItem 并在元素转换失败时使用 source_index。
- [x] `to_first_with` 在首个保留项后返回，不消费尾部。
- [x] 将 `DataConverters` 简化为 `DataConverters<I>`，移除 PhantomData lifetime；
  转换方法声明自己的 `'a`，使 owned `Vec<&'a str>` 可用。
- [x] 验证空输入、全 Skip、多 delimiter、Unicode、borrowed/owned iterator。

### Task 6：`rs-datatype` 文档、矩阵与版本 0.3.0

**文件：**

- 修改：`rs-datatype/README.md`
- 修改：`rs-datatype/README.zh_CN.md`
- 修改：`rs-datatype/COVERAGE.md`
- 修改：`rs-datatype/src/lib.rs` crate docs
- 修改：`rs-datatype/Cargo.toml`
- 修改：`rs-datatype/Cargo.lock`

- [x] 在 rustdoc/README 增加完整转换矩阵、Exact/Lossy、trim、bool、Duration 和
  feature 示例。
- [x] 将 README 的错误 `BlankStringPolicy::AsNone` 改为真实 API，并让关键示例在
  doctest 中有唯一来源。
- [x] 清理 COVERAGE.md 的 lang module 和过时 CI 示例。
- [x] 将 crate 版本升为 `0.3.0`，README 安装版本升为 `0.3`。
- [x] 运行 doctest、feature matrix、coverage；函数覆盖率保持 100%，每个源码文件
  line/region 高于 CI 阈值。

### Task 7：迁移 `rs-value` 到 0.8.0

**文件：**

- 修改：`rs-value/Cargo.toml`、`Cargo.lock`、README 双语版本引用。
- 修改：`rs-value/src/value/value_converters.rs`
- 修改：`rs-value/src/multi_values/multi_values_converters.rs`
- 修改：`rs-value/src/value_error.rs`
- 修改：相关 value/multi_values/error 测试。

**产出接口：** `ValueError::DataConversion(DataConversionError)` 和包含
`source_index` 的结构化列表转换错误；删除重复 conversion/json 字符串 variants。

- [x] 先添加 typed/text Exact/Lossy 一致性和结构化错误测试，确认旧映射失败。
- [x] 更新 DataConverter/DataConverters 新 API 和 default_ref 使用。
- [x] 删除手工逐 variant 错误复制，保留 DataConversionError 作为 source。
- [x] 将 qubit-datatype 依赖升为 0.3 并启用 converter；crate 版本升 0.8.0。
- [x] 运行 rs-value 全测试和 doctest。

### Task 8：迁移 `rs-serde` 到 0.3.0 与 `rs-progress` 到 0.6.0

**文件：**

- 修改：`rs-serde/Cargo.toml`、Cargo.lock、README 双语。
- 修改：`rs-serde/src/serde/duration_millis.rs`
- 修改：`rs-serde/src/serde/duration_with_unit.rs`
- 修改：对应测试。
- 修改：`rs-progress/Cargo.toml`、Cargo.lock、README 双语及受影响测试。

- [x] 在 rs-serde 先写 1.5ms 仍序列化为 2ms、空白 Duration 文本按新契约拒绝的
  测试。
- [x] MILLISECOND_CONVERSION_OPTIONS 显式设置 NumericConversionPolicy::Lossy，
  不依赖全局默认。
- [x] rs-serde 升 0.3.0，依赖 datatype 0.3 + converter。
- [x] rs-progress 升 0.6.0，依赖 serde 0.3；更新公开 wire 行为测试。

### Task 9：迁移 `rs-config` 到 0.14.0

**文件：**

- 修改：`rs-config/Cargo.toml`、Cargo.lock、README 双语。
- 大幅精简：`rs-config/src/options/config_read_options.rs`
- 修改：`rs-config/src/from/from_config.rs`
- 修改：`rs-config/src/config_error.rs`、`src/config.rs`、`src/from/helpers.rs`
- 修改：options、from、config、error 和各 source 测试。

- [x] 先写 typed `Vec<char/NaiveDate/NaiveDateTime/BigInt/BigDecimal/Duration>`
  回归测试，确认旧 String 中转失败或丢精度。
- [x] 重写 `FromConfig for Vec<T>`，逐 typed Value 转换，不先构造 Vec<String>；仅
  scalar string source 使用 ScalarStringDataConverters。
- [x] 删除 DataConversionOptionsSerde 及所有镜像 enum/struct/mapping/default helper，
  ConfigReadOptions 直接序列化 `DataConversionOptions`。
- [x] 更新结构化错误映射，保留 key 与 source_index，不回显原始配置值。
- [x] 默认 Exact；仅调用方显式 Lossy 时允许有损配置读取。
- [x] 依赖 datatype 0.3、value 0.8、serde 0.3；crate 升 0.14.0。
- [x] 运行 minimal、rich-types 和 source feature matrix。

### Task 10：迁移 `rs-metadata` 到 0.6.0

**文件：**

- 修改：`rs-metadata/Cargo.toml`、Cargo.lock、README 双语。
- 修改：`rs-metadata/src/schema/filter_validation.rs`
- 修改：相关 schema/filter 测试。

- [x] 先写基于 DataType::ALL 的分类完整性测试。
- [x] 删除本地 is_numeric/is_float/is_big_number，调用 DataType const API。
- [x] 更新 ValueError/DataConversionError 模式匹配。
- [x] 依赖 datatype 0.3、value 0.8；crate 升 0.6.0。

### Task 11：迁移配置、重试与元数据第一层下游

**crate 与版本：**

- `rs-retry` 0.15.0 → 0.16.0：config 0.14、serde 0.3；datatype 从 dependencies
  移到 dev-dependencies 0.3。
- `rs-mime` 0.8.0 → 0.9.0：config 0.14。
- `rs-fs` 0.1.0 → 0.2.0：metadata 0.6。

**文件：** 每个 crate 的 Cargo.toml、Cargo.lock、README 双语和因公开错误/类型变化
失败的测试或实现文件。

- [x] 按 retry → mime → fs 顺序，先用本地 patch 运行测试确认编译/模式匹配失败。
- [x] 更新依赖和结构化错误模式，删除对旧错误字符串 variants 的依赖。
- [x] 更新版本和文档引用，分别运行全 feature 测试。

### Task 12：迁移第二层下游

**crate 与版本：**

- `rs-cas` 0.8.0 → 0.9.0：retry 0.16。
- `rs-http` 0.9.0 → 0.10.0：config 0.14、retry 0.16；datatype 移到
  dev-dependencies 0.3。
- `rs-batch` 0.9.0 → 0.10.0：progress 0.6。
- `rs-magika` 0.7.1 → 0.8.0：mime 0.9；dev config 0.14。

- [x] 每个 crate 先运行本地 patch 下的全 feature 测试并保存失败原因。
- [x] 仅修改新公开类型身份和错误契约造成的代码；不顺手改业务行为。
- [x] 更新版本、依赖、Cargo.lock 和双语 README。
- [x] 分别运行全 feature 测试和 doctest。

### Task 13：迁移最终公开依赖闭包

**crate 与版本：**

- `rs-state-machine` 0.5.0 → 0.6.0：cas 0.9。
- `rs-rayon-batch` 0.7.0 → 0.8.0：batch 0.10、progress 0.6。
- `rs-executor` 0.7.0 → 0.7.1：cas 0.9、state-machine 0.6；公开 API 不变。
- `rs-event-bus` 0.7.0 → 0.8.0：retry 0.16、metadata 0.6、executor 0.7.1。

- [x] 逐 crate 先验证失败，再更新依赖和必要模式匹配。
- [x] 对 public re-export/API 身份变化的三个 crate 升 minor；仅内部依赖变化的
  executor 升 patch。
- [x] 更新 Cargo.lock、双语 README 和相应编译测试。

### Task 14：未发布版本本地 patch 与逐 crate CI

**临时文件：** `/tmp/qubit-refactor-cargo-home/config.toml`，不加入任何仓库。

- [x] 创建临时 CARGO_HOME，复用用户现有 cargo bin/registry/git 缓存，并在
  `[patch.crates-io]` 中以绝对路径映射本计划所有新版本 crate。
- [x] 按发布拓扑执行：datatype → value/serde → config/metadata/progress →
  retry/mime/fs → cas/http/batch/magika → state-machine/rayon-batch → executor →
  event-bus。
- [x] 每个被修改 crate 运行：
  `CARGO_HOME=/tmp/qubit-refactor-cargo-home RS_CI_CARGO_HOME_MODE=shared ./align-ci.sh`。
- [x] 紧接着运行：
  `CARGO_HOME=/tmp/qubit-refactor-cargo-home RS_CI_CARGO_HOME_MODE=shared ./ci-check.sh`。
- [x] 对 `rs-execution-services`、`rs-tokio-executor`、`rs-thread-pool`、`rs-task`、
  `rs-rayon-executor` 运行同环境传递集成 CI；若 align-ci 产生变更，单独判断其是否
  属于本次依赖迁移。
- [x] 任一 CI 失败时按 systematic-debugging：记录首个根因，补失败测试，再修复；
  不跳过 package、coverage 或 audit 步骤。

### Task 15：最终审查、版本一致性与交付

**检查范围：** 本计划列出的全部独立 Git 仓库。

- [x] 使用 `rg` 核对旧版本依赖和旧错误 variant 已全部消失。
- [x] 分别运行 `git status --short` 和 `git --no-pager diff --check`，确认没有覆盖
  用户原有未跟踪文档，没有跨仓库混合差异。
- [x] 由独立 reviewer 对照本计划审查：转换矩阵、脱敏错误、feature 最小构建、
  typed Vec、Serde wire、版本拓扑和每 crate CI 证据。
- [x] 修复 reviewer 的 Critical/Important 问题，并重新运行受影响 crate 的
  align-ci.sh 与 ci-check.sh。
- [x] 最终报告每个 crate 的版本、变更类别、测试/CI 结果，以及仍需按拓扑发布的
  顺序；不自动 commit 或 push。

---

## 实施记录（2026-07-13）

### 版本与发布拓扑

本计划已完成 17 个发布 crate 的迁移：

1. `qubit-datatype` 0.3.0；
2. `qubit-value` 0.8.0、`qubit-serde` 0.3.0；
3. `qubit-config` 0.14.0、`qubit-metadata` 0.6.0、`qubit-progress` 0.6.0；
4. `qubit-retry` 0.16.0、`qubit-mime` 0.9.0、`qubit-fs` 0.2.0；
5. `qubit-cas` 0.9.0、`qubit-http` 0.10.0、`qubit-batch` 0.10.0、
   `qubit-magika` 0.8.0；
6. `qubit-state-machine` 0.6.0、`qubit-rayon-batch` 0.8.0；
7. `qubit-executor` 0.7.1；
8. `qubit-event-bus` 0.8.0。

发布时必须保持以上拓扑顺序。最终 Cargo.toml 使用正常版本依赖，不含本地依赖
`path` 或 `[patch.crates-io]`。

### 统一验证环境

- 使用临时 `CARGO_HOME=/tmp/rs-datatype-global-cargo-home`，通过绝对路径 patch
  映射上述 17 个未发布版本；配置只存在于 `/tmp`，未写入仓库。
- 为满足未发布的真实依赖闭包，临时环境额外 patch 了 `qubit-clock` 0.9.0 和
  `qubit-codec-misc` 0.2.0。首次 `rs-retry` 验证暴露缺少 clock patch，补齐后通过。
- 17 个发布 crate 均在同一环境按 `align-ci.sh`、`ci-check.sh` 顺序完成验证，
  每个 crate 的 CI 11 个阶段全部通过，包括 package、coverage 和 audit。
- `rs-execution-services`、`rs-tokio-executor`、`rs-thread-pool`、`rs-task`、
  `rs-rayon-executor` 五个传递下游也在统一环境通过完整 align/CI。

### 传递 lock 清理

统一全量 patch 会让纯验证下游的 Cargo.lock 记录无关 unused patch。最终为五个传递
下游改用只包含 executor 真实闭包的最小临时 patch：executor、cas、state-machine、
retry、clock、datatype、serde，并重新解析 lock。清理后五个 lock 均不含
`[[patch.unused]]`：

- `rs-execution-services`、`rs-thread-pool`、`rs-task`、`rs-rayon-executor` 将
  `crossbeam-epoch` 0.9.18 更新为 0.9.20，修复 RUSTSEC-2026-0204；
- `rs-tokio-executor` 的依赖图不含 `crossbeam-epoch`；
- 五个 crate 均在最小 patch 环境重新通过 all-features test 和 cargo audit。

这些 crate 仅用于传递集成验证，不因 Cargo.lock 更新加入本次发布列表。

### 最终审查修复

- 清除了 `rs-datatype` 四个 Markdown 文件的 EOF 多余空行。
- 为 `boolean_literal_conflict_error.rs`、`scalar_item.rs`、
  `scalar_item_error.rs`、`scalar_items.rs` 增加镜像行为测试并完成测试装配；targeted、
  all-features、doctest、align 和完整 CI 均通过。
- `rs-config` 的 datatype 依赖以及 `rs-http`、`rs-retry` 的 datatype
  dev-dependency 均显式启用 `converter` feature，避免依赖默认 feature 偶然生效。
- `rs-config` 的 char 反序列化改用共享 `DataConverter<char>`，产生结构化、
  value-free 的 `InvalidSyntax`；Serde `Message` 路径在公开错误边界统一替换为固定
  脱敏消息。char 与未知 enum 的 secret-marker 端到端 RED 测试先失败、修复后通过，
  config 全量测试及 CI 通过。
- 审查修复后，`rs-datatype`、`rs-config`、`rs-http`、`rs-retry` 均重新运行
  align-ci 和 ci-check，全部通过。

### 已有文档状态

最终审查确认 `rs-value`、`rs-config`、`rs-serde` 的用户文档在本轮最终审查开始前
已经存在于各自仓库的 HEAD。实施过程未 reset、amend、删除或覆盖这些文档；该既有
状态保留并由最终交付报告明确说明。

---

## 计划自检

- 评审中的转换矩阵、有损策略、单点 normalize、集合索引、to_first、依赖分层、
  options Serde、结构化错误、性能分配、DataType 分类、文档和测试意见均有对应任务。
- 所有新增 API 在本计划中给出确定签名或确定语义，不含待定实现项。
- 下游版本遵循公开类型身份传播：datatype 0.3 → value/serde → config/metadata/
  progress → retry/mime/fs → cas/http/batch/magika → state-machine/rayon-batch →
  executor/event-bus。
- 实现阶段不并行派发多个写代码 subagent；可并行只读审查和最终验证，但共享文件
  写入始终串行。
