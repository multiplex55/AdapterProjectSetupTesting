# How to Add a Target10 Feature

See also: [Folder guide](./folder-guide.md) · [Add algorithm](./how-to-add-algorithm.md) · [Add control flow](./how-to-add-control-flow.md) · [Add adapter](./how-to-add-adapter.md) · [Add Target5 feature](./how-to-add-target5-feature.md) · [Add DLL/SO-backed provider](./how-to-add-dll-so-backed-provider.md) · [Add C FFI wrapper](./how-to-add-c-ffi-wrapper.md)

## When to use this guide
Use this guide for end-to-end Target10 behavior changes across contracts, domain flow, adapters, and Target10 app wiring.

## Files to touch
- `crates/core/src/algorithms/*` and/or `crates/core/src/flows/*` for business behavior.
- `crates/messages/src/target10/*` for canonical cross-layer contracts.
- `crates/adapters/target10/src/lib.rs` and related transport adapters.
- `apps/target10-app/src/main.rs` for composition-only wiring.

## Files not to touch
- Do **not** place domain logic in `apps/*/src/main.rs`; app mains are composition-only.
- Do **not** add platform-specific logic in `crates/core`.
- Do **not** create ad hoc cross-layer payloads outside `crates/messages`.

## Step-by-step changes
1. Update shared Target10 payload contracts in `crates/messages/src/target10/`.
2. Add/adjust core algorithm and flow behavior in `crates/core/src/algorithms/` and `crates/core/src/flows/`.
3. Update Target10 adapter behavior in `crates/adapters/target10/` (and transport adapters when needed).
4. Keep `apps/target10-app/src/main.rs` changes limited to dependency/runtime composition.
5. Validate workspace buildability and boundaries.

## Small example
- Extend `crates/messages/src/target10/mod.rs`, update flow behavior in `crates/core/src/flows/target5_to_target10.rs`, and translate in `crates/adapters/target10/src/lib.rs`.

## Common mistakes
- Writing Target10 business policy into app startup code.
- Leaking protocol/platform concerns into `crates/core` modules.
- Defining adapter-specific cross-layer message versions outside `crates/messages`.
