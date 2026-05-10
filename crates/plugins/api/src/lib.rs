//! Plugin ABI contracts shared between host and plugin implementations.

pub const PLUGIN_ABI_VERSION_MAJOR: u16 = 1;
pub const PLUGIN_ABI_VERSION_MINOR: u16 = 0;
pub const PLUGIN_ABI_VERSION_PATCH: u16 = 0;

pub const PLUGIN_ABI_VERSION: u32 =
    ((PLUGIN_ABI_VERSION_MAJOR as u32) << 16) | (PLUGIN_ABI_VERSION_MINOR as u32);

pub const SYMBOL_PLUGIN_DESCRIPTOR_V1: &str = "adapter_plugin_descriptor_v1";
pub const SYMBOL_PLUGIN_FN_TABLE_V1: &str = "adapter_plugin_function_table_v1";

pub const CAPABILITY_COMPUTE: u32 = 1;
pub const CAPABILITY_TRANSPORT: u32 = 2;
pub const CAPABILITY_CLOCK: u32 = 3;

pub const STATUS_OK: i32 = 0;
pub const STATUS_ERR_NOT_SUPPORTED: i32 = 10;
pub const STATUS_ERR_INVALID_INPUT: i32 = 11;
pub const STATUS_ERR_INTERNAL: i32 = 12;

pub const STATUS_ERR_ABI_MISMATCH: i32 = 20;
pub const STATUS_ERR_MISSING_SYMBOL: i32 = 21;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbiVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluginDescriptorV1 {
    pub abi: AbiVersion,
    pub capability_id: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PluginFunctionTableV1 {
    /// Opaque handle passed back to the plugin on each call.
    pub context: *mut core::ffi::c_void,
    /// C-ABI callback for provider operations.
    pub invoke: extern "C" fn(
        context: *mut core::ffi::c_void,
        request_kind: u32,
        payload_ptr: *const u8,
        payload_len: usize,
        out_ptr: *mut u8,
        out_len: usize,
    ) -> i32,
}

/// ABI boundary rule: export only C-compatible symbols and function tables.
///
/// Rust trait objects are explicitly not ABI-stable and must never be exported
/// across dynamic library boundaries.
pub const ABI_BOUNDARY_RULE_NO_TRAIT_OBJECTS: &str = "no-rust-trait-object-export";
