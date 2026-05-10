# How to add a C driver wrapper

This guide defines the required layering and safety standards for integrating new C drivers.

## Required crate layering

1. Add or update **generated raw bindings** only in `crates/ffi/target-bindings`.
   - This crate is ABI-focused: generated C-compatible types, constants, and `extern` declarations.
2. Add **safe wrappers** only in `crates/adapters/c-drivers`.
   - This crate owns pointer/buffer validation, error translation, and safe APIs consumed by upper layers.
3. Do not skip layers.
   - `crates/core` and other domain-facing crates must use adapter interfaces, never raw FFI symbols.
4. Raw C structs must not become core domain models.
   - Core/domain types are Rust semantic models in message/core crates, mapped from adapter wrapper outputs.

## Bindgen workflow

Use this workflow whenever C headers change.

### Binding generation location rules

- Generated bindgen output stays in: `crates/ffi/target-bindings`.
- Handwritten safe wrappers stay in: `crates/adapters/c-drivers`.
- Do not copy bindgen-generated raw structs/enums into `core` or message-domain modules.

### Typical regeneration flow

1. Update or add vendor C headers used by bindings.
2. Regenerate bindgen outputs inside `crates/ffi/target-bindings`.
3. Review generated diff for ABI-impacting changes.
4. Update safe wrapper mappings in `crates/adapters/c-drivers`.
5. Re-run adapter tests and workspace checks.

### Regeneration/update hygiene tips

- Keep generated files clearly separated from handwritten code inside `crates/ffi/target-bindings`.
- Review points for bindgen diffs:
  - integer width/signedness changes
  - struct layout/packing/alignment changes
  - pointer constness changes
  - added/removed enum variants and constants
- Wrapper validation steps after regeneration:
  - verify null/length/pointer checks still match new signatures
  - verify raw status/error code mapping remains complete
  - verify no new raw types leak through public safe wrapper APIs

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

- [ ] Regenerate/update ABI declarations in `crates/ffi/target-bindings` only.
- [ ] Implement/extend safe wrapper in `crates/adapters/c-drivers`.
- [ ] Confirm raw C structs are mapped into semantic Rust types before crossing into domain/core.
- [ ] Add `SAFETY:` comments for each unsafe block.
- [ ] Add unit tests for error translation and boundary checks.
- [ ] Add mock-based adapter tests that run in host CI without hardware.
- [ ] Run `cargo metadata` and `cargo check --workspace` before merging.
