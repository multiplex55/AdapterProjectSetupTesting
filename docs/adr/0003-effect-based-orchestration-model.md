# ADR-0003: Adopt an effect-based orchestration model

- **Status:** Accepted
- **Date:** 2026-05-12

## Context

Use cases must trigger IO and platform actions while preserving core determinism and portability. Direct IO calls from core would violate dependency rules and make host-targeted builds harder to keep deterministic.

## Decision

Adopt effect-based orchestration:

- `core/flows` return explicit effect values (contracts) instead of executing side effects directly.
- `runtime` interprets and dispatches effects through port/adapters.
- Cross-layer payload contracts remain centralized in `crates/messages`.
- Effect dispatch failures are surfaced as explicit typed errors.

## Consequences

### Positive

- Keeps core deterministic and free of adapter/runtime dependencies.
- Makes side effects observable and testable as returned effect values.
- Supports composition across host and target profiles without core branching.

### Tradeoffs

- Introduces effect type design/maintenance overhead.
- Requires clear evolution strategy for effect/message contracts.

## Alternatives considered

1. Service-locator style calls from core to runtime/adapters.
   - Rejected: violates core purity and hides dependencies.
2. Callback-heavy orchestration with implicit side effects.
   - Rejected: weak observability and test clarity.
3. Per-adapter bespoke contracts.
   - Rejected: causes contract drift; conflicts with centralized messages rule.
