# Agent Guardrails: Architecture Integrity

## Primary rules

1. Preserve clean architecture boundaries documented in `docs/dependency-rules.md`.
2. Enforce **core purity**: never add direct dependencies from `crates/core` to `runtime`, `adapters`, `plugins`, or `ffi` crates.
3. Enforce **centralized messages**: cross-layer payload contracts belong in `crates/messages`; avoid ad hoc per-adapter contract drift.
4. Keep unsafe code isolated:
   - Default posture is deny unsafe and undocumented unsafe blocks.
   - Exceptions are only for `crates/ffi/*` and explicitly designated adapter crates.
5. Enforce explicit errors and no hidden fallbacks:
   - Failure paths must surface explicit, typed errors.
   - Fallback behavior must be deliberate, observable, and message-visible.
   - Silent fallback/suppression is prohibited.
6. Keep app mains composition-only:
   - `apps/*` should wire dependencies and start runtime.
   - Domain/business logic belongs in `core` (or port implementations), not executable entrypoints.
7. Avoid giant feature-flag matrices:
   - Prefer boundary composition (crate selection/runtime wiring) over deeply nested shared-crate `cfg` trees.
   - Keep shared crates deterministic and host-buildable.
8. Keep crates buildable as host skeletons; avoid target-specific code in shared crates.

## Governance and decision logging

- Record future architecture tradeoffs in `docs/adr/` (one ADR per decision).
- Keep governance docs aligned with actual crate dependency edges and runtime behavior.

## Change checklist

- Update `docs/dependency-rules.md` when adding/changing crate edges.
- Validate workspace integrity with:
  - `cargo metadata`
  - `cargo check --workspace`
- Keep CI scope aligned to currently buildable slices.
- Ensure new contributor guidance remains sufficient to avoid boundary violations.
