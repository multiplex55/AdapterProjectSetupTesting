# Adapter Project Setup Testing

This repository is a Rust workspace used to validate adapter-oriented architecture setup, layering, and runtime behavior before full feature implementation.

## Workspace purpose and layer map

The workspace is intentionally split into architectural layers so contributors can evolve runtime behavior without violating domain boundaries:

- `crates/messages`: canonical message contracts (DTOs, events, request/response envelopes).
- `crates/ports`: domain-facing interfaces and capability traits.
- `crates/core`: platform-agnostic business/domain logic; depends only on `messages` + `ports`.
- `crates/adapters/*`: external-system implementations of ports (I/O, providers, targets).
- `crates/runtime`: composition and orchestration of core + selected adapters/plugins.
- `crates/plugins/*`: optional extension points and plugin loading surfaces.
- `crates/ffi/*`: explicitly isolated unsafe and ABI boundary crates.
- `apps/*`: thin executables that assemble runtime components.
- `scenarios/*`: scenario definitions and target simulation assets.
- `tests/*`: integration/contract tests.

Authoritative dependency rules are documented in `docs/dependency-rules.md`.

## Architecture constraints

- Dependency and layering constraints: `docs/dependency-rules.md`
- Architecture decision log policy (one ADR per significant tradeoff): `docs/adr/`

## Why `core` is platform-agnostic

`crates/core` represents domain intent and policy, so it must not embed platform concerns (OS, transport, simulator, FFI ABI, plugin loading strategy). This keeps domain behavior:

- testable on any host,
- reusable across Target5/Target10 variants,
- insulated from adapter/runtime churn.

When behavior needs platform data, add or evolve a port in `crates/ports`, then implement it in adapters.

## Why `cfg(...)` belongs at boundaries only

Conditional compilation is allowed at boundaries (`apps`, `runtime`, adapter/ffi edges) to select platform integrations. Avoid broad `cfg` branching inside shared domain crates because it creates hidden behavior matrices and weakens reproducibility.

Rule of thumb:

- **Boundary crates:** choose platform wiring with `cfg`.
- **Shared crates (`messages`, `ports`, `core`):** keep logic deterministic and platform-neutral.

## Current State

### Implemented paths (runnable now)

From repository root:

```bash
# Inspect workspace structure and dependency graph
cargo metadata --format-version 1 > /tmp/metadata.json

# Validate currently buildable host slices
cargo check --workspace

# Run the scenario runner utility
cargo run -p scenario-runner -- --help

# Run simulator app entrypoints
cargo run -p windows-target5-sim -- --help
cargo run -p windows-target10-sim -- --help

# Run app entrypoints
cargo run -p target5-app -- --help
cargo run -p target10-app -- --help
```

### Scaffold/planned paths (not yet runnable)

Use the following only as planned interface examples:

```bash
# Planned: scenario-specific CLI surfaces and options are still evolving.
# Planned: cargo run -p windows-target5-sim -- --scenario scenarios/target5
# Planned: cargo run -p windows-target10-sim -- --scenario scenarios/target10

# Planned: deterministic replay CLI contract is not finalized in scenario-runner.
# Planned: cargo run -p scenario-runner -- --input ./replays/sample.json --target target5
# Planned: cargo run -p scenario-runner -- --input ./replays/sample.json --target target10
```

## Provider fallback behavior

Provider fallback must be explicit and observable:

1. Attempt primary provider selected by runtime composition.
2. On explicit recoverable error, select configured secondary provider.
3. Emit structured message/event describing fallback reason and selected provider.
4. If no valid fallback exists, return explicit error (no silent success path).

No hidden fallback paths are allowed in `core`; fallback policy belongs in runtime/adapters with message-level visibility.

## Target5/Target10 relationship and communication matrix

Target5 and Target10 share domain contracts (`messages`, `ports`, `core`) and diverge at adapter/runtime boundary.

| From \ To | Core | Runtime | Target5 Adapter | Target10 Adapter | External Systems |
| --- | --- | --- | --- | --- | --- |
| Core | N/A | via ports only | no direct | no direct | no direct |
| Runtime | invokes core ports | N/A | yes | yes | orchestrates through adapters |
| Target5 Adapter | no direct | callback/events | N/A | no direct | Target5-specific protocols |
| Target10 Adapter | no direct | callback/events | no direct | N/A | Target10-specific protocols |

Principle: Target-to-target communication is mediated through runtime/message contracts, not direct adapter coupling.

## Explicit "not implemented yet"

The following are intentionally incomplete and should be treated as roadmap items:

- Final simulator command options may still change.
- Full provider catalog and fallback priority configuration are not finalized.
- Cross-target conformance suite coverage is partial.

Use this repository as an architecture scaffold first; feature completeness is secondary at this stage.

## Contributor verification checklist

- [ ] Run `cargo metadata --format-version 1`
- [ ] Run `cargo check --workspace`
- [ ] Keep `README.md` package command examples aligned to actual workspace package names.
- [ ] Document non-trivial architecture tradeoffs in `docs/adr/`.

See also `docs/testing-scope.md` for host vs target-only expectations.
