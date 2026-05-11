# Application Profile Matrix

This document maps runtime profiles to **real-vs-sim adapter selection** and intended app entrypoints.

## Source of truth

- Runtime profile definitions: [`crates/runtime/src/app_profile.rs`](../crates/runtime/src/app_profile.rs).
- App composition entrypoints: [`apps/target5-app/src/main.rs`](../apps/target5-app/src/main.rs), [`apps/target10-app/src/main.rs`](../apps/target10-app/src/main.rs), [`apps/windows-target5-sim/src/main.rs`](../apps/windows-target5-sim/src/main.rs), [`apps/windows-target10-sim/src/main.rs`](../apps/windows-target10-sim/src/main.rs).

## Profile matrix (real vs sim adapter posture)

| Profile ID | Environment | Primary adapter posture | Intended app(s) |
|---|---|---|---|
| `target5-real` | Real hardware | Real Target5 + real transport adapters (no sim loopback path) | `apps/target5-app` |
| `target10-real` | Real hardware | Real Target10 + real transport adapters | `apps/target10-app` |
| `windows-target5-sim` | Simulation | Windows simulation adapters + loopback/sim transports | `apps/windows-target5-sim` |
| `windows-target10-sim` | Simulation | Windows simulation adapters + simulated CommType pathways | `apps/windows-target10-sim` |

## Composition-only main reminder

Application `main.rs` files select a profile and wire runtime + adapters; they do not implement domain rules. Domain behavior remains in `crates/core`, which depends only on `ports` and `messages` per [`docs/dependency-rules.md`](./dependency-rules.md).

## Optional / later validation tooling

The following are optional and not part of primary profile ownership:

- Replay/scenario runner paths.
- Scenario assets under `scenarios/`.
- CI narratives about replay-oriented checks.

Use these as secondary validation layers after profile wiring is correct.

## Related docs

- [Runtime startup flow](./runtime-startup-flow.md)
- [Provider requirement model](./provider-requirement-model.md)
- [App composition guide](./app-composition-guide.md)
- [How to debug startup provider failure](./how-to-debug-startup-provider-failure.md)
