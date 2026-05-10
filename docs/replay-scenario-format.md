# Replay Scenario Format

This document defines the **canonical** replay scenario format for integration scenarios.

## Source of truth in code

- Replay schema and artifacts: [`scenarios/integration/target5_to_target10/replay-schema.json`](../scenarios/integration/target5_to_target10/replay-schema.json), [`scenarios/integration/target5_to_target10/sample-replay.json`](../scenarios/integration/target5_to_target10/sample-replay.json).
- Compatibility parser (non-canonical): [`crates/adapters/windows-sim/src/replay.rs`](../crates/adapters/windows-sim/src/replay.rs).
- Scenario-driven runtime wiring: [`apps/tools/scenario-runner/src/main.rs`](../apps/tools/scenario-runner/src/main.rs), [`crates/runtime/src/startup.rs`](../crates/runtime/src/startup.rs).
- Replay-focused tests: [`tests/integration/tests/target5_target10_loopback.rs`](../tests/integration/tests/target5_target10_loopback.rs).

Canonical replay artifacts are JSON documents validated against the schema at:

- [`scenarios/integration/target5_to_target10/replay-schema.json`](../scenarios/integration/target5_to_target10/replay-schema.json)

## Canonical format and compatibility mode

- Canonical: schema-driven JSON replay files.
- Compatibility-only helper: line-based replay parsing in [`crates/adapters/windows-sim/src/replay.rs`](../crates/adapters/windows-sim/src/replay.rs), specifically `DeterministicReplay::from_lines`.

Line-based replay remains available only for backward compatibility and is **non-canonical** for new scenarios.

## Required top-level objects

A canonical replay JSON document must include all required top-level objects below:

- `scenario`
- `profile`
- `protocol_version`
- `events`
- `expected`

The schema enforces all of these via required fields and `additionalProperties: false` at top level.

## Top-level object expectations

### `scenario`

Describes scenario identity and intent.

Required fields:

- `id` (non-empty string)
- `name` (non-empty string)
- `description` (non-empty string)

### `profile`

Declares the source/target profile pairing the scenario is built for.

Required fields:

- `source`
- `target`

For current target5-to-target10 integration scenarios, enum values are constrained by the schema.

### `protocol_version`

Pins protocol family and semantic version components for compatibility checks.

Required fields:

- `family`
- `major`
- `minor`

### `events`

Ordered replay inputs.

- Must contain at least one event.
- Each event requires:
  - `timestamp_ms` (integer >= 0)
  - `kind`
  - `payload`

Validation and authoring expectations:

- Event `kind` must match schema enum values.
- `timestamp_ms` should be monotonic non-decreasing across the event list to preserve deterministic replay semantics.

### `expected`

Declares expected outputs and final state after consuming all events.

Required fields:

- `outputs`
- `final_state`

`outputs` is a list of expected emitted messages; `final_state` captures deterministic summary state checks.

## Validation expectations

Replay JSON files should be treated as invalid unless they pass schema validation against:

- [`scenarios/integration/target5_to_target10/replay-schema.json`](../scenarios/integration/target5_to_target10/replay-schema.json)

Recommended practice:

1. Author/update replay JSON.
2. Validate against schema before running integration tests.
3. Reject malformed or contract-drifting payloads early.

## Reference examples

- Valid sample: [`scenarios/integration/target5_to_target10/sample-replay.json`](../scenarios/integration/target5_to_target10/sample-replay.json)
- Intentionally malformed sample: [`scenarios/integration/target5_to_target10/malformed-replay.json`](../scenarios/integration/target5_to_target10/malformed-replay.json)


## Boundary guardrail reminder

Keep replay parsing and platform/runtime orchestration out of `crates/core` per [Dependency Rules](./dependency-rules.md). Core should consume stable contracts (for example via `crates/messages`) rather than host-specific parser behavior.
