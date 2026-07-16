# rs-datatype README Usage Alignment Design

## Goal

Keep `README.md` and `README.zh_CN.md` structurally equivalent and aligned
with the current public API. Add representative, compilable examples that
show how consumers use the type vocabulary, conversion engine, downstream
target extension point, collection conversion, and numeric comparison.

## Structure

Both READMEs use the same section order and equivalent content. A new
`Typical usage` / `典型用法` section follows the feature table and precedes
the detailed API contracts. It contains four matching examples:

1. Inspect runtime types with `DataType` and `DataTypeOf`.
2. Convert one value and a collection with `DataConverter` and
   `DataConverters`, including an explicit lossy conversion.
3. Implement `DataConversionTarget` for a downstream-owned `Port` newtype and
   delegate to the built-in `u16` target.
4. Compare heterogeneous numeric representations with `compare_numeric`,
   demonstrating the difference between `Exact` and `Approximate`.

Existing detailed sections remain authoritative for edge cases. Examples in
the new section stay short and do not duplicate the full conversion matrix or
format specifications.

## Code Alignment

All examples use current names and contracts:

- Generic conversion targets require `T: DataConversionTarget`.
- `DataConverter::to` and `DataConverters::to_vec` are the primary scalar and
  collection examples.
- Lossy conversion uses `DataConversionOptions::lossy()` with `to_with`.
- Numeric comparison uses `NumericValueRef`, `NumericComparisonPolicy`, and
  `compare_numeric`.
- Feature-dependent examples are hidden behind appropriate doctest `cfg`
  guards.

The surrounding prose is audited for the current 25-type vocabulary, feature
composition, conversion policies, structured error variants, and exact
numeric comparison behavior.

## Verification

- Compare both README heading trees and code-block counts.
- Run `cargo test --doc --all-features` so the English README-derived concepts
  and crate documentation remain consistent.
- Extract all Rust README blocks into temporary doctest crates only if the
  repository's existing tooling does not compile README code directly.
- Run `git diff --check` and a stale-name search for removed APIs.
