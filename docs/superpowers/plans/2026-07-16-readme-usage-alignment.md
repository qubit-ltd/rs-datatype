# rs-datatype README Usage Alignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add representative current-API examples to both rs-datatype READMEs and keep their structure and technical content equivalent.

**Architecture:** Treat `README.md` as the English presentation and `README.zh_CN.md` as its section-for-section Chinese counterpart. Add the same four usage scenarios to each, then verify every Rust block against an all-features build and compare both heading trees mechanically.

**Tech Stack:** Markdown, Rust 2024, `rustdoc`, `qubit-datatype` public APIs.

## Global Constraints

- Modify only the two READMEs plus this design/plan documentation.
- Keep English and Chinese heading order and example behavior equivalent.
- Use only current public APIs; do not mention removed conversion traits.
- Examples that use the conversion engine require the `converter` feature.
- Numeric comparison remains available without the `converter` feature; big-number examples are not required.
- Do not commit unless separately authorized.

---

### Task 1: Add Matching Typical-Usage Sections

**Files:**
- Modify: `README.md`
- Modify: `README.zh_CN.md`

**Interfaces:**
- Consumes: `DataType`, `DataTypeOf`, `DataConverter`, `DataConverters`, `DataConversionTarget`, `DataConversionOptions`, `NumericValueRef`, `NumericComparisonPolicy`, and `compare_numeric`.
- Produces: Four short, equivalent English/Chinese examples that users can copy.

- [x] **Step 1: Add scalar, batch, and policy conversion examples**

Add a `Typical usage` / `典型用法` section after the feature table. Include this behavior in both languages:

```rust
use qubit_datatype::{
    DataConversionOptions, DataConverter, DataConverters,
};

assert_eq!(DataConverter::from("8080").to::<u16>(), Ok(8080));

let ports: Vec<u16> = DataConverters::from(vec!["8080", "8081"])
    .to_vec()
    .unwrap();
assert_eq!(ports, vec![8080, 8081]);

let lossy = DataConversionOptions::lossy();
assert_eq!(DataConverter::from("3.9").to_with::<i32>(&lossy), Ok(3));
```

- [x] **Step 2: Add the downstream target-extension example**

Use the same `Port` implementation in both READMEs:

```rust
use qubit_datatype::{
    DataConversionError, DataConversionOptions, DataConversionTarget,
    DataConverter, DataType, DataTypeOf,
};

#[derive(Debug, PartialEq, Eq)]
struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert_from(
        source: &DataConverter<'_>,
        options: &DataConversionOptions,
    ) -> Result<Self, DataConversionError> {
        u16::convert_from(source, options).map(Self)
    }
}

assert_eq!(DataConverter::from("8080").to::<Port>(), Ok(Port(8080)));
```

- [x] **Step 3: Add an exact-versus-approximate numeric comparison example**

Use fixed-width values so the example requires no optional big-number dependency:

```rust
use std::cmp::Ordering;
use qubit_datatype::{
    NumericComparisonPolicy, NumericValueRef, compare_numeric,
};

let integer = NumericValueRef::UInt64((1_u64 << 53) + 1);
let float = NumericValueRef::Float64((1_u64 << 53) as f64);

assert_eq!(
    compare_numeric(integer, float, NumericComparisonPolicy::Exact),
    Some(Ordering::Greater),
);
assert_eq!(
    compare_numeric(integer, float, NumericComparisonPolicy::Approximate),
    Some(Ordering::Equal),
);
```

- [x] **Step 4: Keep the detailed sections concise**

Retain the existing type-vocabulary, conversion-contract, numeric-comparison,
format, error, and development sections. Remove only duplicated example prose
made redundant by the new typical-usage section.

### Task 2: Align Both READMEs With the Current Code

**Files:**
- Modify: `README.md`
- Modify: `README.zh_CN.md`

**Interfaces:**
- Consumes: the public exports in `src/lib.rs`, feature declarations in `Cargo.toml`, and the conversion/error contracts in `src/converter`.
- Produces: Matching technical claims and section trees.

- [x] **Step 1: Compare heading structure**

Run:

```bash
rg '^#{1,6} ' README.md
rg '^#{1,6} ' README.zh_CN.md
```

Expected: the same number and order of headings, translated where appropriate.

- [x] **Step 2: Audit feature and API names**

Run:

```bash
rg -n 'DataConvertTo|FromMetadataValue|NumberComparisonPolicy|number_comparison_policy' README.md README.zh_CN.md
```

Expected: no matches.

Confirm both files describe 25 runtime types, the same seven feature rows,
the four `DataConversionError` variants, `DataConversionTarget`, and
`NumericComparisonPolicy::{Exact, Approximate}`.

- [x] **Step 3: Compare fenced-block counts**

Run:

```bash
rg -c '^```' README.md README.zh_CN.md
```

Expected: equal counts.

### Task 3: Verify Documentation Examples

**Files:**
- Verify: `README.md`
- Verify: `README.zh_CN.md`

**Interfaces:**
- Consumes: the completed matching README examples.
- Produces: evidence that formatting, crate tests, and the documented APIs are valid.

- [x] **Step 1: Format and lint the crate**

Run:

```bash
./align-ci.sh
cargo +nightly-2026-06-05 fmt -- --config-path .rs-ci/rustfmt.toml --check
```

Expected: both commands exit zero.

- [x] **Step 2: Run all-feature tests and doctests**

Run:

```bash
cargo test --all-features
```

Expected: all unit, integration, property, and doctests pass.

- [x] **Step 3: Compile the README Rust blocks**

Build the all-features library, then run `rustdoc --test` for each README with
the local dependency search path and every crate feature cfg enabled:

```bash
cargo build --all-features
rustdoc --edition 2024 --test README.md \
  -L dependency=target/debug/deps \
  --extern qubit_datatype=target/debug/libqubit_datatype.rlib \
  --cfg 'feature="converter"' --cfg 'feature="chrono"' \
  --cfg 'feature="big-number"' --cfg 'feature="url"' \
  --cfg 'feature="json"'
rustdoc --edition 2024 --test README.zh_CN.md \
  -L dependency=target/debug/deps \
  --extern qubit_datatype=target/debug/libqubit_datatype.rlib \
  --cfg 'feature="converter"' --cfg 'feature="chrono"' \
  --cfg 'feature="big-number"' --cfg 'feature="url"' \
  --cfg 'feature="json"'
```

Expected: every runnable README block passes.

- [x] **Step 4: Run final repository checks**

Run:

```bash
git diff --check
git status --short
```

Expected: no whitespace errors; only the already scoped implementation and
README design/plan changes are present. Do not commit without separate user
authorization.
