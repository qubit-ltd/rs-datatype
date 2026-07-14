# Code Coverage Guide

This crate uses `cargo-llvm-cov` through the repository's pinned CI scripts.
Coverage runs with all features so the optional conversion engine and every
rich type mapping are measured.

## Requirements

Install `cargo-llvm-cov`; the scripts install or verify the pinned LLVM tools
for Rust 1.94:

```bash
cargo install cargo-llvm-cov
```

## Commands

```bash
./coverage.sh text
./coverage.sh json --clean
./coverage.sh html
./coverage.sh lcov
./coverage.sh cobertura
./coverage.sh all --clean
```

Generated artifacts are written below `target/llvm-cov/`. JSON and `all`
runs enforce the same per-source thresholds as CI:

- functions: at least 100%
- lines: greater than 95%
- regions: greater than 95%

Tests, benches, and examples are excluded by [`.llvm-cov.toml`](.llvm-cov.toml).
Do not copy standalone GitHub Actions snippets from this document; the
authoritative workflow is `./ci-check.sh` and the checked-in `.rs-ci`
configuration.

## Feature-specific inspection

The default coverage run uses `--all-features`. For local diagnosis, override
the script environment:

```bash
COVERAGE_ALL_FEATURES=0 COVERAGE_NO_DEFAULT_FEATURES=1 ./coverage.sh text
COVERAGE_ALL_FEATURES=0 COVERAGE_FEATURES=converter ./coverage.sh text
```

## Interpreting failures

A JSON run prints every source file below a threshold. Add deterministic tests
for real public or internal behavior; do not exclude production files merely to
raise the percentage. Clean stale instrumentation with `--clean` when source
locations or feature sets change.
