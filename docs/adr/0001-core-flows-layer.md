# ADR-0001: Introduce `core/flows` for domain orchestration

- **Status:** Accepted
- **Date:** 2026-05-12

## Context

Domain logic previously risked being split between pure algorithms in `crates/core/src/algorithms`, runtime orchestration in `crates/runtime`, and ad hoc sequencing in app entrypoints. This blurred ownership of use-case sequencing and made boundary drift more likely.

We need a dedicated domain-layer place for orchestration that:

- sequences multiple domain steps;
- can request external effects only through ports/contracts;
- stays independent from adapter/runtime implementation details.

## Decision

Introduce `crates/core/src/flows` as the canonical location for domain use-case orchestration.

`core/flows` modules:

- compose deterministic algorithm calls and domain-state transitions;
- emit effect values/contracts that outer layers interpret;
- surface explicit typed flow-level errors;
- do **not** call adapters, runtime services, plugins, or FFI directly.

## Consequences

### Positive

- Clarifies boundary ownership: algorithms vs orchestration vs runtime dispatch.
- Reduces business logic leakage into `apps/*` and `runtime`.
- Improves testability (flow behavior can be tested without host/runtime wiring).

### Tradeoffs

- Adds one more conceptual layer for contributors to learn.
- Requires discipline to keep flow interfaces stable and contract-driven.

## Alternatives considered

1. Keep orchestration in `runtime`.
   - Rejected: mixes domain policy with host/lifecycle concerns.
2. Keep orchestration in app entrypoints.
   - Rejected: duplicates policy across apps and weakens reuse.
3. Encode orchestration only as algorithm chaining.
   - Rejected: obscures explicit effect boundaries and flow-level errors.
