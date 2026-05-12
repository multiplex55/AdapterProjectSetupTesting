# Windows vs Target Development Guide

See also: [Dependency rules](./dependency-rules.md) · [Application profile matrix](./application-profile-matrix.md)

## Principle

Use the **same core logic** for Windows and target builds. Change adapters and composition, not domain rules.

## What stays identical

- `crates/messages`: shared payload contracts.
- `crates/core`: algorithms, flows, domain state transitions.
- `crates/ports`: effect contracts that core depends on.

## What changes by environment

- `adapters/windows-sim` for host simulation/testing workflows.
- target adapters (`adapters/target5`, `adapters/target10`, etc.) for real hardware/protocol integration.
- `apps/*` wiring/profile selection deciding which adapter set is active.

## Hard rules

- No platform forks in `core`.
- No target-specific `cfg` trees in shared domain crates.
- Keep unsafe/platform ABI handling confined to `ffi/*` and designated adapters.

## Delivery workflow

1. Add/adjust shared message contracts.
2. Implement/extend core algorithm/flow once.
3. Implement required port behavior in both Windows-sim and target adapters as needed.
4. Wire environment-specific app profiles without duplicating domain logic.
5. Validate workspace integrity checks.

## Optional/later

If you want parity checks between Windows simulation and target runs, optional replay/scenario tooling can be introduced later (see [How to add replay scenario](./how-to-add-replay-scenario.md)).
