# Adapter Project Setup Testing

This repository is organized as a Rust workspace with architecture-oriented boundaries.

## Architecture overview (placeholder)

- `crates/messages`: shared DTOs and wire-safe message definitions.
- `crates/ports`: abstractions and interfaces consumed by the domain.
- `crates/core`: domain logic depending only on ports/messages.
- `crates/runtime`: composition root wiring core + adapters + plugin loader.
- `crates/adapters/*`: platform and infrastructure implementations against ports.
- `crates/ffi/*`: isolated unsafe FFI boundary crates.
- `crates/plugins/*`: plugin API and loading support.
- `apps/*`: executable entry points and tooling.
- `scenarios/*`: scenario definitions.
- `tests/*`: integration and contract test surfaces.

See `docs/dependency-rules.md` for dependency constraints.


See `docs/testing-scope.md` for dev-host vs target-only test expectations and CI commands.
