# Qubit Datatype User Guide

[中文用户手册](user_guide.zh_CN.md) · Applies to `qubit-datatype` 0.15.0

This guide is for Rust application developers who must turn configuration,
protocol, or ingestion values into typed values while retaining explicit rules
for precision, accepted text, and resource use. It covers the crate's current
public API; see the [README](../README.md) for a capability overview and
[docs.rs](https://docs.rs/qubit-datatype) for API-level details.

## Purpose and audience

Use this crate when the source type is available only at runtime, when a schema
needs a stable type vocabulary, or when unlike numeric representations must be
compared without an implicit `f64` conversion. It is not a general-purpose
serialization framework, and it does not make unsupported source/target pairs
convertible.

The default feature set provides `DataType`, `DataTypeOf`, `NumberRef`, and
numeric comparison. Enable `duration` for Duration text and Serde adapters;
enable `converter` for runtime conversion. Rich-type features such as `chrono`,
`big-integer`, `big-decimal`, `url`, and `json` add their mappings, but require
`converter` to participate in conversion.

## Conceptual model

`DataType` is the crate's stable runtime vocabulary, and `DataTypeOf` maps
supported Rust types to it. `DataConverter` holds a borrowed or owned runtime
source. A conversion combines that source with a `ConversionPolicy` (meaning)
and `ConversionLimits` (resource bounds). `ConversionSession` is optional: use
one only when several calls must share the same cumulative budget.

```text
runtime source ── DataConverter ── policy + limits ──> typed result
                                      │
                                      └─ optional shared ConversionSession
```

`DataConversionError` exposes a stable kind and type context, but deliberately
does not retain the source value. That makes it suitable to report for sources
such as environment variables without accidentally exposing their contents.

## Scenario: reading deployment settings

Assume a service receives `PORT=8080`, `ADMIN_ENABLED=yes`, and
`RETRY_DELAY=1500ms`. Success means it obtains a `u16`, a `bool`, and a
`Duration`; an invalid value must be rejected with a stable error classification
rather than being coerced invisibly.

### Installation and minimal configuration

Add the conversion and Duration APIs:

```toml
[dependencies]
qubit-datatype = { version = "0.15", default-features = false, features = ["converter"] }
```

`converter` includes `duration`. For only `DataType` and `NumberRef`, use
`qubit-datatype = "0.15"` without optional features.

### Core workflow

The strict policy is the default. It preserves whitespace, accepts only the
default Boolean literals, and rejects precision-losing numeric and Duration
conversions. An environment-style input normally benefits from the explicit
`env_friendly()` profile instead. Boolean literal matches take precedence over
the numeric text byte limit. For an unknown Boolean string, exceeding
`numeric.max_text_bytes` returns a `NumericTextBytes` limit error even when the
text is not valid integer syntax; an unknown string within the limit remains an
`InvalidBoolean` error. Errors report type and resource context without retaining
the source value.

```rust
use std::time::Duration;

use qubit_datatype::{ConversionLimits, ConversionPolicy, DataConversionError, DataConverter};

fn main() -> Result<(), DataConversionError> {
let policy = ConversionPolicy::env_friendly();
let limits = ConversionLimits::default();

let port = DataConverter::from("8080").to_with::<u16>(&policy, &limits)?;
let admin = DataConverter::from(" yes ").to_with::<bool>(&policy, &limits)?;
let retry = DataConverter::from("1500ms").to_with::<Duration>(&policy, &limits)?;

assert_eq!(port, 8080);
assert!(admin);
assert_eq!(retry, Duration::from_millis(1500));
    Ok(())
}
```

The result is observable: the values have their requested Rust types, and no
conversion rule was selected implicitly. For repeated independent conversions,
calling `to_with` is sufficient because each call creates a fresh session.

### Profiles and numeric text syntax

Conversion profiles select normalization and conversion behavior, such as
trimming text or allowing a precision-losing conversion. They do not impose one
shared lexer on every target type. `BigDecimal` accepts the underscore forms
supported by its parser under all three profiles: `"12_34"`, `"1__2"`,
`"12_"`, and `"1_2.5"`. The same text is invalid integer syntax for fixed-width
integer targets and `BigInt`, so those targets return `InvalidSyntax`.

This target-specific behavior preserves the existing `BigDecimal` parser
compatibility while keeping integer syntax explicit. Choosing `lossy()` or
`env_friendly()` does not change which numeric spellings each target parser
accepts.

### Bound numeric conversion resources

`NumericConversionLimits` applies each limit at a specific conversion stage:

| Limit | Applies when |
| --- | --- |
| `max_text_bytes` | Parsing numeric text, after configured string normalization; the count is UTF-8 bytes. |
| `max_big_integer_digits` | Materializing a `BigInt` result; leading zeros and a zero result do not consume the digit budget. |
| `max_big_decimal_coefficient_digits` | Converting to a `BigDecimal`, including copying an existing value to that target. |
| `max_big_decimal_scale_magnitude` | Converting to a `BigDecimal`, including copying an existing value to that target. |

For example, when an application accepts integer text as `BigInt`, configure the
`converter` and `big-integer` features and set both the text and result limits:

```toml
[dependencies]
qubit-datatype = { version = "0.15", default-features = false, features = ["converter", "big-integer"] }
num-bigint = "0.4"
```

The following complete example accepts the three-digit result `999` and rejects
`1000` because its materialized `BigInt` result has four digits. The input text
limit is set higher than either input so the rejection identifies the result
digit limit:

```rust
use num_bigint::BigInt;
use qubit_datatype::{
    ConversionLimits, ConversionPolicy, ConversionResource, DataConverter,
    NumericConversionLimits,
};

fn main() {
    let numeric = NumericConversionLimits::builder()
        .max_text_bytes(64)
        .max_big_integer_digits(3)
        .build();
    let limits = ConversionLimits::builder().numeric_limits(numeric).build();
    let policy = ConversionPolicy::strict();
    assert_eq!(
        DataConverter::from("999").to_with::<BigInt>(&policy, &limits),
        Ok(BigInt::from(999_u16)),
    );
    let error = DataConverter::from("1000")
        .to_with::<BigInt>(&policy, &limits)
        .expect_err("four result digits exceed the configured maximum");
    assert_eq!(error.resource(), Some(ConversionResource::BigIntegerDigits));
    assert_eq!(error.configured_limit(), Some(3));
}
```

These limits do not cap every temporary allocation while converting an
already-materialized number. For example, converting an existing `BigDecimal`
to a float may format its full coefficient as parser input; the coefficient
and scale limits apply when `BigDecimal` is the target, not to all work on an
existing value. If a stronger resource boundary is required, constrain
untrusted number representations at the application's input or deserialization
boundary. `NumberRef` comparisons are also independent of conversion sessions
and their budgets; see [Compare mixed numeric values](#compare-mixed-numeric-values).

For the detailed contract of each setter and its target stage, see the
[`NumericConversionLimits` API documentation](https://docs.rs/qubit-datatype/0.15.0/qubit_datatype/struct.NumericConversionLimits.html).

## Advanced usage

### Read a deployment port list

For a scalar setting such as `PORTS=8080,, 8443;9000`, configure splitting,
delimiters, trimming, and empty-item handling explicitly. The
[`deployment_lists` example](../examples/deployment_lists.rs) is runnable with
`cargo run --example deployment_lists --features converter` and exercises the
same calls below:

```rust
use qubit_datatype::{
    CollectionConversionPolicy, ConversionLimits, ConversionPolicy,
    DataConversionErrorKind, EmptyItemPolicy, ScalarStringDataConverters,
};

let collection = CollectionConversionPolicy::builder()
    .split_scalar_strings(true)
    .delimiters([',', ';'])
    .trim_items(true)
    .empty_item_policy(EmptyItemPolicy::Skip)
    .build();
let policy = ConversionPolicy::builder().collection_policy(collection).build();
let limits = ConversionLimits::default();
let ports = ScalarStringDataConverters::from("8080,, 8443;9000")
    .to_vec_with::<u16>(&policy, &limits)
    .expect("all configured ports should be valid");
assert_eq!(ports, vec![8080, 8443, 9000]);

let error = ScalarStringDataConverters::from("8080,,bad")
    .to_vec_with::<u16>(&policy, &limits)
    .expect_err("the third item is invalid");
assert_eq!(error.source_index(), 2);
assert_eq!(error.conversion_error().kind(), DataConversionErrorKind::InvalidValue);
assert_eq!(
    ScalarStringDataConverters::from("8080,,bad").to_first_with::<u16>(&policy, &limits)
        .expect("the first item is valid"),
    8080,
);
```

`source_index()` is zero-based in the original input. Skipping the empty item
does not renumber `bad`, so its index remains `2`. `to_first_with` converts only
the first item and does not validate unvisited trailing items; use
`to_vec_with` when the entire setting must be valid before applying it. For an
already materialized list, use `DataConverters` instead of splitting each
string element again.

When several conversions belong to one operation, reuse one
`ConversionSession`. The example sets a two-item operation limit, attempts a
three-port list, and observes `ConversionResource::Items` at source index `2`.
The complete scalar source is admitted for its byte budget first, then item
budget is charged as items are visited. A failed conversion does not refund
input bytes or items already charged. Apply a deployment update only after the
full-list conversion succeeds; on rejection, report its index and error kind,
not the original configuration value, which may contain a secret.

At the lower-level iterator boundary, `ScalarItems` remains usable after a
rejected empty item, while its item-limit error exhausts that iterator.
`AdmittedScalarSource::next_item` exhausts its source after any splitting or
item-admission error; a later target-conversion error does not itself exhaust
the source. Ordinary applications can use `ScalarStringDataConverters` and
leave these lower-level iteration details to the adapter.

### Share a budget across a logical operation

Build a `ConversionSession` when a whole request, rather than each field,
needs one cumulative item or byte budget. The next example permits two admitted
items and rejects the third.

```rust
use qubit_datatype::{
    ConversionLimits, ConversionOperationLimits, ConversionPolicy,
    ConversionSession, DataConversionError, DataConverter,
};

fn main() -> Result<(), DataConversionError> {
let policy = ConversionPolicy::strict();
let limits = ConversionLimits::builder()
    .operation_limits(ConversionOperationLimits::builder().max_items(2).build())
    .build();
let mut session = ConversionSession::new(&policy, &limits);

assert_eq!(DataConverter::from("1").to_in::<u16>(&mut session)?, 1);
assert_eq!(DataConverter::from("2").to_in::<u16>(&mut session)?, 2);
assert!(DataConverter::from("3").to_in::<u16>(&mut session).is_err());
    Ok(())
}
```

An attempt that has passed the item-budget check retains that item charge if
an input check or target conversion later fails. Successfully charged input
bytes are retained too. A failed item-budget check adds no item. A failed
individual input-byte charge adds no bytes, but does not undo the item charge
that preceded it. Built-in transactional String output is committed only on
success. Use the session's `*_used`, `*_remaining`, and `*_limit` accessors when
you need to observe the budget. `ConversionSession` is an operation-level
owner: custom targets do not receive it directly.

### Extend a target type safely

Implement `DataConversionTarget` for a downstream newtype. The converter gives
the implementation an `AdmittedConversion` containing the exact source value
and the already-bound policy, limits, session, and error context. This value is
created by the framework, so the target does not implement an input wrapper or
perform top-level admission itself.

```rust
use qubit_datatype::{
    AdmittedConversion, DataConversionError, DataConversionTarget, DataConverter,
    DataType, DataTypeOf,
};

struct Port(u16);

impl DataTypeOf for Port {
    const DATA_TYPE: DataType = DataType::UInt16;
}

impl DataConversionTarget for Port {
    fn convert(input: AdmittedConversion<'_, '_, '_>) -> Result<Self, DataConversionError> {
        input.convert::<u16>().map(Self)
    }
}

fn main() -> Result<(), DataConversionError> {
    assert_eq!(DataConverter::from("8080").to::<Port>()?.0, 8080);
    Ok(())
}
```

`input.convert::<u16>()` consumes the admitted wrapper and delegates the same
source; it does not create a second top-level conversion. The wrapper also
provides `from_type()`, `to_type()`, `source()`, JSON helpers (with `json`), and
transactional `write_string`. The old unsafe delegation methods are no longer
part of the API, and raw admission/accounting operations remain internal.

Delegation is limited to a target whose `DataType` matches the outer target bound
to this context. A different descriptor is rejected with `Unsupported` before
the delegated target runs; the error retains the original source and the outer
target type. The top-level item and successfully charged input bytes remain
used, but the rejected delegation adds no further charges and does not roll
back those earlier charges.

This tightens behavior for custom targets that previously delegated across
descriptors: such a chain may now return `Unsupported`. If the newtype only adds
a domain constraint, give it the wrapped type's `DataType`, delegate, then check
the domain rule. For genuinely different semantics, implement the range check and
mapping from `source()` in the custom target and use the existing budget helpers
for output. Do not create another session to avoid cumulative limits.

### Compare mixed numeric values

Choose `NumericComparisonPolicy::Exact` for validation, storage, deterministic
ordering, and keys. `Approximate` is only for pairwise IEEE-style comparison:
across mixed representations it can be non-transitive, so do not use it to
implement `Ord`, sort or group values, or create ordered-map/set keys.

```rust
use std::cmp::Ordering;

use qubit_datatype::{NumberRef, NumericComparisonPolicy};

fn main() {
let integer = NumberRef::from((1_u64 << 53) + 1);
let float = NumberRef::from((1_u64 << 53) as f64);
assert_eq!(
    integer.compare(float, NumericComparisonPolicy::Exact),
    Some(Ordering::Greater),
);
}
```

NaN is unordered and signed zeroes compare equal.

Fixed-width comparisons need no heap allocation. Some paths involving
`BigInt` or `BigDecimal` may clone coefficients or construct exact rational or
decimal values, so their cost can grow with the number representation; some
large-number pairs have direct comparison paths. `ConversionLimits` and
`ConversionSession` govern conversions and do not budget this independent
comparison. If numbers come from untrusted input, bound their representation
size at the input boundary.

### Parse standalone Duration text

When runtime conversion is unnecessary, enable `duration` and call the
lightweight text API. Default options reject a suffixless input and use strict
unit spellings.

```rust
use std::time::Duration;

use qubit_datatype::{parse_duration_text, DurationTextOptions};

fn main() -> Result<(), qubit_datatype::DurationParseError> {
let value = parse_duration_text("1500ms", &DurationTextOptions::default())?;
assert_eq!(value, Duration::from_millis(1500));
assert!(parse_duration_text("1500", &DurationTextOptions::default()).is_err());
    Ok(())
}
```

Strict mode accepts `us`, `µs`, and `μs` for microseconds. It rejects the
lenient-only minute alias `m`; canonical output uses `µs` and `min`.

For Serde fields, select the adapter from the wire contract:

| Adapter | Representation | Precision and range |
| --- | --- | --- |
| `duration_millis` | JSON number | Half-up to milliseconds; errors if the rounded result exceeds `u64::MAX` milliseconds |
| `duration_millis_with_unit` | `"<integer>ms"` | Half-up to milliseconds; serialization saturates at `18446744073709551615999ms`, while deserialization rejects values outside `Duration` range |
| `duration_millis_with_unit_exact` | `"<integer>ms"` | Rejects submillisecond values; whole-millisecond range follows `Duration`, without a `u64` millisecond cap |
| `duration_with_unit` | Canonical unit string | Exact, with no rounding or saturation |

These examples need only the `duration` feature; they do not require `converter`.
Add direct dependencies on Serde derive and `serde_json`:

```toml
[dependencies]
qubit-datatype = { version = "0.15", default-features = false, features = ["duration"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

```rust
use std::time::Duration;
use serde::Serialize;

#[derive(Serialize)]
struct NumericDelay {
    #[serde(with = "qubit_datatype::serde::duration_millis")]
    delay: Duration,
}

#[derive(Serialize)]
struct TextDelay {
    #[serde(with = "qubit_datatype::serde::duration_millis_with_unit")]
    delay: Duration,
}

#[derive(Serialize)]
struct ExactDelay {
    #[serde(with = "qubit_datatype::serde::duration_millis_with_unit_exact")]
    delay: Duration,
}

fn main() {
    let rounded = NumericDelay { delay: Duration::from_micros(1_500) };
    assert_eq!(serde_json::to_value(&rounded).unwrap(), serde_json::json!({"delay": 2}));
    let inexact = ExactDelay { delay: Duration::from_micros(1_500) };
    assert!(serde_json::to_string(&inexact).is_err());
    let exact = ExactDelay { delay: Duration::from_millis(2) };
    assert_eq!(serde_json::to_value(&exact).unwrap(), serde_json::json!({"delay": "2ms"}));
    assert!(serde_json::to_string(&NumericDelay { delay: Duration::MAX }).is_err());
    assert_eq!(
        serde_json::to_value(&TextDelay { delay: Duration::MAX }).unwrap(),
        serde_json::json!({"delay": "18446744073709551615999ms"}),
    );
}
```

### Bound structured input

When configuration or protocol input contains nested JSON, use
`StructuredConversionLimits` to bound each value before it is materialized.
Depth counts the root as one. Sequence and map maxima count direct children
separately for each container, rather than summing all entries in the tree.
The cumulative `max_structured_nodes` operation limit is a separate session
budget.

`max_text_bytes` bounds normalized source text on supported text conversion
paths. It is not a per-string limit for an already materialized JSON value.
The reusable `value()` template also carries a string-byte limit with the same
maximum, which consumers that apply the template directly enforce. Built-in
conversion sessions select its structure limits and add cumulative node and
payload budgets; they do not apply its string-byte limit. The template is not
a snapshot of a session's effective budgets.

When a custom target calls `AdmittedConversion::decode_json`, the helper also
checks the exact bytes passed to it against `max_text_bytes`, including any
whitespace. It does not normalize the input or admit it again. If this check
rejects the text, the already charged item and source bytes remain charged, and
no structured or output bytes are added. Built-in text-to-JSON conversion
continues to check its normalized source text before decoding.

This example requires the `json` feature in addition to `converter`, and a direct dependency on `serde_json`. Use the following dependency configuration:

```toml
[dependencies]
qubit-datatype = { version = "0.15", default-features = false, features = ["converter", "json"] }
serde_json = "1.0"
```

```rust
use qubit_datatype::{
    ConversionLimits, ConversionPolicy, ConversionResource, DataConverter,
    StructuredConversionLimits,
};

fn main() {
    let policy = ConversionPolicy::strict();
    let limits = ConversionLimits::builder()
        .structured_limits(
            StructuredConversionLimits::builder()
                .max_depth(3)
                .max_sequence_items(2)
                .max_map_entries(1)
                .build(),
        )
        .build();
    let value = DataConverter::from("[[1],[2]]")
        .to_with::<serde_json::Value>(&policy, &limits)
        .expect("the nested value fits depth three");
    assert_eq!(value, serde_json::json!([[1], [2]]));

    let zero_items = ConversionLimits::builder()
        .structured_limits(StructuredConversionLimits::builder().max_sequence_items(0).build())
        .build();
    assert!(DataConverter::from("[]")
        .to_with::<serde_json::Value>(&policy, &zero_items)
        .is_ok());
    let error = DataConverter::from("[1]")
        .to_with::<serde_json::Value>(&policy, &zero_items)
        .expect_err("a zero item limit rejects a non-empty sequence");
    assert_eq!(error.resource(), Some(ConversionResource::SequenceItems));
}
```

The same checks apply when the source is already a materialized
`serde_json::Value`. A zero depth limit rejects even a root scalar because its
depth is one; zero sequence or map limits accept only empty containers.

### Control policy builder order

Group setters such as `string_policy` replace the whole policy group. Leaf
setters such as `blank_string_policy` update one field in the current group.
Calls take effect in order, so a later group setter resets an earlier leaf
override:

`BooleanConversionPolicy`'s builder collects literal lists and scalar settings
and validates their final combination at `build()`. Its result therefore does
not depend on whether `case_sensitive` or a literal setter is called first.
Repeated settings of the same scalar field use the last value. This does not
change the replacement behavior of group setters such as `string_policy`.

```rust
use qubit_datatype::{
    BlankStringPolicy, CollectionConversionPolicy, ConversionPolicy,
    EmptyItemPolicy, StringConversionPolicy,
};

fn main() {
    let leaf_last = ConversionPolicy::builder()
        .string_policy(StringConversionPolicy::default())
        .blank_string_policy(BlankStringPolicy::Reject)
        .build();
    assert_eq!(leaf_last.string().blank_string_policy(), BlankStringPolicy::Reject);

    let group_last = ConversionPolicy::builder()
        .blank_string_policy(BlankStringPolicy::Reject)
        .string_policy(StringConversionPolicy::default())
        .build();
    assert_eq!(group_last.string().blank_string_policy(), BlankStringPolicy::Preserve);

    let collection = ConversionPolicy::builder()
        .empty_item_policy(EmptyItemPolicy::Reject)
        .collection_policy(CollectionConversionPolicy::default())
        .build();
    assert_eq!(collection.collection().empty_item_policy(), EmptyItemPolicy::Keep);
}
```

### Parse Instant text precisely

With the `chrono` feature, RFC 3339 Instant text may contain at most nine
fractional digits. Every conversion profile enforces this limit, including
when excess digits are trailing zeroes. For example, `.123456789Z` is accepted
but `.1234567890Z` is rejected. If an upstream source supplies finer
precision, apply the domain's explicit rounding or truncation rule before
conversion, or pass an already constructed `DateTime<Utc>`.

## Errors and diagnostics

Handle `DataConversionError` by its stable `DataConversionErrorKind`, not by
formatting an error message. The main kinds are `Missing`, `EmptyCollection`,
`Unsupported`, `InvalidValue`, and `LimitExceeded`; `Quantity` reports that a
native measurement could not fit the configured resource quantity.

```rust
use qubit_datatype::{DataConversionErrorKind, DataConverter};

fn main() {
let error = DataConverter::from("3.9").to::<i32>().unwrap_err();
assert_eq!(error.kind(), DataConversionErrorKind::InvalidValue);
}
```

For an invalid value, inspect `reason()` when the caller needs a stable reason.
For a limit failure, inspect `resource()`, `configured_limit()`,
`observed_lower_bound()`, `used()`, `remaining()`, and `requested()` as
applicable. Batch APIs return `DataListConversionError`, which retains the
original `source_index` alongside the conversion error.

When a JSON conversion fails, `json_diagnostic()` provides safe decoder
metadata without changing the general `InvalidValue` classification. Its
`category()` is one of `Read`, `EmptyInput`, `InvalidUtf8`, `InvalidJson`,
`UnexpectedTopLevel`, or `Deserialize`. This example uses the `converter` and
`json` features and a direct `serde_json` dependency, as shown in [Bound
structured input](#bound-structured-input):

```rust
use qubit_datatype::{DataConverter, JsonConversionErrorCategory};

let error = DataConverter::from("{\n@}")
    .to::<serde_json::Value>()
    .unwrap_err();
let diagnostic = error.json_diagnostic().expect("JSON diagnostic");
assert_eq!(diagnostic.category(), JsonConversionErrorCategory::InvalidJson);
assert_eq!(diagnostic.line(), Some(2));
assert_eq!(diagnostic.column(), Some(1));
```

The line number is one-based. Coordinates refer to the text passed to the
decoder, which may be normalized text. `InvalidJson` columns count Unicode
characters; `Deserialize` columns count UTF-8 bytes. A decoder may omit either
coordinate, so neither coordinate is guaranteed for every category or entry
point; an omitted coordinate is `None`. The diagnostic stores no input value,
object key, or backend message. Budget failures remain `LimitExceeded` errors
and have no JSON diagnostic. Adding this metadata does not change public error
equality: diagnostics are not part of `DataConversionError` equality.

`requested()` describes the rejected cumulative request. It may be a
conservative minimum reservation or one formatter write, rather than the exact
final text size. Resource failures also expose their underlying cause through
`std::error::Error::source`; use the stable accessors for classification.

For custom scalar-list adapters, call
`session.admit_scalar_string_source(text)` and use the returned
`AdmittedScalarSource::next_item` to borrow one admitted item at a time.
Consume each item with `convert` or `convert_with_session`. This preserves
original indices and charges the full source bytes exactly once, while unvisited
items are not scanned. Splitting and item-admission errors exhaust the source.

## Troubleshooting

| Symptom | Check | Resolution |
| --- | --- | --- |
| `Unsupported` | The active feature set and the source/target pair | Enable `converter` plus the matching rich-type feature, or choose a supported target. |
| `InvalidValue` for `" 42 "` | The strict profile does not trim | Use an explicit policy such as `ConversionPolicy::env_friendly()` only when trimming is intended. |
| Integer conversion rejects `"3.9"` | Strict numeric rules preserve precision | Keep the rejection or select `ConversionPolicy::lossy()` deliberately. |
| `LimitExceeded` | Per-value and session limits | Increase the relevant documented limit only after deciding the input is trustworthy. |
| Duration text is rejected | Suffix and parse mode | Supply a canonical suffix such as `ms`; use a configured policy only for a known external format. |

## Limitations and best practices

- Do not treat `DataType` spellings as disposable: lowercase strings exposed by
  `as_str`, accepted by Serde, and listed by `DataType::ALL` are a compatibility
  surface.
- `isize` and `usize` have no `DataTypeOf` mapping because their representation
  depends on the target platform.
- Keep semantic policy separate from resource limits. Policies decide what is
  acceptable; limits bound text, collections, structured input, and sessions.
- Errors retain type context but not source text. Log source values only under
  the calling application's own secret-handling policy.

## Further reading

- [README](../README.md)
- [中文用户手册](user_guide.zh_CN.md)
- [API documentation on docs.rs](https://docs.rs/qubit-datatype)
