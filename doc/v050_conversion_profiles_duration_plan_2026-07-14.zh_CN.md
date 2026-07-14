# `rs-datatype` 0.5 转换配置与错误模型重构实施计划

> **供自动化执行者使用：** 必须按任务顺序执行。每个行为变更先写失败测试并确认
> RED，再修改生产代码并确认 GREEN。任务状态使用复选框跟踪。

**目标：** 为 0.5 增加语义明确的严格/有损转换预设，拆分 Duration 输入输出策略，
修复 Unicode 微秒后缀，结构化 Duration 溢出和空集合错误，并补充属性测试。

**架构：** `DataConversionOptions` 负责组合命名 profile；Duration 的数值输入单位、
无后缀字符串策略和输出单位分别建模；低层 Duration 溢出使用独立错误，转换层继续
映射为稳定的 `InvalidValueReason::OutOfRange`；空集合使用独立错误枚举值，不伪造源类型。

**技术栈：** Rust 1.94、Edition 2024、Serde、thiserror 2.0、proptest 1、项目 `.rs-ci`。

## 全局约束

- 保留用户手动设置的 `qubit-datatype` 版本 `0.5.0`，不回退版本。
- 允许破坏性 API 变更，不提供旧字段或旧 builder 的兼容层。
- `DataConversionOptions::default()` 的现有字段语义保持不变。
- `DataConversionOptions::lossy()` 启用有损数值转换，并裁剪字符串前后空白；不得改变
  blank、Boolean、Collection 和 Duration 的其他默认策略。
- 不修改 `rs-value`；不拆分 `converter` Cargo feature。
- 新增或变更的公开 API 必须有完整 rustdoc 和精确测试。
- 不执行 Git 写操作。
- 最终依次运行 `./align-ci.sh` 和 `./ci-check.sh`。

---

### Task 1：命名转换 profile

**文件：**

- 修改：`tests/converter/options/data_conversion_options_tests.rs`
- 修改：`src/converter/options/data_conversion_options.rs`
- 修改：`src/lib.rs`
- 修改：`src/converter/data_converter.rs`

**接口：**

- 产生：`DataConversionOptions::strict() -> Self`
- 产生：`DataConversionOptions::lossy() -> Self`
- 保持：`DataConversionOptions::default()` 与 `strict()` 完全相等

- [x] 在 options 测试中调用尚不存在的 `strict()`、`lossy()`，精确断言 strict 等于
  default，lossy 仅将 `numeric_policy` 设为 `Lossy`、将 `string.trim` 设为 `true`。
- [x] 运行 `cargo +1.94.0 test --all-features test_data_conversion_options_profiles`，确认
  因关联函数不存在而 RED。
- [x] 实现两个构造函数；移除 `DataConversionOptions` 的派生 `Default`，改为手写
  `Default` 并委托给 `strict()`。
- [x] 在两个构造函数 rustdoc 中逐字段说明 Numeric、String、Boolean、Collection、
  Duration 的预设值和适用场景，并提供可执行示例。
- [x] 将 crate 根文档和 `DataConverter` 示例中的重复 builder 改为 `lossy()`。
- [x] 重跑 targeted test 和 doctest，确认 GREEN。

### Task 2：拆分 Duration 策略并修复微秒解析

**文件：**

- 新增：`src/converter/options/suffixless_duration_policy.rs`
- 新增：`tests/converter/options/suffixless_duration_policy_tests.rs`
- 修改：`src/converter/options/duration_conversion_options.rs`
- 修改：`src/converter/options/mod.rs`
- 修改：`src/converter/mod.rs`
- 修改：`src/lib.rs`
- 修改：`src/converter/data_converter/duration.rs`
- 修改：`src/converter/data_converter/numeric.rs`
- 修改：`tests/converter/options/duration_conversion_options_tests.rs`
- 修改：`tests/converter/data_converter/duration_tests.rs`
- 修改：对应 `tests/**/mod.rs`
- 修改：`rs-serde`、`rs-config` 中直接使用旧 Duration builder/字段的代码与测试；
  不修改 `rs-value`。

**接口：**

- 产生：`SuffixlessDurationPolicy::{Reject, Assume(DurationUnit)}`
- 产生字段：`numeric_input_unit`、`suffixless_string_policy`、`output_unit`、
  `append_unit_suffix`
- 产生 builder：`with_numeric_input_unit`、`with_suffixless_string_policy`、
  `with_output_unit`、`with_append_unit_suffix`
- 删除字段：`unit`
- 删除 builder：`with_unit`

- [x] 先将 Duration options 测试改为新字段和 builder，并增加 `Reject` 拒绝无后缀文本、
  `Assume(unit)` 接受无后缀文本、输入输出使用不同单位的测试。
- [x] 增加 `1us`、`1µs`、`1μs` 均转换为一微秒的回归测试。
- [x] 运行 Duration targeted tests，确认因新 API 不存在且 Unicode 后缀失败而 RED。
- [x] 新增并导出 `SuffixlessDurationPolicy`，实现 Serde snake_case 表示和完整 rustdoc。
- [x] 重构 `DurationConversionOptions`；默认值仍等价于旧配置：三个单位决策均使用毫秒，
  输出附带后缀。
