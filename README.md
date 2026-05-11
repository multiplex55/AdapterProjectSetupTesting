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

## Extension pathways

Common extension routes in this workspace:

- **Algorithm extension:** add domain decision logic in `crates/core/algorithms`.
- **Control-flow extension:** add or evolve orchestrated business flows in `crates/core/flows`.
- **Adapter extension:** add/modify concrete integrations in `crates/adapters/*` while preserving port/message contracts.
- **DLL/SO provider extension:** introduce provider/plugin loading behavior in `crates/plugins/*` and runtime composition where appropriate.
- **C FFI extension:** add ABI bindings/surfaces in `crates/ffi/*` and keep unsafe usage isolated there (or designated adapter crates).

## Linux roadmap

For Linux target-build planning and phased execution details, see:

- [`docs/linux-target-build-roadmap.md`](docs/linux-target-build-roadmap.md)

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
