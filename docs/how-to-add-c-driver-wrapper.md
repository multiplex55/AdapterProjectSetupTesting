# How to add a C driver wrapper

This guide defines the required layering and safety standards for integrating new C drivers.

## Required crate layering

1. Add or update **raw bindings** only in `crates/ffi/target-bindings`.
   - Keep this crate ABI-focused: C-compatible types, extern declarations, and no domain logic.
2. Add **safe wrappers** only in `crates/adapters/c-drivers`.
   - This crate owns pointer/buffer validation, error translation, and safe APIs consumed by upper layers.
3. Do not skip layers.
   - `crates/core` and other domain-facing crates must use adapter interfaces, never raw FFI symbols.

## Unsafe policy

Unsafe is allowed only at designated FFI boundaries:

- `crates/ffi/*` for extern declarations and ABI boundary definitions.
- explicitly designated adapter boundary crates (including `crates/adapters/c-drivers`) where raw C calls are wrapped.

Rules for every unsafe block:

- Keep scope as narrow as possible.
- Add a nearby `SAFETY:` justification explaining preconditions/invariants.
- Push all post-conditions into safe Rust types before returning.

## Wrapper API expectations

Every new wrapper API in `crates/adapters/c-drivers` must:

1. Convert raw error/status codes into typed Rust errors.
   - Never leak magic integers to higher layers.
2. Validate pointer/null/length and buffer preconditions before/after FFI calls.
   - Invalid boundary data must surface explicit typed errors.
3. Expose safe, domain-facing interfaces through port traits and message types.
   - Keep wire details and C ABI details out of core logic.

## Explicit prohibition

`crates/core` must never call raw FFI directly.

- No direct dependency from `crates/core` to `crates/ffi/*`.
- No `extern "C"` usage in core.
- All hardware access must traverse ports + adapter wrappers.

## Test reinforcement requirements

When adding a new C wrapper, include tests in `crates/adapters/c-drivers` that cover:

1. Error translation unit tests
   - Map representative raw return codes to typed Rust errors.
2. Boundary validation unit tests
   - Validate null/pointer/length preconditions and malformed response handling.
3. Mock-based adapter behavior tests (no hardware)
   - Use mock FFI-facing traits/stubs to verify wrapper behavior without real devices.
   - Verify success and failure paths are explicit and deterministic.

## Suggested implementation checklist

- [ ] Add/extend ABI declarations in `crates/ffi/target-bindings` only.
- [ ] Implement/extend safe wrapper in `crates/adapters/c-drivers`.
- [ ] Add `SAFETY:` comments for each unsafe block.
- [ ] Add unit tests for error translation and boundary checks.
- [ ] Add mock-based adapter tests that run in host CI without hardware.
- [ ] Run `cargo check --workspace` and relevant tests before merging.
