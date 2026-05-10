# How to Add a Replay Scenario

This guide explains how to author and wire a new replay scenario using the canonical JSON replay contract.

## Prerequisites

Review the canonical format first:

- [Replay Scenario Format](./replay-scenario-format.md)

Reference artifacts:

- Schema: [`scenarios/integration/target5_to_target10/replay-schema.json`](../scenarios/integration/target5_to_target10/replay-schema.json)
- Valid sample: [`scenarios/integration/target5_to_target10/sample-replay.json`](../scenarios/integration/target5_to_target10/sample-replay.json)
- Malformed sample: [`scenarios/integration/target5_to_target10/malformed-replay.json`](../scenarios/integration/target5_to_target10/malformed-replay.json)

## Step-by-step flow

1. **Choose profile pair**
   - Set `profile.source` and `profile.target` for the integration path you are modeling.
   - Keep values aligned to schema constraints for the scenario family.

2. **Craft events with monotonic `timestamp_ms`**
   - Add ordered entries under `events`.
   - Each event must include `timestamp_ms`, `kind`, and `payload`.
   - Keep `timestamp_ms` monotonic non-decreasing to preserve deterministic execution order.

3. **Set expected outputs and `final_state`**
   - Define expected emitted messages under `expected.outputs`.
   - Define deterministic state assertions under `expected.final_state`.
   - Ensure expectations cover both output behavior and terminal state consistency.

4. **Validate against schema**
   - Validate your JSON scenario against `replay-schema.json` before test integration.
   - Treat validation failures as contract failures; do not bypass by relying on legacy line-based formats.

5. **Integrate into tests**
   - Add or update integration tests to load and execute the new scenario.
   - Ensure test assertions verify both emitted outputs and final-state expectations.
   - Keep scenario files and tests colocated with the relevant integration slice for discoverability.

## Notes on canonical vs compatibility replay

- New scenarios must use canonical schema-driven JSON.
- The line-based parser helper (`DeterministicReplay::from_lines`) is compatibility-only and non-canonical.
