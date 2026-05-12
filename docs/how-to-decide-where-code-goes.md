# How to Decide Where Code Goes

See also: [Dependency rules](./dependency-rules.md) · [Folder guide](./folder-guide.md) · [App composition guide](./app-composition-guide.md)

Use this routing table before adding new files. If a change touches more than one layer, keep each concern in its boundary and connect them through ports/messages.

## Routing table (concrete)

| Concern | Put it in | What it should contain | Must not contain |
|---|---|---|---|
| DTOs / cross-layer payload contracts | `crates/messages` | Serializable request/response/event types shared across boundaries | Runtime singletons, adapter handles, platform-specific pointers |
| Pure algorithms | `crates/core/src/algorithms` | Deterministic transforms, validation rules, calculations | IO, retries, network calls, adapter imports |
| Use-case flows / orchestration | `crates/core/src/flows` | Step ordering across ports + algorithms, explicit typed flow errors | Direct adapter calls, hidden fallback paths |
| Domain state | `crates/core` (domain model modules) | Business state and invariants independent of host/platform | Thread/runtime handles, sockets, process globals |
| Port contracts | `crates/ports` | Traits/interfaces that define effects core can request | Concrete adapter implementations |
| Runtime wiring and effect execution | `crates/runtime` | Startup, lifecycle, scheduling, adapter/plugin composition via ports | New domain rules that belong in core |
| Adapter implementations | `adapters/*` | Concrete transport/device/protocol implementations of ports | Core orchestration or new message schema drift |
| FFI boundaries | `ffi/*` (+ designated adapter wrappers) | Unsafe interop surface and ABI translation | Domain logic, business flow policy |
| Plugin APIs/loaders | `plugins/api`, `plugins/loader` | Plugin interfaces, discovery, loading contracts | Core domain rules, adapter-only details |
| App wiring / executable entrypoints | `apps/*` | Dependency composition, profile selection, runtime start | Business logic, algorithm internals |

## Fast decision checklist

1. **Is it shared payload across layers/processes?** Put it in `crates/messages`.
2. **Is it pure and deterministic?** Put it in `core/algorithms`.
3. **Is it sequencing decisions across multiple steps/effects?** Put it in `core/flows`.
4. **Does it touch OS/hardware/network/plugin loading?** Keep it in `runtime`, `adapters/*`, `ffi/*`, or `plugins/*`.
5. **Is it only startup/config wiring?** Keep it in `apps/*`.

## Guardrails to preserve architecture

- Keep `core` pure: no direct dependencies to `runtime`, `adapters/*`, `plugins/*`, or `ffi/*`.
- Keep cross-layer contract types centralized in `crates/messages`.
- Keep failures explicit and typed; never hide fallback behavior.
- Keep target/platform forks out of `core`; fork only at adapter/wiring boundaries.

## Optional/later

If you are planning deterministic diagnostics or simulation playback, see replay/scenario docs: [Replay scenario format](./replay-scenario-format.md) and [How to add replay scenario](./how-to-add-replay-scenario.md). This is optional and can be adopted later.
