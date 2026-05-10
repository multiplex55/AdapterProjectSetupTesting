# How to run dependency boundary checks

This workspace includes an integration test that asserts `crates/core` does not depend on forbidden layers (`runtime`, `adapters/*`, `plugins/*`, `ffi/*`).

## Run only the boundary check

```bash
cargo test -p integration-tests core_has_no_forbidden_workspace_edges
```

## Run all integration tests

```bash
cargo test -p integration-tests
```

## What it checks

- Calls `cargo metadata --format-version 1`.
- Locates the workspace package rooted at `crates/core/Cargo.toml`.
- Verifies direct workspace dependencies of `core` do not include:
  - `runtime`
  - `adapter-*`
  - `plugins-*`
  - `ffi-*`

If a forbidden edge is introduced, the test fails with the exact offending dependency names.
