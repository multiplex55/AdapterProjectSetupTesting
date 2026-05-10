# How to Add an Ethernet Message (Target5 ↔ Target10)

This guide documents the end-to-end workflow for introducing a new message that crosses Target5, Ethernet transport, and Target10 boundaries without contract drift.

## Goals

- Keep payload contracts centralized in `crates/messages`.
- Ensure the domain-level semantic shape and transport-level representation stay aligned.
- Ensure replay and integration coverage are updated with each new message.

## Step-by-step workflow

1. **Define a common shared type (when semantics are shared).**
   - File: `crates/messages/src/common/mod.rs`
   - Add or extend shared structs/enums used by both Target5 and Target10.
   - Keep this layer semantic (not transport-specific) and reusable across boundaries.

2. **Add the Target5-side message model.**
   - File: `crates/messages/src/target5/mod.rs`
   - Introduce the Target5 representation that produces or consumes the new message.
   - Use shared `common` types where appropriate to avoid duplicate definitions.

3. **Add the Target10-side message model.**
   - File: `crates/messages/src/target10/mod.rs`
   - Introduce the Target10 representation for the same message intent.
   - Mirror required semantics from Target5/common while preserving Target10-specific modeling constraints.

4. **Add the Ethernet transport representation.**
   - File: `crates/messages/src/ethernet/mod.rs`
   - Define the wire/transport-facing representation for the new message.
   - Include conversion helpers between Ethernet form and Target5/Target10/domain models.

5. **Export public API.**
   - File: `crates/messages/src/lib.rs`
   - Re-export new modules/types/converters needed by downstream crates.
   - Keep the public surface explicit and discoverable.

## Mapping checklist (contract drift prevention)

Use this checklist before merging:

- [ ] **Required fields parity**
  - Every required field is represented consistently across:
    - `target5`
    - `target10`
    - `ethernet`
    - `common` (when applicable)

- [ ] **Enum/variant compatibility**
  - Variants are mapped exhaustively and intentionally.
  - Unknown/unsupported variants return explicit typed errors (no silent fallback).

- [ ] **Serialization format expectations**
  - Field names, casing, optionality, and numeric/string formats are explicitly verified.
  - Backward/forward compatibility expectations are documented where relevant.

## Replay schema update

When adding a new replayable message, update:

- `scenarios/integration/target5_to_target10/replay-schema.json`

Recommended steps:

1. Add the new message shape and constraints to the schema.
2. Validate existing scenarios still pass schema validation.
3. Add/update schema examples if maintained alongside the schema.

## Test update instructions

Update integration coverage in:

- `tests/integration/tests/target5_target10_loopback.rs`

Recommended steps:

1. Add a loopback case that includes the new message in the Target5 → Ethernet → Target10 path.
2. Assert semantic parity at receive side (required fields, enums, and units).
3. Assert explicit error behavior for incompatible/unsupported payload forms.

## Test reinforcement expectations

### Unit tests in `crates/messages`

Add or expand tests for:

- Serde round-trip stability (serialize → deserialize → equality/semantic parity).
- Schema-conformance helpers (if present), including both success and failure cases.
- Conversion integrity between `target5`, `target10`, and `ethernet` representations.

### Integration replay coverage

Add an integration test case using the new message in scenario replay:

- Ensure replay fixtures include the message.
- Ensure replay execution validates the message end-to-end through loopback behavior.
- Ensure failures are explicit and typed when fixtures violate expected contracts.

## Done criteria

A new Ethernet message is considered complete when:

- Models are added in `common` (if needed), `target5`, `target10`, and `ethernet`.
- Exports are added in `lib.rs`.
- Replay schema is updated.
- Loopback integration test is updated.
- `crates/messages` unit tests cover serde and schema/conversion behavior.
