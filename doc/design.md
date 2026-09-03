# Conversion execution boundary

## Problem

`ConversionSession` owns cumulative policy and resource accounting. Passing it
to every `DataConversionTarget` also gave downstream targets raw operations
that could charge an arbitrary type pair, repeat an admission, or omit one.
Those mistakes are easy to compile and difficult to audit from a caller.

## Decision

The public target-extension boundary is `ConversionContext<'_, '_>`.
`DataConverter` admits the top-level item and input exactly once, creates a
context bound to the runtime source and requested target types, and invokes the
target. An `AdmittedScalarItem` follows the same rule after its one-use token is
consumed.

The context exposes only operations that are meaningful inside that conversion:

- nested borrowed and owned delegation;
- policy, limits, and bound type accessors;
- JSON decode, seed decode, value accounting, and encode when `json` is
  enabled;
- transactional String rendering.

`ConversionSession` remains public for construction, reuse, scalar-item
admission, configuration, and cumulative usage observation. Its raw byte and
structured-accounting primitives are crate-private.

## Invariants

1. A top-level converter call charges its item and input before the target runs.
2. Delegation performs no second top-level admission.
3. Every context budget error carries the active source and target types.
4. Failed String rendering and failed output admission do not commit output
   bytes.
5. A scalar-item token can be consumed only once and receives a bound context.

## Compatibility

This is intentionally a breaking API change: downstream
`DataConversionTarget` implementations must replace their
`&mut ConversionSession` parameter with `&mut ConversionContext` and call
context methods. The change removes a capability rather than adding a parallel
API, keeping the extension model singular and auditable.

## Runtime descriptors and capability discovery

`DataType` owns its stable name and feature-independent semantic category.
Callers use `DataType::as_str` and `DataType::category` directly. The former
`DataTypeInfo` wrapper remains deprecated for source compatibility, but it
delegates to those methods and is not a second metadata authority.

Conversion path topology has one internal canonical matrix. The public
`ConversionCapabilities` facade first filters source and target availability
using the active Cargo features, then consults that matrix. This keeps feature
discovery public while preventing documentation, capability queries, and
converter dispatch relationships from developing parallel policy tables.
