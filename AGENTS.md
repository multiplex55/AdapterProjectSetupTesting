# Agent Guardrails: Architecture Integrity

## Primary rules

1. Preserve clean architecture boundaries documented in `docs/dependency-rules.md`.
2. Never add direct dependencies from `crates/core` to `runtime`, `adapters`, `plugins`, or `ffi` crates.
3. Keep unsafe code isolated:
   - Default: deny unsafe and undocumented unsafe blocks.
   - Exceptions are only for `crates/ffi/*` and explicitly designated adapter crates.
4. Favor adding interfaces in `ports` and message contracts in `messages` instead of cross-layer coupling.
5. Keep crates buildable as host skeletons; avoid target-specific code in shared crates.

## Change checklist

- Update dependency-rules documentation when adding/changing crate edges.
- Validate workspace integrity with `cargo metadata` and `cargo check --workspace`.
- Keep CI scope aligned to currently buildable slices.
