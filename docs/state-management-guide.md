# State Management Guide

See also: [Dependency rules](./dependency-rules.md) · [Runtime startup flow](./runtime-startup-flow.md)

This guide defines where state lives and who owns it.

## Core rules

1. **No random globals**
   - Do not introduce mutable global state as a convenience cache or hidden registry.
   - Any process-wide state must have an explicit owner module and lifecycle entry/exit.
2. **Per-process state ownership**
   - Runtime owns process lifecycle state (startup mode, running services, shutdown coordination).
   - Core owns domain state/invariants required for business behavior.
   - Adapters own transport/device/session state needed to implement ports.
3. **Shared types vs non-shared memory**
   - Shared data contracts across boundaries belong in `crates/messages` as value types.
   - In-memory synchronization primitives, handles, and buffers remain local to the owning crate.
   - Never leak adapter-private memory/handle types into `core` or `messages`.
4. **Boundary-specific state**
   - **Domain state (`core`)**: business facts/rules; deterministic; host-agnostic.
   - **Runtime state (`runtime`)**: orchestration/lifecycle, worker ownership, scheduling context.
   - **Adapter state (`adapters/*`, `ffi/*`)**: connection handles, protocol sessions, device internals.

## Practical patterns

- Pass state explicitly through constructors and function parameters.
- Prefer typed state containers over maps-of-anything.
- Model failure states explicitly via typed errors/events in flows/messages.
- Keep synchronization local to runtime/adapters; core APIs should stay deterministic.

## Anti-patterns

- Mutable static singleton in shared crates.
- Core structs storing adapter-specific handles or raw pointers.
- Silent state resets/fallbacks that hide lifecycle faults.

## Minimal ownership template

- `apps/*`: build dependency graph.
- `runtime`: own long-lived process components.
- `core`: own domain model/state transitions.
- `adapters/*`: own external system/session state.
- `messages`: define boundary-safe value payloads only.
