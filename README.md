# Adapter Project Setup Testing

## Primary repository story

This workspace is organized around **four application entrypoints** that compose the same architecture for different deployment contexts:

- `apps/target5-app`: real Target5 deployment composition.
- `apps/target10-app`: real Target10 deployment composition.
- `apps/windows-target5-sim`: Windows simulation composition for Target5-oriented behavior.
- `apps/windows-target10-sim`: Windows simulation composition for Target10-oriented behavior.

The primary goal is to keep domain behavior stable while letting runtime and adapter wiring vary by app profile.

## Crate responsibilities

- `crates/core`: pure domain model, algorithms, and flows. Depends only on `crates/ports` and `crates/messages`.
- `crates/runtime`: startup lifecycle, orchestration, and effect dispatch coordination.
- `crates/messages`: centralized cross-layer payload contracts.
- `crates/ports`: boundary traits/capabilities implemented by runtime/adapters/plugins.
- `crates/adapters/*`: concrete environment/protocol integrations.
- `crates/ffi/*`: C/ABI boundaries and isolated unsafe interop.
- `crates/plugins/*`: plugin contracts and loader extension points.

These responsibilities must remain aligned with [`docs/dependency-rules.md`](docs/dependency-rules.md).

## Composition-only `main.rs` rule

Every app `main.rs` under `apps/*` is a **composition root only**:

- select config/profile;
- wire runtime + adapter implementations;
- start runtime lifecycle.

Do **not** place domain/business logic in app entrypoints. Domain logic belongs in `crates/core` and must not gain dependencies on `runtime`, `adapters`, `ffi`, or `plugins`.

## Short glossary

- **Algorithm**: deterministic domain rule/calculation (usually in `crates/core/algorithms`).
- **Flow**: domain use-case orchestration sequence that coordinates algorithms and ports (usually in `crates/core/flows`).
- **Effect**: an outward action request produced by core flow outcomes and carried through runtime to adapter implementations.
- **Domain state**: business-level state owned by core types/invariants, independent of platform/runtime.
- **Runtime state**: process/lifecycle/diagnostic state managed by runtime startup and orchestration.
- **Adapter state**: integration-specific protocol/device/session state maintained inside concrete adapters.

## Folder ownership map

Use [`docs/folder-guide.md`](docs/folder-guide.md) as the canonical “put X here” placement guide.

## Runtime profiles

For profile-to-adapter selection behavior (real vs simulation wiring), see [`docs/application-profile-matrix.md`](docs/application-profile-matrix.md).

## Startup + dispatch path

For startup lifecycle and core-to-runtime-to-adapter dispatch framing, see [`docs/runtime-startup-flow.md`](docs/runtime-startup-flow.md).

## Choose the right guide

| Change you need | Guide |
| --- | --- |
| Add pure business rule logic | [`docs/how-to-add-algorithm.md`](docs/how-to-add-algorithm.md) |
| Add orchestration/use-case sequencing | [`docs/how-to-add-control-flow.md`](docs/how-to-add-control-flow.md) |
| Add a concrete integration crate | [`docs/how-to-add-adapter.md`](docs/how-to-add-adapter.md) |
| Implement a Target5 end-to-end feature | [`docs/how-to-add-target5-feature.md`](docs/how-to-add-target5-feature.md) |
| Implement a Target10 end-to-end feature | [`docs/how-to-add-target10-feature.md`](docs/how-to-add-target10-feature.md) |
| Add dynamic DLL/SO provider loading | [`docs/how-to-add-dll-so-backed-provider.md`](docs/how-to-add-dll-so-backed-provider.md) |
| Add C ABI wrapper/bindings | [`docs/how-to-add-c-ffi-wrapper.md`](docs/how-to-add-c-ffi-wrapper.md) |
| Unsure where code belongs | [`docs/folder-guide.md`](docs/folder-guide.md) |

## Optional / later validation tooling

These are useful, but secondary to the architecture and composition story above:

- `scenarios/`: optional scenario/replay assets.
- `tests/`: optional integration/contract validation suites.
- CI/policy docs: automation scope and enforcement references.

Use them for confidence-building and rollout validation, not as the primary architecture contract.
