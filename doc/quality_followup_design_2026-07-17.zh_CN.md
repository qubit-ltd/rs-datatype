# `rs-datatype` 质量改进设计

## 目标

在保持现有公共 API 和转换语义的前提下，修复 `converter` 单独启用时
`StringMap` 不能完成恒等转换的问题，收紧 options 的 Serde 输入契约，降低数值转换
模块复杂度，并让源码文档、README 与双语用户手册保持一致。

## 已确认设计

- `HashMap<String, String>` 的目标转换属于 `converter` 核心能力，不依赖 `json`；
  JSON 字符串解析和 `serde_json::Value` 转换仍由 `json` feature 控制。
- `DataConversionOptions`、`StringConversionOptions`、
  `CollectionConversionOptions`、Boolean 的反序列化辅助类型都拒绝未知字段；
  Duration 已有的严格行为保持不变。
- `numeric.rs` 仅保留目标类型 trait 实现和薄分派；整数、浮点、大数与语法解析逻辑
  拆入 `data_converter/internal/numeric/` 私有子模块。
- `ParsedNumber`、`StringMapVisitor`、`UncheckedBooleanConversionOptions` 分别移入其
  逻辑目录的 `internal/` 子目录；不扩大可见性，不改变公共路径。
- `src/**/*.rs` 与 `tests/**/*.rs` 使用 `structured.rs` 的五行 SPDX 文件头；Rustdoc
  参数章节统一使用 `# Parameters`，并补齐适用的参数、返回、错误和 panic 契约。
- 新增 `doc/user_guide.md` 与 `doc/user_guide.zh_CN.md`，完整说明类型、数值比较、转换器、
  options、集合工具、feature 和扩展方式；双语 README 简化为入口、快速示例和手册链接。

## 兼容性与验证

- 不增加公开类型，不移动公开导出，不修改错误枚举或转换策略语义。
- 先增加回归测试并确认旧实现失败，再实施行为修复。
- 结构重构前后运行现有 numeric、structured、options 测试，并验证 converter-only、
  all-features、doctest、Clippy 和项目 CI。
- 不执行 `git add`、`git commit` 或 `git push`。
