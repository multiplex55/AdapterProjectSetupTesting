# ADR-0002: Replace `SimulationState` with explicit flow and dispatch state

- **Status:** Accepted
- **Date:** 2026-05-12

## Context

A monolithic `SimulationState` abstraction encouraged conflating domain state with runtime/dispatch bookkeeping. This weakened clean boundaries and made ownership of mutations and failure modes less clear.

## Decision

Replace `SimulationState` with split, purpose-specific state:

- **Domain state** remains in core domain models and is mutated by `core/flows`.
- **Runtime dispatch/operational state** is owned by runtime components (for example effect dispatch bookkeeping).

All transitions must remain explicit through typed APIs and error types; no hidden fallback or silent suppression is allowed.

## Consequences

### Positive

- Preserves core purity and clear ownership boundaries.
- Makes failure paths and state mutation sites easier to audit.
- Simplifies reasoning about deterministic domain tests versus runtime integration behavior.

### Tradeoffs

- Requires migration effort where old `SimulationState` was used.
- May introduce additional adapter/runtime plumbing to pass state explicitly.

## Alternatives considered

1. Keep `SimulationState` and document better.
   - Rejected: naming/docs alone do not solve ownership ambiguity.
2. Move all state to runtime.
   - Rejected: would pull domain invariants out of core.
3. Keep split internals but a single façade type.
   - Rejected: still hides boundaries and encourages accidental coupling.
