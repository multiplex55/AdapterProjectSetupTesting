# How to Add Control Flow

See also: [Folder guide](./folder-guide.md) · [Add algorithm](./how-to-add-algorithm.md) · [Add adapter](./how-to-add-adapter.md) · [Add Target5 feature](./how-to-add-target5-feature.md) · [Add Target10 feature](./how-to-add-target10-feature.md) · [Add DLL/SO-backed provider](./how-to-add-dll-so-backed-provider.md) · [Add C FFI wrapper](./how-to-add-c-ffi-wrapper.md)

## When to create a new flow vs extend algorithm/state

Create a **new flow** when the change:
- Sequences multiple steps/effects (e.g., validate -> query port -> transform -> publish).
- Requires explicit branching/retry/compensation policy.
- Coordinates more than one algorithm or external effect boundary.
- Needs a dedicated typed error surface for an end-to-end use case.

Extend an **existing algorithm** when the change is:
- Pure deterministic transformation/validation.
- Independent from effect orchestration order.

Extend **state/model** when the change is:
- New domain fact/invariant/transition stored as business state.
- Not primarily about sequencing side effects.

## Files to touch
- `crates/core/src/flows/*.rs` for orchestration entrypoints.
- `crates/core/src/flows/mod.rs` to expose the flow module.
- `crates/core/src/algorithms/*.rs` only for pure transforms called by the flow.

## Files not to touch
- Do **not** place domain logic in `apps/*/src/main.rs`; app mains are composition-only.
- Do **not** add platform-specific logic in `crates/core`.
- Do **not** create ad hoc cross-layer payloads outside `crates/messages`.

## Step-by-step changes
1. Confirm the behavior is orchestration (not just algorithm/state extension).
2. Add a flow module under `crates/core/src/flows/` for the use case.
3. Sequence port-driven calls and algorithm functions explicitly.
4. Define typed flow errors and return them directly.
5. Export the flow from `crates/core/src/flows/mod.rs` and `crates/core/src/lib.rs` when needed.
6. Keep app crates focused on wiring runtime to this flow.
7. Run `cargo metadata` and `cargo check --workspace`.

## Small example
- Add `crates/core/src/flows/target5_to_target10.rs` orchestration that calls deterministic logic in `crates/core/src/algorithms/target5_to_target10.rs` and returns a flow error enum.

## Common mistakes
- Treating an algorithm module as a use-case orchestration API.
- Calling adapters directly from flow code instead of through `crates/ports` abstractions.
- Catch-all fallback behavior that suppresses the real failure reason.
