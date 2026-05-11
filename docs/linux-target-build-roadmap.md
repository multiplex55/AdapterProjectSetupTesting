# Linux Target Build Roadmap (Aspirational)

> **Status: Not implemented yet**
>
> This document is a forward-looking roadmap. It is **not** an active build contract and does not replace current build/test instructions.

## Scope and intent

- **Status: Not implemented yet**
- Define planned Linux target-build structure for Target5 and Target10 application variants.
- Keep architecture boundaries intact while adding Linux support incrementally.
- Preserve core purity: no platform-specific logic in `crates/core` or shared domain/message crates.

## Intended future build commands (Target5 / Target10)

- **Status: Not implemented yet**
- Tentative command pattern for Target5 app build:

  ```bash
  cargo build -p target5-app --target <TENTATIVE_TARGET5_TRIPLE>
  ```

- Tentative command pattern for Target10 app build:

  ```bash
  cargo build -p target10-app --target <TENTATIVE_TARGET10_TRIPLE>
  ```

- These commands are placeholders for planning only and should not be treated as currently supported workflows.

## Placeholder target triples (tentative)

- **Status: Not implemented yet**
- The following are explicit placeholders and may change during implementation:
  - `TENTATIVE_TARGET5_TRIPLE=x86_64-unknown-linux-gnu`
  - `TENTATIVE_TARGET10_TRIPLE=aarch64-unknown-linux-gnu`
- Final target triples must be confirmed against deployment/runtime constraints before activation in CI.

## Planned `.cargo/config.toml` location and responsibilities

- **Status: Not implemented yet**
- Future location:
  - Repository root: `.cargo/config.toml`
- Planned contents (high-level):
  - Named target configuration blocks for approved Linux target triples.
  - Optional linker/toolchain configuration required per target.
  - Target-specific rustflags only where necessary and documented.
- This file should remain a build-composition artifact; it must not introduce domain behavior or platform policy into shared crates.

## Placement for Linux-specific adapter and integration code

- **Status: Not implemented yet**
- Linux-specific implementation code should live in boundary crates such as:
  - `crates/adapters/*` for protocol/device/transport integration details.
  - Platform integration crates (for example, under `crates/adapters/` or other boundary-focused crate roots) when Linux runtime coupling is required.
- Runtime wiring should remain in composition/orchestration layers (`apps/*` + `crates/runtime`) and call into ports/adapters explicitly.

## Where Linux-specific code must not go

- **Status: Not implemented yet**
- Do **not** place Linux-specific logic in:
  - `crates/core/*` (algorithms, flows, domain state/models).
  - `crates/messages` (canonical cross-layer contracts).
  - Other shared domain/message crates that must remain host-buildable and platform-agnostic.
- Maintain centralized messages and core purity rules from `docs/dependency-rules.md`.

## Activation criteria before this roadmap becomes active contract

- **Status: Not implemented yet**
- Before moving from roadmap to active contract:
  - Confirm target triples and toolchains.
  - Add concrete build commands to active contributor/build docs.
  - Validate workspace integrity with:
    - `cargo metadata`
    - `cargo check --workspace`
  - Ensure CI covers supported Linux target slices without violating dependency rules.
