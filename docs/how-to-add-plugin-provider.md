# How to Add a Plugin Provider

This guide describes the end-to-end integration path for a new provider plugin across:

- `crates/plugins/api`
- `crates/plugins/loader`
- `crates/runtime/src/provider_registry.rs`
- `crates/runtime/src/startup.rs`

It also documents stable boundary rules, lifecycle behavior, and explicit error surfacing requirements.

## 1) Define/verify ABI contract in `crates/plugins/api`

Plugin boundaries must be stable across DLL/SO compilation units.

### Stable boundary rules

1. **No Rust trait objects across DLL/SO boundary.**
   - Do not export `dyn Trait` pointers, vtables, or Rust-native object layouts.
2. **Use C ABI-compatible descriptors and function tables.**
   - Keep boundary types `#[repr(C)]` and composed of ABI-stable primitives/pointers.
3. **Version the ABI explicitly.**
   - Verify major/minor compatibility during load.
4. **Use fixed symbol names.**
   - Export descriptor + function table symbols matching the constants in `plugins_api`.

Current contract anchors:

- `PluginDescriptorV1` (`#[repr(C)]`) with ABI and capability metadata.
- `PluginFunctionTableV1` (`#[repr(C)]`) with function pointers.
- `ABI_BOUNDARY_RULE_NO_TRAIT_OBJECTS` as an explicit boundary invariant.

## 2) Implement loader behavior in `crates/plugins/loader`

`load_plugin` is responsible for deterministic, explicit contract validation.

### Required validation sequence

1. File presence / required-vs-optional semantics.
2. Extension check (`.dll` on Windows, `.so` elsewhere).
3. Resolve descriptor symbol.
4. Resolve function table symbol.
5. Validate ABI compatibility.
6. Validate expected capability identity.

### Error policy

- No silent fallback in loader.
- Return typed `LoadError` + `LoadErrorKind` with diagnostic context:
  - attempted path
  - missing symbol name (when relevant)
  - expected vs found ABI/capability

## 3) Register in runtime provider selection

Runtime selection is split between:

- `provider_registry.rs`: source ordering + resolution attempts
- `startup.rs`: startup orchestration, required capability policy, diagnostics

### Provider source order

Resolution order for each capability:

1. explicit provider
2. discovered plugin provider
3. built-in optimized provider
4. built-in reference provider

This sequence must remain observable through fallback diagnostics.

## 4) Registration lifecycle (end-to-end)

For each capability, host startup should process:

1. **discovery**
   - identify candidate plugin paths and map to capability intent.
2. **load/init**
   - load plugin library, read descriptor/function table, validate ABI/capability.
3. **capability check**
   - ensure descriptor capability matches requested runtime capability.
4. **runtime registration**
   - insert accepted provider candidate into runtime registry and diagnostics.

## 5) Explicit error surfacing requirements

All failure paths must be explicit and typed.

- **No silent fallback**:
  - optional behavior can fallback, but only with recorded attempts/status.
  - required behavior must fail startup on unresolved provider.
- **Typed errors with diagnostic context**:
  - runtime uses `StartupError` variants by failure class.
  - loader uses `LoadErrorKind` variants with contextual fields.

When adding new failure modes, add a typed enum variant and ensure diagnostics include the attempted stage/source.

## 6) Integration checklist

1. Add/extend capability constants and descriptor semantics in `plugins/api`.
2. Ensure plugin exports required C symbols and function table.
3. Add loader validation and typed errors in `plugins/loader`.
4. Wire discovered provider candidate into `ProviderRegistry`.
5. Verify startup required/optional behavior in `startup.rs`.
6. Add/update tests:
   - runtime provider registration/selection contract tests
   - malformed descriptor and missing symbol negative tests

## 7) Test expectations

Minimum reinforcement for new provider integration:

- Runtime contract tests validate provider source precedence and required capability failures.
- Runtime tests verify ABI mismatch and load failures surface typed `StartupError`.
- Loader negative tests verify malformed descriptors and missing symbols fail with explicit `LoadErrorKind`.

