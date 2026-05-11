# Adapter Project Setup Testing

## Purpose of this repository

This workspace is an architecture-first foundation for building and validating two real application variants plus their Windows simulation counterparts:

- `apps/target5-app`: real Target5 application composition.
- `apps/target10-app`: real Target10 application composition.
- `apps/windows-target5-sim`: Windows simulation composition for Target5-oriented flows.
- `apps/windows-target10-sim`: Windows simulation composition for Target10-oriented flows.

The repository is designed to let teams evolve domain behavior, adapter integrations, and runtime composition safely while preserving strict layer boundaries and clear extension seams.

It also serves as an extension sandbox where new algorithms, control flows, adapters, plugin/provider implementations, and C/FFI integration surfaces can be added without violating dependency rules.

## Core architecture principles

This repository enforces three non-negotiable principles:

1. **Core purity**
   - `crates/core` is platform-agnostic domain logic and must depend only on `crates/ports` and `crates/messages`.
   - `core` must not depend on `runtime`, `adapters`, `plugins`, or `ffi` crates.

2. **Message contract centralization**
   - Cross-layer payloads belong in `crates/messages`.
   - Do not create ad hoc per-adapter or per-app contracts when payloads are exchanged across layers.

3. **Composition-only applications**
   - `apps/*` are composition roots only: wire dependencies, configuration, and startup.
   - Domain/business behavior belongs in `core` (and port-driven implementations), not in executable entrypoints.

For authoritative allowed/forbidden dependency edges, see [`docs/dependency-rules.md`](docs/dependency-rules.md).

## Folder ownership map

Use [`docs/folder-guide.md`](docs/folder-guide.md) as the canonical placement guide. Quick ownership summary:

- `apps/*`: executable composition roots.
- `crates/core/*`: domain model, algorithms, control flows.
- `crates/messages`: canonical cross-layer contracts.
- `crates/ports`: interfaces and abstractions used by `core` and implementations.
- `crates/runtime`: orchestration and lifecycle behavior.
- `crates/adapters/*`: concrete environment/protocol/target integrations.
- `crates/ffi/*`: C/ABI boundary crates and unsafe isolation zones.
- `crates/plugins/*`: plugin API/loader extension surfaces.


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

## Extension pathways

Common extension routes in this workspace:

- **Algorithm extension:** add domain decision logic in `crates/core/algorithms`.
- **Control-flow extension:** add or evolve orchestrated business flows in `crates/core/flows`.
- **Adapter extension:** add/modify concrete integrations in `crates/adapters/*` while preserving port/message contracts.
- **DLL/SO provider extension:** introduce provider/plugin loading behavior in `crates/plugins/*` and runtime composition where appropriate.
- **C FFI extension:** add ABI bindings/surfaces in `crates/ffi/*` and keep unsafe usage isolated there (or designated adapter crates).


### Algorithms vs flows placement example

Concrete Target5 → Target10 placement:

- `crates/core/src/algorithms/target5_to_target10.rs` keeps the pure mapping transform (`Target5Status` -> `Target10Command`).
- `crates/core/src/flows/target5_to_target10.rs` provides orchestration API and flow-level typed error surface for use-case callers.

Use this split whenever a transform remains stateless but call sequencing/policy ownership needs an explicit flow boundary.

## Linux roadmap

For Linux target-build planning and phased execution details, see:

- [`docs/linux-target-build-roadmap.md`](docs/linux-target-build-roadmap.md)

This roadmap is aspirational and **not** an active build contract yet.

## Optional validation/scenario tooling

The following are useful but **non-primary** architecture references:

- Scenario artifacts and replay inputs under `scenarios/`.
- Workspace/system validation under `tests/`.
- CI and automation policy/docs that validate buildable slices and boundary conformance.

Representative commands:

```bash
cargo metadata --format-version 1
cargo check --workspace
cargo run -p scenario-runner -- --help
```

Use optional tooling to validate behavior, but treat the architecture/dependency docs as the primary source of truth.

## Runtime profile matrix

| ProfileId | Input mode | Enabled transports | Disabled transports | Intent |
| --- | --- | --- | --- | --- |
| `Target5Real` | `Live` | `Ethernet` | `CommType1`, `CommType2`, `LoopbackEthernet` | Real Target5 hardware runtime. |
| `Target10Real` | `Live` | `Ethernet`, `CommType1`, `CommType2` | `LoopbackEthernet` | Real Target10 hardware runtime. |
| `WindowsTarget5Sim` | `Simulated` | `LoopbackEthernet` | `Ethernet`, `CommType1`, `CommType2` | Windows Target5 simulation runtime. |
| `WindowsTarget10Sim` | `Simulated` | `LoopbackEthernet`, `CommType1`, `CommType2` | `Ethernet` | Windows Target10 simulation runtime with simulated CommType1/CommType2 pathways. |
| `ReplayRunner` | `Replay` | `LoopbackEthernet` | `Ethernet`, `CommType1`, `CommType2` | Non-primary replay profile retained for scenario replay workflows. |
