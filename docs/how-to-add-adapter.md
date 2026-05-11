# How to Add an Adapter

See also: [Folder guide](./folder-guide.md) · [Add algorithm](./how-to-add-algorithm.md) · [Add control flow](./how-to-add-control-flow.md) · [Add Target5 feature](./how-to-add-target5-feature.md) · [Add Target10 feature](./how-to-add-target10-feature.md) · [Add DLL/SO-backed provider](./how-to-add-dll-so-backed-provider.md) · [Add C FFI wrapper](./how-to-add-c-ffi-wrapper.md)

## When to use this guide
Use this guide when implementing a concrete integration in `crates/adapters/*` that satisfies `crates/ports` traits.

## Files to touch
- `crates/adapters/<adapter-name>/src/lib.rs` (or split modules) for concrete implementation.
- `crates/adapters/<adapter-name>/Cargo.toml` for crate wiring.
- `crates/ports/src/*.rs` only if a new abstraction is required.
- App/runtime composition files to register or select the adapter.

## Files not to touch
- Do **not** place domain logic in `apps/*/src/main.rs`; app mains are composition-only.
- Do **not** add platform-specific logic in `crates/core`.
- Do **not** create ad hoc cross-layer payloads outside `crates/messages`.

## Step-by-step changes
1. Choose or create the adapter crate under `crates/adapters/`.
2. Implement relevant port traits from `crates/ports`.
3. Translate boundary payloads using canonical message types from `crates/messages`.
4. Surface integration failures with explicit typed errors.
5. Register/wire the adapter in runtime/app composition.
6. Run `cargo metadata` and `cargo check --workspace`.

## Small example
- Extend `crates/adapters/ethernet/src/lib.rs` to implement a transport port trait from `crates/ports/src/transport.rs`.
- Compose it from `apps/target10-app/src/main.rs` via runtime startup wiring.

## Common mistakes
- Adding business rules into adapter code that should live in `crates/core`.
- Inventing adapter-local DTOs for cross-layer traffic instead of reusing `crates/messages`.
- Swallowing I/O errors and returning success-like defaults.
