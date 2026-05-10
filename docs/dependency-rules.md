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

## Enforcement

- Workspace default lints deny `unsafe_code` and `clippy::undocumented_unsafe_blocks`.
- FFI and selected adapter crates explicitly opt into `unsafe_code` where necessary.
- CI runs workspace checks and Windows-slice build placeholders.
