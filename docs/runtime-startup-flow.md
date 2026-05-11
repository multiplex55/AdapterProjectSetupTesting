# Runtime Startup Flow

This document describes startup lifecycle orchestration in [`crates/runtime/src/startup.rs`](../crates/runtime/src/startup.rs) and how runtime dispatches effects from core flows into adapters.

## Source of truth in code

- Startup orchestration: [`crates/runtime/src/startup.rs`](../crates/runtime/src/startup.rs).
- Provider resolution ordering and decisions: [`crates/runtime/src/provider_registry.rs`](../crates/runtime/src/provider_registry.rs).
- App entrypoint wiring examples: [`apps/target5-app/src/main.rs`](../apps/target5-app/src/main.rs), [`apps/windows-target10-sim/src/main.rs`](../apps/windows-target10-sim/src/main.rs).

## Startup lifecycle

1. **Config validation**
   - Validate required startup inputs (for example plugin search path requirements).
   - Return typed startup validation errors when requirements are not met.
2. **Profile hydration**
   - Build runtime profile state from app-selected profile ID.
   - Initialize diagnostics and runtime state containers.
3. **Capability/provider resolution**
   - Resolve required providers in deterministic capability order.
   - Record selected provider and explicit fallback path in diagnostics.
4. **Runtime activation**
   - Start orchestrators that coordinate core flows and adapter-backed capabilities.
   - Runtime is now ready to receive flow requests and dispatch effects.

## Dispatch path: core flow -> runtime -> adapters

```text
app main (composition only)
  -> runtime startup lifecycle
  -> core flow executes domain logic
  -> core flow emits effect requests (via ports/messages contracts)
  -> runtime dispatches effects to selected adapter implementations
  -> adapters perform integration I/O and return typed outcomes
  -> runtime reports results/diagnostics to caller
```

This keeps `core` platform-agnostic and prevents illegal direct dependencies from core to runtime/adapters/ffi/plugins.

## Typed error posture

Runtime startup and dispatch paths must surface explicit, typed failures (for example validation/provider/ABI failures) rather than silent fallbacks or hidden suppression.

## Optional / later validation tooling

The following are optional validation aids and not primary startup ownership:

- scenario-runner/replay narratives;
- replay-scenario authoring workflows;
- CI slices that exercise replay-oriented paths.

Use these after lifecycle and dispatch boundaries are correct.

## Related docs

- [Application profile matrix](./application-profile-matrix.md)
- [Provider requirement model](./provider-requirement-model.md)
- [App composition guide](./app-composition-guide.md)
- [How to debug startup provider failure](./how-to-debug-startup-provider-failure.md)
