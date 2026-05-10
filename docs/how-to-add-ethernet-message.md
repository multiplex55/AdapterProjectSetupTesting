# How to Add an Ethernet Message (Target5 ↔ Target10)

This guide documents the end-to-end workflow for introducing a new message that crosses Target5, Ethernet transport, and Target10 boundaries without contract drift.

## Goals

- Keep payload contracts centralized in `crates/messages`.
- Ensure the domain-level semantic shape and transport-level representation stay aligned.
- Ensure replay and integration coverage are updated with each new message.

## Complete worked example: `Target5Heartbeat`

Use this section as a concrete template when introducing a new message.

### 1) Add shared semantic model

**File:** `crates/messages/src/common/mod.rs`

Add a shared semantic model used by both endpoint-specific modules and transport mappings.

Example shape:

- `Target5Heartbeat`
  - `sequence: u32`
  - `uptime_ms: u64`
  - `healthy: bool`

Guidance:

- Keep this semantic and platform-agnostic.
- Do not encode Ethernet field names or framing details here.
- Prefer explicit field names/units (for example `uptime_ms`, not `uptime`).

### 2) Add Target5 message model and conversion touchpoint

**File:** `crates/messages/src/target5/mod.rs`

Add Target5-side representation and conversion boundary to/from the shared model.

Typical touchpoints:

- `Target5Message::Heartbeat(Target5Heartbeat)` (or equivalent enum variant)
- `impl From<Target5HeartbeatWireOrLocal> for common::Target5Heartbeat` (or `TryFrom` when validation is needed)
- `impl From<common::Target5Heartbeat> for Target5HeartbeatWireOrLocal`

Validation rules should be explicit:

- Invalid Target5 values return typed conversion errors.
- No default/fallback hydration of missing required fields.

### 3) Add Ethernet transport representation and mapping

**File:** `crates/messages/src/ethernet/mod.rs`

Add the wire-facing shape and mapping for `Target5Heartbeat`.

Typical touchpoints:

- Ethernet payload enum/struct variant for heartbeat.
- `TryFrom<target5::...> for ethernet::...` and reverse mappings.
- Any transport-only framing metadata stays local to this module.

Mapping expectations:

- Field parity with `common::Target5Heartbeat` is exact for required fields.
- Optional transport decorations are additive and must not mutate semantic values.
- Unsupported encodings return explicit typed errors.

### 4) Export from message crate public surface

**File:** `crates/messages/src/lib.rs`

Ensure downstream crates can consume the new message and mappings by updating exports.

Typical touchpoints:

- Public module exports for updated `common`, `target5`, and `ethernet` items.
- Re-export aliases/helpers used by integration tests and adapters.

Rule:

- Keep exports explicit; avoid broad wildcard re-exports that hide API changes.

### 5) Update replay schema for scenario validation

**File:** `scenarios/integration/target5_to_target10/replay-schema.json`

Add schema entry for the new heartbeat event shape used in replay fixtures.

Expected schema updates:

- Add a `Target5Heartbeat` event discriminator (or extend existing enum).
- Add `sequence`, `uptime_ms`, `healthy` with explicit type/required constraints.
- If units/ranges are constrained (for example non-negative uptime), encode them in schema.

Checklist:

- Existing scenarios remain valid.
- New heartbeat fixture validates against schema.
- Schema rejects malformed heartbeat payloads.

### 6) Add loopback assertion in integration test

**File:** `tests/integration/tests/target5_target10_loopback.rs`

Add a loopback test case that sends `Target5Heartbeat` over Target5 → Ethernet → Target10 flow.

Assertion shape to include:

- Arrange: construct a heartbeat input with concrete values.
- Act: pass through loopback path.
- Assert semantic parity at receive side:
  - `sequence` unchanged
  - `uptime_ms` unchanged
  - `healthy` unchanged
- Assert explicit error path for malformed/unsupported heartbeat payload (no silent fallback).

---

## General step-by-step workflow

1. **Define a common shared type (when semantics are shared).**
   - File: `crates/messages/src/common/mod.rs`
2. **Add the Target5-side message model.**
   - File: `crates/messages/src/target5/mod.rs`
3. **Add the Target10-side message model (if required by route).**
   - File: `crates/messages/src/target10/mod.rs`
4. **Add the Ethernet transport representation.**
   - File: `crates/messages/src/ethernet/mod.rs`
5. **Export public API.**
   - File: `crates/messages/src/lib.rs`
6. **Update replay schema and loopback integration tests.**
   - Files:
     - `scenarios/integration/target5_to_target10/replay-schema.json`
     - `tests/integration/tests/target5_target10_loopback.rs`

## Mapping checklist (contract drift prevention)

Use this checklist before merging:

- [ ] **Required fields parity** across `target5`, `target10` (when applicable), `ethernet`, and `common`.
- [ ] **Enum/variant compatibility** is exhaustive and explicit.
- [ ] **Serialization expectations** (names/casing/optionality/units) are verified.
- [ ] **Failure behavior** returns typed errors and never silently falls back.

## Test reinforcement expectations

### Unit tests in `crates/messages`

Add/expand tests for:

- Serde round-trip stability.
- Conversion integrity across `target5`, `target10`, and `ethernet`.
- Typed conversion failures for unsupported or malformed payload forms.

### Integration replay coverage

Add scenario replay coverage ensuring:

- Fixtures include the new message.
- End-to-end loopback preserves semantics.
- Invalid fixtures fail explicitly.