- [x] 数值到 Duration 读取 `numeric_input_unit`，无后缀字符串读取
  `suffixless_string_policy`，Duration 到数值/字符串读取 `output_unit`。
- [x] 删除阻止 Unicode 微秒后缀到达 `DurationUnit::from_suffix()` 的 ASCII 预检查，
  同时保持非法标点为 `InvalidSyntax`、未知字母单位为 `UnsupportedDurationUnit`。
- [ ] 迁移 `rs-serde`、`rs-config` 的直接调用，运行三个 crate 的相关 targeted tests，
  确认 GREEN。代码迁移已完成；`rs-serde` 的 35 个测试已通过，`rs-config` 必须等待
  用户在另一线程完成 `rs-value` 0.5 迁移后才能解析并编译完整依赖图。

### Task 3：结构化 Duration 溢出和空集合错误

**文件：**

- 新增：`src/converter/error/duration_overflow_error.rs`
- 新增：`tests/converter/error/duration_overflow_error_tests.rs`
- 修改：`src/converter/options/duration_unit.rs`
- 修改：`src/converter/error/data_conversion_error.rs`
- 修改：`src/converter/error/mod.rs`
- 修改：`src/converter/mod.rs`
- 修改：`src/lib.rs`
- 修改：`src/converter/data_converters.rs`
- 修改：`tests/converter/options/duration_unit_tests.rs`
- 修改：`tests/converter/error/data_conversion_error_tests.rs`
- 修改：`tests/converter/data_converters_tests.rs`
- 修改：对应 `tests/**/mod.rs`

**接口：**

- 产生：`DurationOverflowError`
- 变更：`DurationUnit::duration_from_u64/u128` 返回
  `Result<Duration, DurationOverflowError>`
- 产生：`DataConversionError::EmptyCollection { to: DataType }`

- [x] 先修改 DurationUnit 测试以精确匹配 `DurationOverflowError`，并修改空集合测试以
  匹配 `EmptyCollection { to }`；运行后确认 RED。
- [x] 实现并导出 `DurationOverflowError`，用 `thiserror` 提供不含输入值的稳定文本。
- [x] 将 `DurationUnit` 内部的字符串溢出错误全部替换为该错误类型。
- [x] 为 `DataConversionError` 增加 `EmptyCollection`，使 `DataConverters::to_first_with`
  不再构造 `from == to` 的伪造 `Missing`。
- [x] 更新 rustdoc 和错误 Display 测试，运行 targeted tests 确认 GREEN。

### Task 4：属性测试、文档迁移与最终验证

**文件：**

- 修改：`Cargo.toml`
- 修改：`tests/converter/data_converter/duration_tests.rs`
- 修改：`tests/converter/data_converter/numeric_tests.rs`
- 修改：`tests/converter/options/boolean_conversion_options_tests.rs`
- 修改：`README.md`、`README.zh_CN.md` 和 `doc/**` 中描述当前 API 的文档。

- [x] 增加 `proptest = "1.0"` dev-dependency。
- [x] 增加属性测试：所有可构造 Duration unit count 在相同输出单位下精确格式化并解析
  回原值；任意字符串进入 Numeric/Boolean/Duration parser 均不 panic；任意 Boolean
  literal 组合经成功构造后始终通过 `validate()`。
- [x] 先运行新增属性测试，确认其中依赖新 API 的测试在生产代码完成前 RED；生产代码
  完成后重跑确认 GREEN。
- [x] 全文迁移 `rs-datatype` 和允许修改的下游 crate 中旧 Duration API、重复 lossy
  builder 及过时 rustdoc；保留 `rs-value` 不变。
- [x] 运行 `cargo +1.94.0 test --no-default-features` 和
  `cargo +1.94.0 test --all-features`。
- [x] 在 `rs-datatype` 运行 `./align-ci.sh`，再运行 `./ci-check.sh`。
- [ ] 对发生源码改动的 `rs-config`、`rs-serde` 分别运行 `./align-ci.sh` 和
  `./ci-check.sh`。两个脚本均已实际运行；`rs-serde` 在发布 `qubit-datatype 0.5`
  前止于 package verification，`rs-config` 因 crates.io 尚无 0.5 且 `rs-value` 尚未迁移
  而止于依赖解析。
- [x] 对照本计划逐项复核公开导出、Rustdoc、测试和版本 `0.5.0`，记录完成结果。

## 完成记录

- `rs-datatype`：`./align-ci.sh` 与 `./ci-check.sh` 全部通过；全 feature 共 130 个测试、
  19 个 doctest，通过无默认 feature 的 20 个测试和 5 个 doctest；额外以
  `-D missing_docs` 构建公开文档成功。
- `rs-serde`：迁移后 35 个测试、文档和 CI 前八个阶段通过。由于
  `qubit-datatype 0.5` 尚未发布，`cargo package` 无法从 crates.io 解析该版本。
- `rs-config`：测试源码已迁移并通过 rustfmt 检查；依赖解析等待 `rs-value` 的独立
  迁移和 `qubit-datatype 0.5` 发布，因此未虚报为通过。
