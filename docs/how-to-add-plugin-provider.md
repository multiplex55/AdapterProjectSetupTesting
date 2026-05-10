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
   - Do not export `dyn Trait` pointers, Rust vtables, or Rust-native object layouts.
2. **Use C ABI-compatible descriptors and function tables.**
   - Keep boundary types `#[repr(C)]` and composed of ABI-stable primitives/pointers.
3. **Version the ABI explicitly.**
   - Verify major/minor compatibility during load.
4. **Use fixed symbol names.**
   - Export descriptor + function table symbols with exact required names.
5. **Prefer host-owned buffers across boundary.**
   - The host allocates and passes output buffers where possible.
6. **If plugin allocates memory, plugin must expose matching free/destroy function.**
   - Every plugin allocation crossing boundary must have a paired deallocator exported by the same plugin.

Current contract anchors:

- `PluginDescriptorV1` (`#[repr(C)]`) with ABI and capability metadata.
- `PluginFunctionTableV1` (`#[repr(C)]`) with function pointers.
- `ABI_BOUNDARY_RULE_NO_TRAIT_OBJECTS` as an explicit boundary invariant.

## 2) Concrete plugin crate layout (`cdylib`)

Example crate structure:

```text
crates/plugins/providers/target5-heartbeat/
  Cargo.toml
  src/
    lib.rs
```

### Example `Cargo.toml`

```toml
[package]
name = "target5-heartbeat-provider"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
plugins-api = { path = "../../api" }
```

### Example `src/lib.rs` skeleton

```rust
use plugins_api::{PluginDescriptorV1, PluginFunctionTableV1};

#[no_mangle]
pub static adapter_plugin_descriptor_v1: PluginDescriptorV1 = PluginDescriptorV1 {
    // fill with ABI version + capability identity
};

extern "C" fn provider_init(/* ... */) -> i32 {
    // initialize plugin state
    0
}

extern "C" fn provider_execute(/* ... */) -> i32 {
    // process request using host-owned buffers when possible
    0
}

extern "C" fn provider_shutdown(/* ... */) {
    // cleanup plugin state
}

#[no_mangle]
pub static adapter_plugin_function_table_v1: PluginFunctionTableV1 = PluginFunctionTableV1 {
    // assign init/execute/shutdown pointers
};

// If plugin returns plugin-allocated memory to host, export matching free:
#[no_mangle]
pub extern "C" fn provider_free_buffer(ptr: *mut u8, len: usize) {
    // reclaim allocation created by this plugin
}
```

Required symbol names exported by the plugin are exactly:

- `adapter_plugin_descriptor_v1`
- `adapter_plugin_function_table_v1`

## 3) Implement loader behavior in `crates/plugins/loader`

`load_plugin` is responsible for deterministic, explicit contract validation.

### Required validation sequence

1. File presence / required-vs-optional semantics.
2. Extension check (`.dll` on Windows, `.so` elsewhere).
3. Resolve descriptor symbol `adapter_plugin_descriptor_v1`.
4. Resolve function table symbol `adapter_plugin_function_table_v1`.
5. Validate ABI compatibility.
6. Validate expected capability identity.

### Error policy

- No silent fallback in loader.
- Return typed `LoadError` + `LoadErrorKind` with diagnostic context:
  - attempted path
  - missing symbol name (when relevant)
  - expected vs found ABI/capability

## 4) Register in runtime provider selection

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

## 5) Registration lifecycle (end-to-end)

For each capability, host startup should process:

1. **discovery**
   - identify candidate plugin paths and map to capability intent.
2. **load/init**
   - load plugin library, read descriptor/function table, validate ABI/capability.
3. **capability check**
   - ensure descriptor capability matches requested runtime capability.
4. **runtime registration**
   - insert accepted provider candidate into runtime registry and diagnostics.

## 6) Explicit error surfacing requirements

All failure paths must be explicit and typed.

- **No silent fallback**:
  - optional behavior can fallback, but only with recorded attempts/status.
  - required behavior must fail startup on unresolved provider.
- **Typed errors with diagnostic context**:
  - runtime uses `StartupError` variants by failure class.
  - loader uses `LoadErrorKind` variants with contextual fields.

When adding new failure modes, add a typed enum variant and ensure diagnostics include the attempted stage/source.

## 7) Integration checklist

1. Add/extend capability constants and descriptor semantics in `plugins/api`.
2. Implement plugin crate as `cdylib` and export required symbols.
3. Ensure symbol names are exact:
   - `adapter_plugin_descriptor_v1`
   - `adapter_plugin_function_table_v1`
4. Add loader validation and typed errors in `plugins/loader`.
5. Wire discovered provider candidate into `ProviderRegistry`.
6. Verify startup required/optional behavior in `startup.rs`.
7. Add/update tests:
   - runtime provider registration/selection contract tests
   - malformed descriptor and missing symbol negative tests

## 8) Test expectations

Minimum reinforcement for new provider integration:

- Runtime contract tests validate provider source precedence and required capability failures.
- Runtime tests verify ABI mismatch and load failures surface typed `StartupError`.
- Loader negative tests verify malformed descriptors and missing symbols fail with explicit `LoadErrorKind`.
