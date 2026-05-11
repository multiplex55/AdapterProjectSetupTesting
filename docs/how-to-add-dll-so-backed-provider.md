# How to Add a DLL/SO-Backed Provider

See also: [Folder guide](./folder-guide.md) · [Add algorithm](./how-to-add-algorithm.md) · [Add control flow](./how-to-add-control-flow.md) · [Add adapter](./how-to-add-adapter.md) · [Add Target5 feature](./how-to-add-target5-feature.md) · [Add Target10 feature](./how-to-add-target10-feature.md) · [Add C FFI wrapper](./how-to-add-c-ffi-wrapper.md)

## When to use this guide
Use this guide when adding a dynamically loaded provider implemented as a DLL/SO through plugin/runtime boundaries.

## Files to touch
- `crates/plugins/api/src/lib.rs` for plugin/provider contract traits.
- `crates/plugins/loader/src/lib.rs` for loading/registration behavior.
- `crates/runtime/src/provider_registry.rs` or `crates/runtime/src/startup.rs` for runtime integration.
- Optionally `crates/messages` if cross-layer provider payload contracts change.

## Files not to touch
- Do **not** place domain logic in `apps/*/src/main.rs`; app mains are composition-only.
- Do **not** add platform-specific logic in `crates/core`.
- Do **not** create ad hoc cross-layer payloads outside `crates/messages`.

## Step-by-step changes
1. Define/extend provider interface at `crates/plugins/api`.
2. Implement explicit load/resolve error surfaces in `crates/plugins/loader`.
3. Wire provider discovery/selection in `crates/runtime` with observable failure paths.
4. Keep app entrypoint changes strictly about selecting profile/config.
5. Add/adjust plugin contract tests.
6. Run `cargo metadata` and `cargo check --workspace`.

## Small example
- Add a provider registration path in `crates/runtime/src/provider_registry.rs` that consumes loader output from `crates/plugins/loader/src/lib.rs` and reports a typed startup error when a required symbol is missing.

## Common mistakes
- Silent fallback to a different provider when load fails.
- Encoding runtime/provider payload contracts inside plugin loader crate instead of `crates/messages`.
- Letting app mains orchestrate plugin lifecycle directly.
