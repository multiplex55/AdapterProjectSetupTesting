# Testing Scope: Dev Host vs Target-only

## Dev-host-ready CI commands

Use these commands for host-executable slices (`core`, `ports`, `messages`, `runtime`, Windows simulators, and related adapters):

- `cargo fmt --all --check`
- `cargo clippy -p core -p ports -p messages -p runtime -p adapter-windows-target5 -p adapter-windows-target10 -p adapter-windows-sim -p adapter-sim-transport -p adapter-target5 -p adapter-target10 --all-targets -- -D warnings`
- `cargo test -p integration-tests -p plugin-contract-tests -p adapter-windows-sim -p plugins-loader`
- `cargo metadata --format-version 1 > /dev/null`
- `cargo check --workspace`

## Target-only tests separation

- Target-only tests (hardware, vendor SDK, or non-host toolchain) must run in dedicated target CI jobs.
- Dev-host CI must remain deterministic and not require target-specific compilers/drivers.
- Replay files in `scenarios/integration/target5_to_target10/` are treated as contract fixtures for host deterministic tests.

## First vertical-slice acceptance criteria (executable/measurable)

1. Loopback mapping test passes for `windows-target5-sim` -> `windows-target10-sim`.
2. Replay-driven deterministic output test passes with ordered output assertions.
3. Plugin fallback and ABI-mismatch behavior are detected by contract tests.
4. Any regression in protocol behavior or plugin fallback semantics fails CI.
