# App Composition Guide

This guide documents what `apps/*` mains should do (and should not do), with concrete references to current binaries.

## Composition responsibilities in app mains

App mains should:

- parse app-level arguments and runtime startup options;
- choose profile and required capabilities;
- construct explicit provider map and startup config;
- invoke runtime startup and report diagnostics.

Referenced examples:

- [`apps/target5-app/src/main.rs`](../apps/target5-app/src/main.rs)
- [`apps/target10-app/src/main.rs`](../apps/target10-app/src/main.rs)
- [`apps/windows-target5-sim/src/main.rs`](../apps/windows-target5-sim/src/main.rs)
- [`apps/windows-target10-sim/src/main.rs`](../apps/windows-target10-sim/src/main.rs)

## Composition-only rule

`apps/*` mains are composition roots, not domain containers.

- Keep domain/business logic in `crates/core` and port implementations.
- Do not duplicate or relocate core algorithms into binaries.
- Keep startup/profile/provider orchestration delegated to `crates/runtime`.

Windows simulation apps currently demonstrate this split: they perform startup composition and orchestration, while domain mapping logic is consumed from core crate APIs.

## Practical checklist for new app mains

- Select an existing runtime `ProfileId` (or add one in runtime with docs updates).
- Provide non-empty `plugin_search_paths`.
- Declare required capabilities explicitly.
- Keep provider wiring explicit and deterministic.
- Emit startup diagnostics for observability.

## Related docs

- [Application profile matrix](./application-profile-matrix.md)
- [Runtime startup flow](./runtime-startup-flow.md)
- [Provider requirement model](./provider-requirement-model.md)
- [How to debug startup provider failure](./how-to-debug-startup-provider-failure.md)
