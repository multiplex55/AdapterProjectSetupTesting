# How to Add a Target5 Feature

See also: [Folder guide](./folder-guide.md) · [Add algorithm](./how-to-add-algorithm.md) · [Add control flow](./how-to-add-control-flow.md) · [Add adapter](./how-to-add-adapter.md) · [Add Target10 feature](./how-to-add-target10-feature.md) · [Add DLL/SO-backed provider](./how-to-add-dll-so-backed-provider.md) · [Add C FFI wrapper](./how-to-add-c-ffi-wrapper.md)

## When to use this guide
Use this guide for end-to-end Target5 behavior changes that may span core, messages, adapters, and Target5 app composition.

## Files to touch
- `crates/core/src/algorithms/*` and/or `crates/core/src/flows/*` for business behavior.
- `crates/messages/src/target5/*` for shared Target5-facing payload contracts.
- `crates/adapters/target5/src/lib.rs` (and related adapters) for concrete integration.
- `apps/target5-app/src/main.rs` for composition wiring only.

## Files not to touch
- Do **not** place domain logic in `apps/*/src/main.rs`; app mains are composition-only.
- Do **not** add platform-specific logic in `crates/core`.
- Do **not** create ad hoc cross-layer payloads outside `crates/messages`.

## Step-by-step changes
1. Define/update shared payload contracts in `crates/messages/src/target5/` when layer boundaries change.
2. Implement or adjust core decision/orchestration logic in `crates/core/src/algorithms/` and `crates/core/src/flows/`.
3. Update Target5 adapter implementations in `crates/adapters/target5/`.
4. Keep app entrypoint changes limited to startup wiring in `apps/target5-app/src/main.rs`.
5. Validate with workspace checks.

## Small example
- Add a new Target5 status field in `crates/messages/src/target5/mod.rs`, map it in `crates/core/src/algorithms/target5_to_target10.rs`, and ensure `crates/adapters/target5/src/lib.rs` reads/writes that canonical field.

## Common mistakes
- Implementing Target5 policy branches directly in `apps/target5-app/src/main.rs`.
- Putting Target5-specific wire/protocol details in `crates/core`.
- Duplicating a Target5 payload struct in adapters and messages with drift.
