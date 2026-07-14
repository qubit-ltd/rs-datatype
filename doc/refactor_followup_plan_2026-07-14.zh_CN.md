# `rs-datatype` 后续结构与 rustdoc 重构实施计划

> **供自动化执行者使用：** 逐任务执行；新增行为和公开 API 先写失败测试并确认
> RED，再修改生产代码。任务状态使用复选框跟踪。

**目标：** 落实 2026-07-14 确认的 API、错误、options 目录和 rustdoc 重构设计。

**架构：** `converter/options` 封装全部 profile 与策略，`converter/error` 封装全部
错误及原因，`data_converter` 只负责源值包装和转换分派；根模块维持显式 re-export。

**技术栈：** Rust 1.94、Edition 2024、Serde、thiserror 2.0、项目 `.rs-ci`。

## 全局约束

- 保留工作区已有 `#[inline]` 调整等用户改动，不覆盖或回退。
- 不执行任何 Git 写操作。
- 源码文件移动必须同步移动镜像测试并更新 `tests/**/mod.rs`。
- 全部新增或变更的公开 API 必须有精确测试和完整 rustdoc。
- 最终依次运行 `./align-ci.sh` 和 `./ci-check.sh`。

---

### Task 1：锁定破坏性 API 和 profile 契约

**测试文件：**

- `tests/datatype/data_type_of_tests.rs`
- `tests/converter/options/*_tests.rs`（移动后路径）
- `tests/converter/error/*_tests.rs`（移动后路径）
- `tests/converter/data_converter_tests.rs`

- [ ] 添加无 `json` feature 的 `HashMap<String, String>: DataTypeOf` 编译/行为测试。
- [ ] 为五个 options/policy 的 `env_friendly()` 添加精确字段断言。
- [ ] 将错误模式匹配测试改为 `InvalidValue { reason: InvalidValueReason }`。
- [ ] 将 Boolean 默认 literal 测试改为关联常量，并固定常量内容。
- [ ] 运行 targeted tests，确认因新 API 不存在而 RED。

### Task 2：错误模型、thiserror 与 Result 签名

**生产文件：**

- 移动至 `src/converter/error/` 的全部错误文件。
- 修改 `Cargo.toml`、`src/converter/mod.rs`、`src/lib.rs`。
- 修改全部 converter 生产文件的返回类型与错误构造。
- 删除 `data_conversion_result.rs`、`data_list_conversion_result.rs`。

- [ ] 增加 optional `thiserror = "2.0"`，由 `converter` feature 启用。
- [ ] 实现 `InvalidValueReason` 与 `DataConversionError::InvalidValue`。
- [ ] 用 `thiserror::Error` 替换手写 Display/Error，并保持现有错误文本与脱敏保证。
- [ ] 删除 free `invalid()`，调用处直接构造 `InvalidValue`。
- [ ] 删除两个 Result alias，所有签名展开为标准 `Result`。
- [ ] 运行错误、转换和 doctest targeted tests，确认 GREEN。

### Task 3：options 目录、profile 与 Boolean wire 校验

**生产文件：**

- 移动全部 options/policies 至 `src/converter/options/`。
- 创建 `src/converter/options/mod.rs`。
- 修改 `BooleanConversionOptions` 和 `DataConversionOptions`。

- [ ] 为 String、Collection、Duration、Numeric policy 实现 `env_friendly()`。
- [ ] 让组合 options 只调用各字段的 `env_friendly()`。
- [ ] 增加公开默认 Boolean literal 关联常量，并删除两个 free default 函数。
- [ ] 将 Serde helper 改名为 `UncheckedBooleanConversionOptions`，继续通过 `try_new()`。
- [ ] 同步移动镜像测试和模块装配，运行 options 全测试确认 GREEN。

### Task 4：转换核心辅助函数与模块引用整理

**生产文件：**

- 修改 `src/converter/data_converter.rs` 与 `src/converter/data_converter/*.rs`。
- 创建私有字符串来源辅助模块（若 `normalize()` 无法自然归入现有模块）。

- [ ] 将 `normalize()` 从根文件移动到职责明确的私有模块。
- [ ] 保留 `missing/unsupported/invalid` method 的用途，但按最终错误命名更新。
- [ ] 更新所有 options/error import，禁止 `use super::*`。
- [ ] 运行 all-features test，确认转换行为没有变化。

### Task 5：全源码 rustdoc 补全

**范围：** `src/**/*.rs` 共 37 个现有源文件及本计划移动后的对应文件。

- [ ] 为 DataType/DataTypeOf/DataConverter/DataConvertTo/DataConverters/
  ScalarStringDataConverters 补职责、场景、关系、约束与可执行示例。
- [ ] 为所有 options、policy、error、scalar iterator 类型补完整类型文档。
- [ ] 为每个公开 variant/字段补独立文档。
- [ ] 为所有公共固有方法补参数、返回值、错误条件和关键行为。
- [ ] 为私有辅助函数补职责、参数、返回值、错误和不变量。
- [ ] 在 crate root 增加 `#![deny(missing_docs)]`。
- [ ] 运行 `RUSTDOCFLAGS='-D warnings' cargo +1.94.0 doc --all-features --no-deps`。
- [ ] 运行 doctest，确认所有示例可执行。

### Task 6：最终验证与审查

- [ ] 运行 `cargo +1.94.0 test --no-default-features`。
- [ ] 运行 `cargo +1.94.0 test --all-features`。
- [ ] 运行 feature matrix 和 Clippy。
- [ ] 运行 `./align-ci.sh`。
- [ ] 运行 `./ci-check.sh`，确认 11 个阶段全部通过。
- [ ] 运行 `git diff --check`，确认用户原有改动仍然保留。
- [ ] 对照设计逐项审查，修复 Critical/Important 问题。

