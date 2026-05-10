# Architecture Decision Records (ADR)

Use this directory to capture notable architecture tradeoffs over time.

## When to add an ADR

Create an ADR when changing:

- crate dependency edges,
- fallback/runtime behavior,
- target integration boundaries,
- unsafe/FFI isolation rules,
- message/port ownership and contract strategy.

## Minimal ADR template

1. Title
2. Status
3. Context
4. Decision
5. Consequences
6. Alternatives considered

Keep ADRs concise and link them from relevant docs/PRs.


## Planned ADR stub: rename `core` package to `domain-core`

**TODO:** capture a formal ADR for renaming the package `core` to `domain-core`.

### Rationale (to formalize)
- Clarify that this crate is domain logic, not framework/runtime core infrastructure.
- Reduce onboarding ambiguity between `crates/core` and `crates/runtime` ownership.
- Improve boundary readability in dependency graphs and architecture docs.

### Migration considerations (to formalize)
- Cargo package rename impact across workspace manifests and path dependencies.
- Potential crate name aliasing/transition window to avoid breaking downstream integrations.
- CI, documentation, and examples updates (`cargo metadata`, architecture diagrams, and dependency rules).
- Communication and rollout plan for external consumers, if any.
