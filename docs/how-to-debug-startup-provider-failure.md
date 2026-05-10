# How to Debug Startup Provider Failure

Scenario-oriented troubleshooting for startup/provider failures.

## Primary source files

- Startup orchestration and typed errors: [`crates/runtime/src/startup.rs`](../crates/runtime/src/startup.rs)
- Resolution ordering and candidate validation: [`crates/runtime/src/provider_registry.rs`](../crates/runtime/src/provider_registry.rs)
- Plugin load details (extension/ABI/symbol checks): [`crates/plugins/loader/src/lib.rs`](../crates/plugins/loader/src/lib.rs)

## Decision path first

Start with diagnostics decision paths (if available):

1. Identify failing capability (`Compute`, `Transport`, `Clock`).
2. Read ordered decisions to see where rejection happened.
3. Match rejection reason to one of the typed startup errors.

## Error-keyed troubleshooting

### `StartupValidationFailed`

Likely cause:

- `plugin_search_paths` is empty.

Checklist:

- Verify app built `StartupConfig.plugin_search_paths` with at least one path.
- Confirm path values are intentional for the current host profile.

### `ProviderSpecInvalid`

Likely cause:

- invalid plugin source specification (for example, missing file extension).

Checklist:

- If using plugin path source, ensure path includes extension.
- Ensure extension matches platform expectation.
- Re-check explicit provider candidate source type and path formatting.

### `ProviderAbiMismatch`

Likely cause:

- candidate ABI does not match runtime required ABI.

Checklist:

- Compare candidate `abi` and runtime `required_abi` values.
- For dynamic plugin candidates, validate plugin ABI version compatibility.
- Rebuild plugin/provider against expected ABI contract version.

### `ProviderLoadFailed`

Likely causes:

- loader cannot open plugin;
- required symbols/capability mismatch;
- dynamic backend unavailable in current build;
- fallback exhausted for non-required path.

Checklist:

- Confirm plugin file exists at attempted path.
- Confirm required symbols are exported.
- Confirm expected capability matches plugin descriptor capability id.
- Confirm platform and build include expected dynamic loading backend.

### `RequiredCapabilityMissing`

Likely cause:

- no candidate selected for a required capability after resolution ordering.

Checklist:

- Inspect explicit candidate correctness.
- Inspect discovered candidate population.
- Verify built-in providers exist for that capability in registry defaults.
- Re-check source ordering implications: explicit → discovered → builtin-optimized → builtin-reference.

## Scenario checklists

### Extension mismatch (`.dll` / `.so`)

- Determine host platform expected extension from loader behavior.
- Verify plugin path extension exactly matches expected platform extension.
- Ensure filename is not extensionless and not cross-platform artifact.

### ABI mismatch

- Validate runtime required ABI and candidate ABI fields.
- For plugin binaries, inspect descriptor ABI major/minor compatibility.

### Invalid plugin path spec

- Ensure plugin source is `PluginPath` with a real path-like value.
- Ensure extension is present and path points to intended file.

### Missing required capability

- Confirm capability is intentionally marked required.
- Confirm at least one viable source candidate can satisfy requirement.
- If explicit candidate is strict/required, decide whether fallback should be permitted in future behavior.

## Related docs

- [Runtime startup flow](./runtime-startup-flow.md)
- [Provider requirement model](./provider-requirement-model.md)
- [Application profile matrix](./application-profile-matrix.md)
- [App composition guide](./app-composition-guide.md)
