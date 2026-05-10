# Integration Tests

This crate hosts executable integration tests for the first vertical slice between `windows-target5-sim` and `windows-target10-sim`.

## Scope

- `target5_target10_loopback.rs`
  - loopback message exchange between target5 status and target10 command mapping
  - replay-driven deterministic output behavior

## Target-only separation

- Host-executable tests belong here and must avoid target-specific toolchains.
- Hardware/target-only tests are intentionally out of scope and should live in target-specific CI jobs.
- Replay files under `scenarios/integration/target5_to_target10/` are the canonical deterministic input contracts for host tests.
