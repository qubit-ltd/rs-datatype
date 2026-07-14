# `rs-datatype` 0.5 发布前修正实施计划

> **供自动化执行者使用：** 按任务顺序执行，行为变更必须先确认 RED，再实现 GREEN。
> 本任务不执行 Git 写操作。

**目标：** 修正 0.5 的公开文档和 docs.rs 构建配置，明确拒绝旧 Duration 配置字段，
并以完整 Serde 测试和发布流水线证明 crate 可以发布。

**架构：** `DurationConversionOptions` 继续使用 Serde 派生，但增加未知字段拒绝；测试以
JSON wire 行为覆盖完整结构、默认补齐、全部枚举值、旧字段和任意未知字段。README 仅
描述当前公开 API，docs.rs 使用 all-features 构建完整转换文档。

**技术栈：** Rust 1.94、Serde、serde_json、项目 `.rs-ci`。

## 全局约束

- 保持 `qubit-datatype` 版本 `0.5.0`。
- 不兼容旧 `DurationConversionOptions::unit`，反序列化时必须明确报错。
- 不调整 Cargo `include` 列表。
- 从双语 README 删除 Coverage 文档链接。
- 不修改 `rs-value` 或其他下游 crate。

---

### Task 1：Duration 配置 Serde 回归测试

**文件：**

- 修改：`tests/converter/options/duration_conversion_options_tests.rs`

- [x] 增加完整 wire 测试，断言四个字段的精确 JSON 表示和反序列化往返。
- [x] 增加部分字段测试，断言缺失字段由 `Default` 补齐。
- [x] 覆盖七种 `DurationUnit`、`Reject`、七种 `Assume(unit)` 和 suffix 开关。
- [x] 增加旧 `unit` 字段与任意未知字段被拒绝的测试，并检查错误包含字段名。
- [x] 运行定向测试，确认拒绝旧字段的测试因当前实现忽略未知字段而 RED。

### Task 2：最小实现与发布文档修正

**文件：**

- 修改：`src/converter/options/duration_conversion_options.rs`
- 修改：`Cargo.toml`
- 修改：`README.md`
- 修改：`README.zh_CN.md`

- [x] 为 `DurationConversionOptions` 增加 Serde 未知字段拒绝。
- [x] 在 Cargo metadata 中配置 docs.rs 使用 all features；保持 `include` 原样。
- [x] 双语 README 将错误模型改为四类并使用真实枚举名。
- [x] 双语 README 删除 Coverage 文档链接。
- [x] 重跑定向测试，确认全部 GREEN。

### Task 3：完整验证和发布门禁

- [x] 运行 `./align-ci.sh`。
- [x] 运行 `./ci-check.sh`。
- [x] 从 `cargo package` 生成包运行全 feature 测试和 `-D missing_docs` 文档构建。
- [x] 运行 `cargo publish --dry-run --allow-dirty`。
- [x] 独立复核需求覆盖、公开文档、Serde 行为和发布元数据。

## 完成记录

- TDD RED：定向测试 5 个通过、2 个失败，证明旧 `unit` 和未知字段会被静默忽略。
- TDD GREEN：增加未知字段拒绝后，定向测试 7/7 通过。
- 完整 CI：`./align-ci.sh` 与 `./ci-check.sh` 通过；全 feature 135 个测试、
  19 个 doctest 全部通过，覆盖率和安全审计通过。
- 发布包：从生成包运行全 feature 测试及 `-D warnings -D missing_docs` 文档构建通过；
  `cargo publish --dry-run --allow-dirty` 通过。
- 独立审查：Critical、Important、Minor 均为零，结论为 `APPROVED`。
