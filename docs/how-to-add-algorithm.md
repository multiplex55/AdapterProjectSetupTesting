# How to Add an Algorithm

See also: [Folder guide](./folder-guide.md) · [Add control flow](./how-to-add-control-flow.md) · [Add adapter](./how-to-add-adapter.md) · [Add Target5 feature](./how-to-add-target5-feature.md) · [Add Target10 feature](./how-to-add-target10-feature.md) · [Add DLL/SO-backed provider](./how-to-add-dll-so-backed-provider.md) · [Add C FFI wrapper](./how-to-add-c-ffi-wrapper.md)

## When to use this guide
Use this guide when you need a deterministic, platform-agnostic business rule in `crates/core/src/algorithms`.

## Files to touch
- `crates/core/src/algorithms/*.rs` for the algorithm implementation.
- `crates/core/src/algorithms/mod.rs` to expose the module.
- Optionally `crates/core/src/flows/*.rs` if orchestration must call the new algorithm.

## Files not to touch
- Do **not** place domain logic in `apps/*/src/main.rs`; app mains are composition-only.
- Do **not** add platform-specific logic in `crates/core`.
- Do **not** create ad hoc cross-layer payloads outside `crates/messages`.

## Step-by-step changes
1. Add a new pure function under `crates/core/src/algorithms/`.
2. Keep inputs/outputs portable (`crates/messages` and core domain types).
3. Add explicit typed errors if needed; no hidden fallback behavior.
4. Export the new module/function in `crates/core/src/algorithms/mod.rs`.
5. If orchestration is needed, call it from a flow module in `crates/core/src/flows/`.
6. Run `cargo metadata` and `cargo check --workspace`.

## Small example
- Add `crates/core/src/algorithms/target5_to_target10.rs`-style logic for a new deterministic mapping.
- Wire orchestration caller from `crates/core/src/flows/target5_to_target10.rs` rather than from app entrypoints.

## Common mistakes
- Embedding runtime or adapter calls directly inside algorithm code.
- Hiding failures with silent default values instead of typed errors.
- Defining shared payload structs inside adapter crates instead of `crates/messages`.
