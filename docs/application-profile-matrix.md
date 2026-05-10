# Application Profile Matrix

This document maps runtime app profiles to their communication posture and intended binaries.

## Source of truth

- Runtime profile definitions: [`crates/runtime/src/app_profile.rs`](../crates/runtime/src/app_profile.rs).
- App composition entrypoints: [`apps/target5-app/src/main.rs`](../apps/target5-app/src/main.rs), [`apps/target10-app/src/main.rs`](../apps/target10-app/src/main.rs), [`apps/windows-target5-sim/src/main.rs`](../apps/windows-target5-sim/src/main.rs), [`apps/windows-target10-sim/src/main.rs`](../apps/windows-target10-sim/src/main.rs).

## Profile matrix

| Profile ID | Input mode | Enabled comm types | Disabled comm types | Intended app(s) |
|---|---|---|---|---|
| `target5-real` | `Live` | `Ethernet`, `Serial` | `Loopback` | `apps/target5-app` |
| `target10-real` | `Live` | `Ethernet`, `Serial` | `Loopback` | `apps/target10-app` |
| `windows-target5-sim` | `Simulated` | `Loopback` | `Ethernet`, `Serial` | `apps/windows-target5-sim` |
| `windows-target10-sim` | `Simulated` | `Loopback` | `Ethernet`, `Serial` | `apps/windows-target10-sim` |
| `replay-runner` | `Replay` | `Loopback` | `Ethernet`, `Serial` | `apps/tools/scenario-runner` and replay-oriented startup paths |

## Why this matters for composition-only app mains

Application mains under `apps/*` are composition roots: they select a profile and wire runtime startup inputs, but they should not implement domain behavior. Keeping profile behavior centralized in runtime avoids duplicating transport/input policy across app binaries.

That separation ensures:

- consistent profile behavior across all binaries;
- lower risk of app-specific profile drift;
- clean architecture boundaries where domain logic stays in `crates/core`.

## Related docs

- [Runtime startup flow](./runtime-startup-flow.md)
- [Provider requirement model](./provider-requirement-model.md)
- [App composition guide](./app-composition-guide.md)
- [How to debug startup provider failure](./how-to-debug-startup-provider-failure.md)
