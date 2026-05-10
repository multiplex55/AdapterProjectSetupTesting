# How to Add a New Port and Adapter

This guide defines the standard flow for introducing a new capability contract in `crates/ports` and implementing it in `crates/adapters/*`.

## Boundary rules to preserve

Before making changes, re-check architecture constraints:

- `crates/core` must remain pure:
  - no dependencies from `core` to `runtime`, `adapters`, `plugins`, or `ffi`.
- Shared cross-layer payloads belong in `crates/messages`.
  - avoid ad hoc per-adapter payload drift.
- `crates/ports` contains capability contracts (traits + typed errors), not implementations.

Reference: `docs/dependency-rules.md`.

## 1) Add the port contract in `crates/ports`

Define (or extend) a port module under `crates/ports/src`:

- trait(s) expressing required behavior,
- explicit typed error enum(s),
- minimal, deterministic API surface.

Then expose items through `crates/ports/src/lib.rs`.

Guidelines:

- Keep contracts capability-focused and implementation-agnostic.
- Prefer explicit typed failures over implicit fallbacks.
- Do not couple port traits to concrete adapter/runtime/plugin/ffi types.

## 2) Add/align payload contracts in `crates/messages`

If the port needs request/response DTOs crossing layers:

- define/extend payload types in `crates/messages`,
- reference those types from port traits and adapters.

Do not place cross-layer message structs only inside an adapter crate.

## 3) Use the port from `crates/core`

`crates/core` may depend on `ports` (and `messages`) to express business behavior.

- Inject port abstractions into core workflows/services.
- Keep core free of concrete adapter knowledge.
- Surface typed errors at boundaries rather than suppressing failures.

## 4) Implement adapter in `crates/adapters/*`

Create or extend an adapter crate that implements the new port:

- map external system APIs/protocols into port contract,
- perform boundary validation,
- return explicit typed errors.

Unsafe code policy:

- avoid unsafe by default;
- if unavoidable, keep it isolated and documented in approved crates/surfaces.

## 5) Wire through runtime and app composition

Connect the adapter via runtime/app composition rather than hardwiring core:

- register/select provider/adapter from runtime startup wiring,
- choose required/optional capability posture explicitly,
- keep fallback behavior observable via diagnostics.

`apps/*` mains should remain composition-only and should not absorb domain logic.

## 6) Validation and docs

Update governance/docs to prevent future boundary drift:

- update `docs/dependency-rules.md` if crate dependency edges changed,
- add/update relevant how-to docs and startup flow docs,
- record significant architecture tradeoffs in `docs/adr/`.

Run required workspace checks:

- `cargo metadata`
- `cargo check --workspace`

## Quick checklist

- [ ] Port trait + typed errors added in `crates/ports`.
- [ ] Cross-layer payloads added/updated in `crates/messages`.
- [ ] Core uses port abstractions only (no forbidden dependencies).
- [ ] Adapter implementation added in `crates/adapters/*`.
- [ ] Runtime/apps wiring updated with explicit startup behavior.
- [ ] Typed error + diagnostics behavior preserved (no silent fallback).
- [ ] Dependency docs/checks updated and validated.
