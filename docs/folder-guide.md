# Folder Guide (Canonical Placement)

This is the canonical file-placement guide for the workspace. It is aligned to [`docs/dependency-rules.md`](./dependency-rules.md) and is intentionally explicit about what belongs where.

## `apps/target5-app`

### Purpose
Composition root for the real Target5 application.

### What belongs here
- Startup wiring for runtime + selected adapters.
- App-local CLI/config parsing and bootstrapping.

### What does not belong here
- Domain/business logic that belongs in `crates/core/*`.
- Canonical cross-layer payload schemas (must live in `crates/messages`).

### Example changes that should go here
- Add a startup flag that selects a runtime profile.
- Wire a Target5 adapter implementation into app composition.

## `apps/target10-app`

### Purpose
Composition root for the real Target10 application.

### What belongs here
- Target10 executable startup/config composition.
- Runtime + adapter assembly for Target10 deployment.

### What does not belong here
- Core domain algorithms or flow logic.
- Adapter implementation internals.

### Example changes that should go here
- Add Target10 app CLI options.
- Update Target10 runtime wiring for provider selection.

## `apps/windows-target5-sim`

### Purpose
Windows simulation app composition for Target5-oriented behavior.

### What belongs here
- Simulator executable wiring.
- Runtime + simulation adapter composition for Target5 simulation slice.

### What does not belong here
- Shared domain rules (`crates/core/*`).
- Canonical message contracts.

### Example changes that should go here
- Add simulator startup options for Target5 simulation mode.
- Wire simulation adapter set for Windows-based Target5 runs.

## `apps/windows-target10-sim`

### Purpose
Windows simulation app composition for Target10-oriented behavior.

### What belongs here
- Simulation executable startup and composition.
- Runtime and adapter wiring for Target10 simulation workflows.

### What does not belong here
- Business logic, policy decisions, or domain transitions.
- Cross-layer DTO/schema ownership.

### Example changes that should go here
- Add Target10 simulation entrypoint argument handling.
- Compose runtime with windows-sim and transport adapters.

## `crates/core/algorithms`

### Purpose
Deterministic domain algorithms and decision logic.

### What belongs here
- Pure business calculations and rule evaluation.
- Logic that is portable across targets and environments.

### What does not belong here
- Direct dependencies on runtime/adapters/plugins/ffi.
- Transport/protocol/OS-specific behavior.

### Example changes that should go here
- Add a domain scoring algorithm.
- Refine deterministic validation logic for a domain operation.

Placement example:
- `algorithms/target5_to_target10.rs`: pure status->command transform logic.
- `flows/target5_to_target10.rs`: orchestration API that sequences transform usage and surfaces flow-level errors.

## `crates/core/flows`

### Purpose
Domain control flows and use-case orchestration at core layer.

### What belongs here
- Port-driven domain orchestration logic.
- Explicit error-propagating flow sequencing.

### What does not belong here
- Concrete adapter calls bypassing ports.
- Runtime startup/lifecycle composition concerns.

### Example changes that should go here
- Add a new domain use-case flow that coordinates multiple ports.
- Update flow-level typed error handling for explicit failure paths.

## `crates/core/domain`

### Purpose
Domain entities, value objects, and state-transition models.

### What belongs here
- Core domain types and invariants.
- State models and validation rules independent of host/target.

### What does not belong here
- App CLI/config concerns.
- Adapter protocol or ABI binding details.

### Example changes that should go here
- Introduce a new domain value type with invariants.
- Extend domain state transition definitions.

## `crates/messages`

### Purpose
Canonical cross-layer message/payload contracts.

### What belongs here
- DTOs/events/envelopes exchanged between layers.
- Versioned shared contract schemas.

### What does not belong here
- Dependencies on internal workspace crates.
- Business policy logic or adapter-specific behavior.

### Example changes that should go here
- Add a new shared event payload consumed by runtime and adapters.
- Version a request/response contract used across boundaries.

## `crates/ports`

### Purpose
Abstract interfaces used by core and implemented by runtime/adapters/plugins.

### What belongs here
- Traits/capability interfaces.
- Boundary abstractions that keep core decoupled from implementations.

### What does not belong here
- Dependencies on `core`, `runtime`, `adapters/*`, `plugins/*`, or `ffi/*`.
- Concrete implementation logic.

### Example changes that should go here
- Add a new trait for provider capability.
- Extend an existing port interface with explicit error returns.

## `crates/runtime`

### Purpose
Runtime orchestration, lifecycle sequencing, and operational coordination.

### What belongs here
- Startup sequencing, diagnostics, and orchestration logic.
- Composition internals that sit beneath app entrypoints.

