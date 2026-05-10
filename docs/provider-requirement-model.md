# Provider Requirement Model

This document explains requirement/fallback semantics across runtime startup and provider resolution.

## Source of truth in code

- Runtime startup orchestration: [`crates/runtime/src/startup.rs`](../crates/runtime/src/startup.rs).
- Provider source ordering and resolution: [`crates/runtime/src/provider_registry.rs`](../crates/runtime/src/provider_registry.rs).
- Provider contracts and load errors: [`crates/plugins/api/src/lib.rs`](../crates/plugins/api/src/lib.rs), [`crates/plugins/loader/src/lib.rs`](../crates/plugins/loader/src/lib.rs).
- Runtime/provider boundary tests: [`tests/integration/tests/target5_target10_loopback.rs`](../tests/integration/tests/target5_target10_loopback.rs), [`tests/plugin_contract/tests/plugin_contract.rs`](../tests/plugin_contract/tests/plugin_contract.rs).

## Core concepts

1. **Required capability**
   - The capability must resolve by the end of source ordering; otherwise startup must fail.
   - Expressed by inclusion in `StartupConfig.required_capabilities`.

2. **Required external provider**
   - A specific explicit provider must load/validate successfully.
   - In current behavior, when capability is required and explicit candidate exists, explicit failure is terminal for that capability.

3. **Fallback allowed**
   - Resolution may continue to later sources if an earlier source is missing/rejected and fallback is not blocked.
   - Current source order is:
     `explicit -> discovered -> builtin-optimized -> builtin-reference`.

4. **Fallback forbidden**
   - Resolution stops and returns typed error on blocked conditions.
   - Current mechanics: required+explicit evaluation is strict and may block further fallback.

## Mapping to current runtime mechanics

- Startup side: [`crates/runtime/src/startup.rs`](../crates/runtime/src/startup.rs)
  - determines whether each capability is required;
  - invokes registry `resolve` per capability;
  - maps provider errors to typed startup errors and records diagnostics.

- Registry side: [`crates/runtime/src/provider_registry.rs`](../crates/runtime/src/provider_registry.rs)
  - evaluates provider sources in deterministic order;
  - validates ABI/spec and attempts load;
  - returns decision path entries showing missing/rejected/selected outcomes.

## Important clarification

A capability may be required even when an **external DLL/SO provider is optional**.

If explicit/discovered external candidates are absent or not selected, built-in providers (`builtin-optimized` or `builtin-reference`) can still satisfy the capability requirement, as long as the required-capability resolution reaches a successful selection.

## When startup must fail vs may continue

### Must fail

- `plugin_search_paths` is empty (`StartupValidationFailed`).
- Required capability cannot be resolved across all sources (`RequiredCapabilityMissing`).
- Required explicit candidate fails under strict explicit requirement path (mapped typed errors include ABI/spec/load variants).

### May continue with fallback

- A non-terminal source candidate is missing/rejected and later source candidates remain.
- Optional capabilities can resolve to no concrete provider (`none-optional`) when not required.

## Diagnostics expectations

Diagnostics should make fallback decisions observable:

- selected provider and selected source per capability;
- fallback status (`Succeeded`/`Attempted`/`Failed`);
- detailed decision path showing source-by-source outcomes in order.

## Related docs

- [Runtime startup flow](./runtime-startup-flow.md)
- [Application profile matrix](./application-profile-matrix.md)
- [App composition guide](./app-composition-guide.md)
- [How to debug startup provider failure](./how-to-debug-startup-provider-failure.md)

## Canonical vs compatibility

Provider resolution behavior documented here is canonical for runtime startup sequencing and typed-error semantics. Plugin loading is a staged roadmap (see [Plugin Loading Roadmap](./plugin-loading-roadmap.md)); avoid assuming fully implemented, production-complete dynamic loading across all targets today.

## Boundary guardrail reminder

As defined in [Dependency Rules](./dependency-rules.md), keep platform-specific logic and runtime composition concerns out of `crates/core`. Core remains deterministic and host-buildable, while runtime/adapters/plugins own environment-specific behavior.
