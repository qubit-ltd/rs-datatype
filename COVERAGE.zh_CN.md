# 代码覆盖率指南

本 crate 通过仓库固定的 CI 脚本使用 `cargo-llvm-cov`。默认覆盖率运行启用全部
feature，因此会统计可选转换引擎和所有 rich type 映射。

## 准备

安装 `cargo-llvm-cov`；脚本会为 Rust 1.94 检查或安装固定的 LLVM tools：

```bash
cargo install cargo-llvm-cov
```

## 命令

```bash
./coverage.sh text
./coverage.sh json --clean
./coverage.sh html
./coverage.sh lcov
./coverage.sh cobertura
./coverage.sh all --clean
```

产物写入 `target/llvm-cov/`。`json` 和 `all` 会执行与 CI 相同的逐源码文件
阈值：

- 函数覆盖率不低于 100%
- 行覆盖率高于 95%
- region 覆盖率高于 95%

测试、benchmark 和 example 由 [`.llvm-cov.toml`](.llvm-cov.toml) 排除。
权威 CI 入口是 `./ci-check.sh` 和仓库内 `.rs-ci` 配置，不在本文维护独立的
GitHub Actions 示例。

## 指定 feature 检查

默认使用 `--all-features`。本地诊断可覆盖脚本环境变量：

```bash
COVERAGE_ALL_FEATURES=0 COVERAGE_NO_DEFAULT_FEATURES=1 ./coverage.sh text
COVERAGE_ALL_FEATURES=0 COVERAGE_FEATURES=converter ./coverage.sh text
```

## 处理失败

JSON 运行会列出低于阈值的源码文件。应为真实的公开或内部行为增加稳定测试，不要
为了提高百分比而排除生产代码。源码位置或 feature 集合变化后，可用 `--clean`
清理旧插桩数据。