### What does not belong here
- Core domain algorithms and entity rules.
- Ownership of canonical cross-layer payload types.

### Example changes that should go here
- Add runtime startup validation sequencing.
- Add explicit fallback orchestration with observable message emission.

## `crates/adapters/target5`

### Purpose
Concrete Target5 integration implementations.

### What belongs here
- Target5 protocol and environment-specific port implementations.
- Mapping logic between Target5 integration details and canonical messages/ports.

### What does not belong here
- Core business rule ownership.
- Cross-layer contract ownership outside `crates/messages`.

### Example changes that should go here
- Implement a Target5 provider adapter.
- Add Target5-specific translation layer to canonical payloads.

## `crates/adapters/target10`

### Purpose
Concrete Target10 integration implementations.

### What belongs here
- Target10-specific port implementations and protocol integration.
- Translation between Target10 details and shared contracts.

### What does not belong here
- Domain policy logic.
- Runtime/app composition entrypoint concerns.

### Example changes that should go here
- Add a Target10 transport adapter implementation.
- Update Target10 integration mapping behavior.

## `crates/adapters/windows-sim`

### Purpose
Simulation-oriented adapter implementations used by Windows simulator app slices.

### What belongs here
- Simulation data sources and behavior implementations for port boundaries.
- Deterministic simulation integration components.

### What does not belong here
- Business/domain logic.
- Canonical shared contract definitions.

### Example changes that should go here
- Add simulation provider behavior for a new port.
- Improve deterministic simulator event generation.

## `crates/adapters/ethernet`

### Purpose
Ethernet transport adapter implementations.

### What belongs here
- Ethernet-specific protocol/transport integration.
- Port implementation code that encapsulates ethernet I/O concerns.

### What does not belong here
- App/runtime composition mains.
- Cross-layer message contract ownership.

### Example changes that should go here
- Add ethernet link handling implementation.
- Introduce typed ethernet adapter errors surfaced through ports.

## `crates/adapters/commtype1`

### Purpose
Concrete adapter implementations for commtype1 integration.

### What belongs here
- commtype1 protocol handling and port implementation logic.
- Mapping from commtype1 transport data to canonical messages.

### What does not belong here
- Shared domain invariants/algorithms.
- Direct dependence from core into this adapter.

### Example changes that should go here
- Add commtype1 request/response handling adapter.
- Add commtype1-specific adapter diagnostics.

## `crates/adapters/commtype2`

### Purpose
Concrete adapter implementations for commtype2 integration.

### What belongs here
- commtype2 protocol and integration logic.
- Port implementation for commtype2 communication pathways.

### What does not belong here
- Canonical payload schema ownership.
- Domain flow rules that belong in `crates/core/flows`.

### Example changes that should go here
- Add commtype2 transport translation logic.
- Update commtype2 adapter typed error mapping.

## `crates/adapters/c-drivers`

### Purpose
Adapter-side wrappers around C driver integrations at the adapter boundary.

### What belongs here
- Safe(ish) adapter-facing wrappers over FFI bindings.
- Port implementations that consume driver-level capabilities.

### What does not belong here
- Raw shared contract ownership.
- Core domain rules or app composition.

### Example changes that should go here
- Add driver wrapper integration for a new adapter capability.
- Improve conversion/error mapping around C driver calls.

## `crates/ffi`

### Purpose
C/ABI boundary crates and isolated unsafe interop surfaces.

### What belongs here
- Raw bindings, ABI declarations, and conversion shims.
- Explicitly documented unsafe blocks required for FFI boundaries.

### What does not belong here
- High-level business logic.
- Cross-layer schema ownership that should live in `crates/messages`.

### Example changes that should go here
- Add new external symbol bindings.
- Extend C API boundary conversion with typed error reporting.

## `crates/plugins`

### Purpose
Plugin API and loader extension surfaces.

### What belongs here
- Plugin contracts/interfaces and loader-side integration.
- Provider extension mechanisms that runtime composes.

### What does not belong here
- Direct `core` dependencies on plugin implementations.
- Canonical cross-layer payload ownership when used beyond plugin-local contracts.

### Example changes that should go here
- Add plugin capability metadata.
- Extend plugin loader validation behavior.

## Optional validation/scenario/CI references (non-primary)

This section is intentionally non-primary and exists for supporting validation workflows.

- Scenario assets: `scenarios/`
- Integration/contract/system tests: `tests/`
- Build/test automation and CI policy docs: repository CI configuration + testing scope docs

Primary placement decisions should be made using the sections above and validated against [`docs/dependency-rules.md`](./dependency-rules.md).
