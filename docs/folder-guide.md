# Folder Guide

This guide describes what belongs in each major workspace directory and what must stay out. It complements the dependency policy in [`docs/dependency-rules.md`](./dependency-rules.md).

## Related docs

- [Dependency Rules](./dependency-rules.md)
- [Testing Scope](./testing-scope.md)
- [CLI Argument Contracts](./cli-argument-contracts.md)
- [How to run boundary checks](./how-to-run-boundary-checks.md)

## `apps/`

### Responsibilities
- Composition roots and executable entrypoints.
- Wire runtime + adapters + configuration and start processes.
- Host tools (`apps/tools/*`) for operational checks and dev workflows.

### Forbidden dependencies/concerns
- No domain/business logic in mains.
- Do not move protocol contract ownership here; contracts belong in `crates/messages`.
- Avoid bypassing runtime orchestration with direct cross-layer coupling.

### Typical changes
- Add CLI flags and startup wiring to a specific app.
- Add a new composition binary for a new deployment profile.

## `crates/core`

### Responsibilities
- Domain model, state transitions, algorithms, and service-level business rules.
- Deterministic, host-buildable logic that depends only on stable abstractions.

### Forbidden dependencies/concerns
- **Core purity is mandatory:** `crates/core` must not depend on `runtime`, `adapters/*`, `plugins/*`, or `ffi/*`.
- No target-specific code paths or runtime composition code.
- No hidden fallbacks; errors should remain explicit and typed.

### Typical changes
- Add a new domain service algorithm.
- Extend deterministic state transition logic.

## `crates/ports`

### Responsibilities
- Interface/port traits and boundary abstractions used by core and implementations.
- Shared abstractions for clocks, telemetry, transport, providers, and data sources.

### Forbidden dependencies/concerns
- Must not depend on concrete runtime/adapters/plugins/ffi implementations.
- Must not absorb domain logic that belongs to `core`.

### Typical changes
- Add a new trait method to a provider abstraction.
- Introduce a new port module for a new integration seam.

## `crates/messages`

### Responsibilities
- Canonical message and payload contracts used across layers.
- Versioning and shared message schema modules.

### Forbidden dependencies/concerns
- Must remain standalone and not depend on internal workspace crates.
- Do not duplicate cross-layer payload definitions in adapters/apps/runtime.

### Typical changes
- Add a new wire payload struct used by multiple layers.
- Extend schema versioning metadata.

## Message contract centralization

Cross-layer payload contracts **must live in `crates/messages`**. If multiple layers exchange a payload, define and version it in `messages` rather than introducing per-adapter or per-app variants. This prevents contract drift and keeps compatibility reviews centralized.

## `crates/runtime`

### Responsibilities
- Runtime orchestration, lifecycle management, diagnostics, health, and startup composition internals.
- Dependency assembly that sits beneath app composition entrypoints.

### Forbidden dependencies/concerns
- Do not embed core domain rules that belong in `crates/core`.
- Do not define cross-layer payload schemas here.

### Typical changes
- Add startup sequencing logic.
- Extend diagnostics and host lifecycle integration.

## `crates/adapters/*`

### Responsibilities
- Concrete implementations of ports for environment/protocol/platform integration.
- Simulation and transport/device-specific boundary code.

### Forbidden dependencies/concerns
- Avoid owning canonical message contracts.
- Avoid leaking adapter-specific assumptions into `core`.
- Unsafe use should be minimal and explicitly documented, with most unsafe isolated to designated crates.

### Typical changes
- Implement a transport adapter for a new target family.
- Add simulation adapter behavior for replay/test scenarios.

## `crates/ffi/*`

### Responsibilities
- FFI boundaries, binding surfaces, and interop wrappers.
- Explicit unsafe isolation zones for C/ABI interaction.

### Forbidden dependencies/concerns
- Do not spread unsafe interop requirements into `core`, `ports`, or `messages`.
- Avoid introducing business logic into FFI crates.

### Typical changes
- Add new external symbol bindings.
- Extend a C API surface with explicit conversion/error handling.

## `crates/plugins/*`

### Responsibilities
- Plugin APIs and loading/runtime integration for plugin extension points.
- Plugin contract and loader behavior.

### Forbidden dependencies/concerns
- `core` must not depend directly on plugins.
- Do not place cross-layer payload ownership here if used outside plugin boundaries.

### Typical changes
- Add plugin capability negotiation metadata.
- Extend loader-side validation behavior.

## `scenarios/`

### Responsibilities
- Deterministic scenario artifacts (sample inputs, replay JSON, schemas, docs).
- Shared fixtures for integration/system-level validation.

### Forbidden dependencies/concerns
- No production business logic.
- Avoid storing unstable/generated artifacts without clear ownership.

### Typical changes
- Add a new replay sample and corresponding schema.
- Document a new deterministic integration scenario.

## `tests/`

### Responsibilities
- Workspace-level integration and contract tests that validate cross-crate behavior.
- Boundary and policy enforcement tests (including dependency-boundary checks).

### Forbidden dependencies/concerns
- Avoid turning tests into duplicate production implementations.
- Keep host-only tests host-buildable and deterministic unless explicitly target-scoped.

### Typical changes
- Add an integration test for end-to-end message flow.
- Add policy tests to assert architectural guardrails.

## Adapter naming audit note

The previously ambiguous adapter crates `crates/adapters/windows-target5` and `crates/adapters/windows-target10` were removed as redundant skeletons. Real target composition now uses `adapter-target5`/`adapter-target10`, and Windows simulation composition uses `adapter-windows-sim` + `adapter-ethernet` in app-level wiring.
