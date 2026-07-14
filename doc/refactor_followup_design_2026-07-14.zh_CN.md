# `rs-datatype` 后续结构与 rustdoc 重构设计

## 目标

在不改变既定转换语义的前提下，进一步提高 `rs-datatype` 的公共 API 可读性、
源码目录边界和 rustdoc 完整性，并删除没有实际价值的结果别名及错误构造辅助函数。

## 已确认设计

- `HashMap<String, String>` 的 `DataTypeOf` 不再受 `json` feature 控制。
- 不新增从 `DataType` 构造 `DataConverter` 的 API；没有源值时继续显式使用
  `DataConverter::Empty(DataType)`。
- 删除 `DataConversionResult<T>` 与 `DataListConversionResult<T>`，所有签名直接展示
  `Result<T, DataConversionError>` 或 `Result<T, DataListConversionError>`。
- `DataConversionError::Invalid` 重命名为 `InvalidValue`，字段 `kind` 重命名为
  `reason`，`DataConversionErrorKind` 重命名为 `InvalidValueReason`。
- `DataConversionError`、`InvalidValueReason` 及同模块错误使用可选 `thiserror 2.0`
  实现稳定、脱敏的错误文本。
- 删除 `data_converter.rs` 中的 free `invalid()`；底层转换代码直接构造
  `DataConversionError::InvalidValue`，有 `self` 的路径保留私有错误构造方法。
- `normalize()` 保留单点字符串规范化职责，随转换核心移动到清晰的私有模块。
- 每个 options/policy 都提供 `env_friendly()`；即使当前等价于 `default()`，也将
  环境变量 profile 的决策封装在类型自身。
- `BooleanConversionOptionsDef` 重命名为
  `UncheckedBooleanConversionOptions`，反序列化后继续通过 `try_new()` 校验。
- Boolean 默认 literals 改为 `BooleanConversionOptions` 的公开关联常量
  `&[&str]`。
- options/policies 移入 `converter/options/`，errors 移入 `converter/error/`；
  根模块继续显式 re-export，保持除已确认破坏项外的公共导入路径。

## rustdoc 标准

- 扫描 `rs-datatype/src/**` 全部生产代码。
- 核心类型说明职责、适用场景、与其他核心类型的关系、所有权/生命周期、默认契约，
  并提供可执行示例。
- 公共方法说明参数、返回值、精确错误条件和重要行为；简单 getter 可简写但不能缺失
  返回语义。
- 私有辅助函数说明职责、参数、返回值、错误及不变量，不机械添加无价值示例。
- 标准 trait 方法只记录本实现的额外语义。
- crate root 增加 `#![deny(missing_docs)]`，并以 all-features rustdoc 验证全部公开项。

## 非目标

- 不改变 Exact/Lossy、Boolean、Duration、集合索引或字符串规范化语义。
- 不新增动态 `DataType -> DataConverter` 默认值工厂。
- 不修改 `rs-datatype` 之外的 crate，除非编译证明确有破坏性 API 迁移需要；当前
  workspace 搜索显示两个 Result alias 没有下游 Rust 调用。
- 不执行 `git add`、`git commit` 或 `git push`。
