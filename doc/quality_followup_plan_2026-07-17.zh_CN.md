# `rs-datatype` 质量改进实施计划

## Task 1：规则与回归测试

- [x] 修正 `reviewing-rust-code-style` 的 SPDX 文件头与 `# Parameters` 规则。
- [x] 增加 converter-only `StringMap` 恒等转换测试并确认 RED。
- [x] 为四类尚未严格校验的 options 增加未知字段测试并确认 RED。

## Task 2：行为修复

- [x] 将 `HashMap<String, String>` 目标转换从 `json` feature 中解耦。
- [x] 为 options 的 Serde wire 类型增加 `deny_unknown_fields`。
- [x] 运行定向测试，确认回归测试 GREEN。

## Task 3：私有模块重构

- [x] 将 `ParsedNumber` 与数值转换阶段拆入 `data_converter/internal/numeric/`。
- [x] 将 `StringMapVisitor` 移入 `data_converter/internal/`。
- [x] 将 `UncheckedBooleanConversionOptions` 移入 `options/internal/`。
- [x] 保持 `numeric.rs` 为薄分派层，运行 numeric/structured/options 全测试。

## Task 4：文档与源码规范

- [x] 统一 `src`、`tests` 的五行 SPDX 文件头。
- [x] 将 Rustdoc 参数标题统一为 `# Parameters`，补齐变更范围内的缺失契约。
- [x] 新增中英文用户手册并校验代码示例。
- [x] 精简中英文 README，链接对应用户手册并与 feature/API 对齐。

## Task 5：最终验证

- [x] 运行格式化、converter-only、all-features、doctest 和 Clippy。
- [x] 依次运行 `./align-ci.sh`、`./ci-check.sh`；仅在 CI 报告覆盖率不足时运行
  `./coverage.sh json`。
- [x] 运行 `git diff --check` 并完成结构、文档和兼容性自审。
