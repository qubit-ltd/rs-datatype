# Qubit Datatype

面向 Qubit Rust 项目的运行时数据类型描述与类型转换工具库。

`qubit-datatype` 提供统一的 `DataType` 枚举、通过 `DataTypeOf` 实现的编译期类型映射，以及 `DataConverter`、`DataConverters` 等可复用转换工具。

## 功能

- 为受支持的标量和结构化值提供运行时类型描述。
- 通过 `DataTypeOf` 将 Rust 类型映射到 `DataType`。
- 支持单值和批量值转换。
- 提供字符串、布尔值、集合字符串输入等转换选项。
- 转换错误中保留来源类型和目标类型的结构化信息。

## 安装

```toml
[dependencies]
qubit-datatype = "0.1.0"
```

## 快速开始

```rust
use qubit_datatype::{DataConverter, DataType, DataTypeOf};

assert_eq!(i32::DATA_TYPE, DataType::Int32);

let port: u16 = DataConverter::from("8080")
    .to()
    .expect("valid port should convert");
assert_eq!(port, 8080);
```

## 模块

- `datatype`：`DataType`、`DataTypeOf` 和类型解析错误。
- `converter`：`DataConverter`、`DataConverters`、转换选项和转换错误。

常用类型已在 crate 顶层重新导出。
