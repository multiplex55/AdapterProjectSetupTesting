# How to Add Control Flow in `crates/core`

Use this guide when introducing or refactoring domain behavior that sequences work.

## Placement rule

- Put **pure/stateless transforms** in `crates/core/src/algorithms/*`.
- Put **orchestration, sequencing, and policy-level flow APIs** in `crates/core/src/flows/*`.

## Concrete before/after example

### Before (too much packed into algorithm module)

```rust
// crates/core/src/algorithms/target5_to_target10.rs
pub fn map_target5_status_to_target10_command(...) -> Result<Target10Command, ...>
```

When higher-level callers start depending on this as a use-case API, the module mixes low-level transformation intent with orchestration entrypoint usage.

### After (explicit split)

- Keep deterministic mapping in:
  - `crates/core/src/algorithms/target5_to_target10.rs`
- Add flow-level orchestration API in:
  - `crates/core/src/flows/target5_to_target10.rs`

```rust
pub fn orchestrate_target5_to_target10_command(
    status: &Target5Status,
) -> Result<Target10Command, Target5ToTarget10FlowError>
```

This preserves algorithm purity while giving use-case callers a flow-layer boundary with explicit typed error propagation.

## Checklist

1. Add/extend algorithm functions for deterministic transforms.
2. Add/extend flow module that sequences algorithm calls and exposes flow-level error types.
3. Export `flows` from `crates/core/src/lib.rs`.
4. Keep app mains (`apps/*/src/main.rs`) composition-only; do not move flow logic there.
