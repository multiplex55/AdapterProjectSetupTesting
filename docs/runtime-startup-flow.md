# Runtime Startup Flow

This document describes startup orchestration implemented in [`crates/runtime/src/startup.rs`](../crates/runtime/src/startup.rs).

## Source of truth in code

- Startup orchestration: [`crates/runtime/src/startup.rs`](../crates/runtime/src/startup.rs).
- Provider resolution ordering and decisions: [`crates/runtime/src/provider_registry.rs`](../crates/runtime/src/provider_registry.rs).
- App entrypoint wiring examples: [`apps/target5-app/src/main.rs`](../apps/target5-app/src/main.rs), [`apps/windows-target10-sim/src/main.rs`](../apps/windows-target10-sim/src/main.rs).
- Startup/provider contract tests: [`tests/integration/tests/target5_target10_loopback.rs`](../tests/integration/tests/target5_target10_loopback.rs), [`tests/plugin_contract/tests/plugin_contract.rs`](../tests/plugin_contract/tests/plugin_contract.rs).

## Startup stages

1. **Config validation**
   - Startup immediately fails with `StartupValidationFailed` if `plugin_search_paths` is empty.
2. **Profile hydration**
   - Runtime creates an `AppProfile` using `AppProfile::new(config.profile)`.
   - Diagnostics are initialized from that profile and capture plugin search paths.
3. **Per-capability resolution loop**
   - Runtime iterates in fixed order: `Compute`, `Transport`, `Clock`.
   - For each capability, it computes `required` from `required_capabilities` and calls registry resolution.
4. **Diagnostics recording**
   - On success: selected provider metadata and fallback decision path are recorded.
   - On failure: failed fallback state is recorded before returning a typed startup error.
5. **Terminal outcome**
   - Success returns `StartupResult { diagnostics }`.
   - Failure returns one of the typed startup errors below.

## Typed startup errors

- `RequiredCapabilityMissing`
- `ProviderLoadFailed`
- `ProviderAbiMismatch`
- `ProviderSpecInvalid`
- `StartupValidationFailed`

## Sequence-style narrative

```text
startup request
  -> validate config (plugin_search_paths non-empty)
  -> hydrate app profile + diagnostics
  -> for each capability (Compute, Transport, Clock)
       -> ask provider registry to resolve candidate
       -> registry evaluates sources in order and returns success/failure + decision path
       -> startup records diagnostics (selected provider + fallback status)
       -> on typed failure, startup returns typed StartupError immediately
  -> if all capabilities resolve, return StartupResult with diagnostics
```

## Related docs

- [Application profile matrix](./application-profile-matrix.md)
- [Provider requirement model](./provider-requirement-model.md)
- [App composition guide](./app-composition-guide.md)
- [How to debug startup provider failure](./how-to-debug-startup-provider-failure.md)
- [Replay scenario format](./replay-scenario-format.md)
- [How to add replay scenario](./how-to-add-replay-scenario.md)



## Canonical vs compatibility

The startup sequence and typed error model in runtime are canonical. Plugin loading behavior is intentionally staged (see [Plugin Loading Roadmap](./plugin-loading-roadmap.md)); this flow should not be read as a claim of fully completed dynamic loader support on every platform slice yet.

## Boundary guardrail reminder

Per [Dependency Rules](./dependency-rules.md), `crates/core` must stay free of platform-specific startup/composition logic. Keep orchestration in runtime and concrete behavior in adapters/plugins.
