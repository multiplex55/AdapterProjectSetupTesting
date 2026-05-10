# Runtime Startup Flow

This document describes startup orchestration implemented in [`crates/runtime/src/startup.rs`](../crates/runtime/src/startup.rs).

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
