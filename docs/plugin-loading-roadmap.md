# Plugin Loading Roadmap

This document describes current plugin-loading behavior and the staged path toward real dynamic library loading, while keeping present tests and guards meaningful.

## Current state (today)

Current startup/registry flow already enforces contract-oriented plugin/provider resolution behavior:

- deterministic provider source ordering,
- typed startup/load failures,
- explicit diagnostics about selection/fallback path,
- ABI/spec validation surfaces in loader/runtime error taxonomy.

This gives us observable, testable guardrails even before full production dynamic loading is complete.

## Explicit non-goal

**Non-goal right now:** implementing a full, production-grade real DLL/SO loading stack across all targets/environments.

We are intentionally not claiming end-to-end real-loader completeness yet. The priority is enforcing contract integrity, typed failures, and deterministic startup behavior first.

## Roadmap stages

## Stage 1 — Contract-first baseline (current/near-term)

Focus:

- stabilize plugin API descriptors/function-table contracts,
- keep symbol naming/version expectations explicit,
- ensure runtime/provider registry semantics are deterministic and test-covered.

Required guardrails:

- no trait objects across ABI boundary,
- typed `LoadError` / `StartupError` outcomes,
- no silent fallback; optional fallback remains observable in diagnostics.

Observability + error expectations:

- every failed contract check maps to explicit typed error class,
- diagnostics capture attempt source/path and fallback decisions.

## Stage 2 — Validation-first hardening

Focus:

- deepen structural and semantic validation before integration complexity,
- expand negative-path coverage (missing symbols, ABI mismatch, malformed spec, capability mismatch),
- enforce required-vs-optional capability behavior consistently.

Required guardrails:

- richer validation paths still produce stable typed error taxonomy,
- startup failure points remain explicit and reproducible,
- test fixtures cover malformed and partial provider ecosystems.

Observability + error expectations:

- diagnostics must record which validation stage failed,
- errors must include actionable context (expected vs found, symbol/capability identity).

## Stage 3 — Real loader integration (incremental)

Focus:

- integrate actual platform DLL/SO loading incrementally,
- preserve existing contract and validation checkpoints,
- keep cross-platform behavior explicit and staged by build slice.

Integration principles:

- do not bypass existing typed error/diagnostics model,
- keep loader behavior deterministic where possible (including search path and precedence semantics),
- add platform-specific handling without polluting shared core crates.

Observability + error expectations:

- real loader failures map into existing typed categories (or explicitly added typed categories),
- diagnostics continue to show attempt path/source and fallback trail.

## Stage 4 — Operational hardening and policy

Focus:

- tighten operational policies (version compatibility windows, rollout constraints),
- improve startup-time reporting and supportability tooling,
- align CI/build matrix with mature loader slices.

Observability + error expectations:

- policy violations produce typed, visible startup failures,
- support workflows can classify failures directly from diagnostics + error types.

## Keeping current tests/guards meaningful during roadmap execution

Across all stages:

- keep contract tests authoritative for boundary invariants,
- keep startup/provider resolution tests authoritative for required/optional semantics,
- avoid replacing typed errors with generic or opaque failures,
- avoid introducing silent fallback/suppression paths.

If new behavior is introduced, add explicit tests first (or in same change) and update docs to match runtime behavior.

## Architecture boundary reminders

- `core` must not depend on runtime/adapters/plugins/ffi crates.
- Shared cross-layer payload contracts belong in `crates/messages`.
- App mains remain composition-only; plugin loading policy is runtime concern.

Reference: `docs/dependency-rules.md`.
