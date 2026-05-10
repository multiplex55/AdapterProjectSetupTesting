# Dependency Rules

This document defines the allowed crate dependency graph.

## Allowed edges

- `messages` -> (none; standalone)
- `ports` -> `messages` (optional/minimal)
- `core` -> `ports`, `messages`
- `runtime` -> `core`, `ports`, `messages`, `plugins/loader`
- `adapters/*` -> `ports`, `messages`, optional `ffi/*`
- `plugins/loader` -> `plugins/api`, `messages`
- `plugins/api` -> `messages`
- `ffi/*` -> `messages` (optional)
- `apps/*` -> compose from `runtime`, adapters, and/or tools as needed

## Forbidden edges (non-exhaustive)

- `core` -> `runtime`
- `core` -> `adapters/*`
- `core` -> `plugins/*`
- `core` -> `ffi/*`
- `messages` -> any internal crate
- `ports` -> `core`, `runtime`, `adapters/*`, `plugins/*`, `ffi/*`

- Adapter implementations in-tree include `adapters/target5`, `adapters/target10`, `adapters/windows-sim`, and `adapters/ethernet`; Windows-specific simulation composition lives in `apps/windows-target*-sim`.

## Enforcement

- Workspace default lints deny `unsafe_code` and `clippy::undocumented_unsafe_blocks`.
- FFI and selected adapter crates explicitly opt into `unsafe_code` where necessary.
- CI runs workspace checks, Windows-slice build placeholders, and host-only vertical-slice integration/contract tests (`integration-tests`, `plugin-contract-tests`).


## Current concrete crates

- Target adapters: `adapters/target5`, `adapters/target10`, `adapters/commtype1`, `adapters/commtype2`; simulation adapters: `adapters/windows-sim` and `adapters/ethernet`.
- C interop: raw declarations in `ffi/target-bindings`; safe wrappers in `adapters/c-drivers`; optional outward C surface in `ffi/c-api`.
