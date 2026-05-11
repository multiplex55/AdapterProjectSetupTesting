# How to Add a C FFI Wrapper

See also: [Folder guide](./folder-guide.md) · [Add algorithm](./how-to-add-algorithm.md) · [Add control flow](./how-to-add-control-flow.md) · [Add adapter](./how-to-add-adapter.md) · [Add Target5 feature](./how-to-add-target5-feature.md) · [Add Target10 feature](./how-to-add-target10-feature.md) · [Add DLL/SO-backed provider](./how-to-add-dll-so-backed-provider.md)

## When to use this guide
Use this guide when exposing or consuming C ABI boundaries via `crates/ffi/*` or explicitly designated adapter crates.

## Files to touch
- `crates/ffi/c-api/src/lib.rs` for stable C-facing exported APIs.
- `crates/ffi/c-bindings/src/lib.rs` or `crates/ffi/target-bindings/src/lib.rs` for binding-layer support.
- `crates/adapters/c-drivers/src/lib.rs` when integrating a C-backed adapter implementation.
- Runtime/app wiring only as needed for composition.

## Files not to touch
- Do **not** place domain logic in `apps/*/src/main.rs`; app mains are composition-only.
- Do **not** add platform-specific logic in `crates/core`.
- Do **not** create ad hoc cross-layer payloads outside `crates/messages`.

## Step-by-step changes
1. Define minimal ABI-safe types/functions in the right `crates/ffi/*` crate.
2. Keep unsafe code isolated and documented at the FFI boundary.
3. Convert between ABI-layer representations and canonical message/core types outside `core`.
4. Surface explicit errors; avoid silent fallback in wrapper code.
5. Wire usage through adapter/runtime composition.
6. Run `cargo metadata` and `cargo check --workspace`.

## Small example
- Add a new extern entrypoint in `crates/ffi/c-api/src/lib.rs` and adapt it in `crates/adapters/c-drivers/src/lib.rs` to emit/consume canonical contracts from `crates/messages/src/common/mod.rs`.

## Common mistakes
- Spreading unsafe blocks into non-FFI shared crates.
- Placing ABI details in `crates/core`.
- Returning ambiguous success on FFI call failures.
