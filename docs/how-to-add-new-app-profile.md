# How to Add a New App Profile

This guide walks through adding a new runtime profile end-to-end, from `ProfileId` definition to app wiring and scenario/doc updates.

Primary source files:

- `crates/runtime/src/app_profile.rs`
- `apps/*/src/main.rs`
- `docs/application-profile-matrix.md`
- `docs/runtime-startup-flow.md`

## Architecture intent

App profiles define startup composition posture (input mode + comm allowances). They are a runtime concern, not a domain/business-logic concern.

- Keep `apps/*` mains composition-only.
- Keep business logic in `crates/core` and port implementations.
- Keep profile policy centralized in runtime (`AppProfile::new`) so behavior does not drift between binaries.

## 1) Add a new `ProfileId`

Update `crates/runtime/src/app_profile.rs`:

1. Add a new enum variant to `ProfileId`.
2. Add a stable string mapping in `ProfileId::as_str`.
3. Add a match arm in `AppProfile::new` that sets:
   - `enabled_comms`
   - `disabled_comms`
   - `input_mode`

Guidelines:

- Use explicit allow/deny communication lists to keep startup behavior deterministic.
- Do not hide fallback behavior in profile composition; startup diagnostics should make all decisions observable.
- Reuse existing `InputMode` and `CommType` variants when possible; if adding new variants, update all dependent startup and diagnostics surfaces explicitly.

## 2) Wire profile usage from app composition roots

In the corresponding `apps/*/src/main.rs` entrypoint(s):

- Select the new `ProfileId` in startup config.
- Keep startup options explicit (plugin search paths, required capabilities, provider overrides).
- Avoid introducing domain logic in main.

App mains should only:

- parse args/config,
- choose profile + startup requirements,
- call runtime startup,
- report typed diagnostics/errors.

## 3) Update startup expectations and tests

A new profile affects runtime startup behavior and observable diagnostics. Validate:

- startup still enforces non-empty `plugin_search_paths`;
- required capabilities still fail with typed errors when unresolved;
- profile-specific transport posture appears correctly in startup diagnostics.

At minimum, run workspace integrity checks:

- `cargo metadata`
- `cargo check --workspace`

Add or update tests where profile-specific behavior is asserted.

## 4) Update docs and profile matrix

After adding a profile, update documentation so contributors can compose apps without boundary drift.

Required doc updates:

- `docs/application-profile-matrix.md`
  - add row for new profile (`ProfileId`, input mode, enabled/disabled comm types, intended app).
- `docs/app-composition-guide.md`
  - if the new profile changes composition expectations, update practical checklist text.
- `docs/runtime-startup-flow.md`
  - update if startup stages/typed errors/diagnostics semantics changed.
- `docs/dependency-rules.md`
  - only if crate dependency edges changed (often not needed for profile-only additions).

## 5) Update scenario artifacts when relevant

If the profile introduces or changes simulation/replay behavior, update scenario artifacts:

- `scenarios/*/README.md` for profile-specific usage and expectations.
- Replay assets and schema-linked examples when replay behavior is involved.
- Any scenario-runner docs that mention supported profile IDs or startup assumptions.

Keep examples synchronized so profile onboarding remains deterministic.

## 6) Final checklist

- [ ] `ProfileId` variant added with stable `as_str` mapping.
- [ ] `AppProfile::new` arm added with explicit comm/input posture.
- [ ] App entrypoint(s) wired to new profile with composition-only logic.
- [ ] Startup behavior verified with typed errors + diagnostics intact.
- [ ] Docs updated (matrix/composition/startup, plus dependency rules if needed).
- [ ] Scenario docs/artifacts updated where profile changes behavior.
- [ ] Workspace checks pass (`cargo metadata`, `cargo check --workspace`).
