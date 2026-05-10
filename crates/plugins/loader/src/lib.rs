use std::path::{Path, PathBuf};

use plugins_api::{
    AbiVersion, PluginDescriptorV1, PluginFunctionTableV1, CAPABILITY_CLOCK, CAPABILITY_COMPUTE,
    CAPABILITY_TRANSPORT, PLUGIN_ABI_VERSION_MAJOR, PLUGIN_ABI_VERSION_MINOR,
    STATUS_ERR_ABI_MISMATCH, STATUS_ERR_INTERNAL, STATUS_ERR_INVALID_INPUT,
    STATUS_ERR_NOT_SUPPORTED, STATUS_OK, SYMBOL_PLUGIN_DESCRIPTOR_V1, SYMBOL_PLUGIN_FN_TABLE_V1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    Compute,
    Transport,
    Clock,
}

impl Capability {
    pub fn as_abi_id(self) -> u32 {
        match self {
            Self::Compute => CAPABILITY_COMPUTE,
            Self::Transport => CAPABILITY_TRANSPORT,
            Self::Clock => CAPABILITY_CLOCK,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadErrorKind {
    NotFound,
    UnsupportedExtension,
    MissingRequiredSymbol {
        symbol: &'static str,
    },
    AbiMismatch {
        expected: AbiVersion,
        found: AbiVersion,
    },
    CapabilityMismatch {
        expected: Capability,
        found_capability_id: u32,
    },
    InvokeFailed {
        code: i32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadError {
    pub plugin_path_attempted: PathBuf,
    pub reason: LoadErrorKind,
}

#[derive(Debug, Clone)]
pub struct LoadedPlugin {
    pub path: PathBuf,
    pub descriptor: PluginDescriptorV1,
    pub function_table: PluginFunctionTableV1,
}

#[derive(Debug, Clone)]
pub struct PluginLoadRequest {
    pub path: PathBuf,
    pub expected_capability: Capability,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub enum LoadOutcome {
    Loaded(LoadedPlugin),
    OptionalMissing { attempted_path: PathBuf },
}

pub trait PluginLibrary {
    fn descriptor(&self) -> Option<PluginDescriptorV1>;
    fn function_table(&self) -> Option<PluginFunctionTableV1>;
}

pub fn platform_library_extension() -> &'static str {
    if cfg!(windows) {
        "dll"
    } else {
        "so"
    }
}

pub fn load_plugin(request: &PluginLoadRequest) -> Result<LoadOutcome, LoadError> {
    if !request.path.exists() {
        if request.required {
            return Err(LoadError {
                plugin_path_attempted: request.path.clone(),
                reason: LoadErrorKind::NotFound,
            });
        }
        return Ok(LoadOutcome::OptionalMissing {
            attempted_path: request.path.clone(),
        });
    }

    let expected_ext = platform_library_extension();
    if request.path.extension().and_then(|e| e.to_str()) != Some(expected_ext) {
        return Err(LoadError {
            plugin_path_attempted: request.path.clone(),
            reason: LoadErrorKind::UnsupportedExtension,
        });
    }

    let lib = simulated_open(&request.path);
    let descriptor = lib.descriptor().ok_or_else(|| LoadError {
        plugin_path_attempted: request.path.clone(),
        reason: LoadErrorKind::MissingRequiredSymbol {
            symbol: SYMBOL_PLUGIN_DESCRIPTOR_V1,
        },
    })?;

    let table = lib.function_table().ok_or_else(|| LoadError {
        plugin_path_attempted: request.path.clone(),
        reason: LoadErrorKind::MissingRequiredSymbol {
            symbol: SYMBOL_PLUGIN_FN_TABLE_V1,
        },
    })?;

    validate_abi(&request.path, descriptor.abi)?;
    validate_capability(
        &request.path,
        request.expected_capability,
        descriptor.capability_id,
    )?;

    Ok(LoadOutcome::Loaded(LoadedPlugin {
        path: request.path.clone(),
        descriptor,
        function_table: table,
    }))
}

fn validate_abi(path: &Path, found: AbiVersion) -> Result<(), LoadError> {
    let expected = AbiVersion {
        major: PLUGIN_ABI_VERSION_MAJOR,
        minor: PLUGIN_ABI_VERSION_MINOR,
        patch: 0,
    };
    if found.major == expected.major && found.minor == expected.minor {
        return Ok(());
    }
    Err(LoadError {
        plugin_path_attempted: path.to_path_buf(),
        reason: LoadErrorKind::AbiMismatch { expected, found },
    })
}

fn validate_capability(path: &Path, expected: Capability, found_id: u32) -> Result<(), LoadError> {
    if expected.as_abi_id() == found_id {
        return Ok(());
    }
    Err(LoadError {
        plugin_path_attempted: path.to_path_buf(),
        reason: LoadErrorKind::CapabilityMismatch {
            expected,
            found_capability_id: found_id,
        },
    })
}

#[derive(Debug, Clone, Copy)]
pub struct AbiProviderAdapter {
    function_table: PluginFunctionTableV1,
}

impl AbiProviderAdapter {
    pub fn new(function_table: PluginFunctionTableV1) -> Self {
        Self { function_table }
    }

    pub fn invoke(
        &self,
        request_kind: u32,
        payload: &[u8],
        out: &mut [u8],
    ) -> Result<(), LoadErrorKind> {
        let code = (self.function_table.invoke)(
            self.function_table.context,
            request_kind,
            payload.as_ptr(),
            payload.len(),
            out.as_mut_ptr(),
            out.len(),
        );
        match code {
            STATUS_OK => Ok(()),
            STATUS_ERR_INVALID_INPUT => Err(LoadErrorKind::InvokeFailed { code }),
            STATUS_ERR_NOT_SUPPORTED => Err(LoadErrorKind::InvokeFailed { code }),
            STATUS_ERR_INTERNAL => Err(LoadErrorKind::InvokeFailed { code }),
            STATUS_ERR_ABI_MISMATCH => Err(LoadErrorKind::InvokeFailed { code }),
            _ => Err(LoadErrorKind::InvokeFailed { code }),
        }
    }
}

#[derive(Debug, Clone)]
struct SimulatedLibrary {
    descriptor: Option<PluginDescriptorV1>,
    table: Option<PluginFunctionTableV1>,
}

impl PluginLibrary for SimulatedLibrary {
    fn descriptor(&self) -> Option<PluginDescriptorV1> {
        self.descriptor
    }
    fn function_table(&self) -> Option<PluginFunctionTableV1> {
        self.table
    }
}

fn simulated_open(path: &Path) -> SimulatedLibrary {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    let bad_abi = name.contains("abi2");
    let missing_descriptor = name.contains("missing-desc");
    let missing_table = name.contains("missing-table");
    let capability = if name.contains("transport") {
        CAPABILITY_TRANSPORT
    } else if name.contains("clock") {
        CAPABILITY_CLOCK
    } else {
        CAPABILITY_COMPUTE
    };

    SimulatedLibrary {
        descriptor: (!missing_descriptor).then_some(PluginDescriptorV1 {
            abi: AbiVersion {
                major: if bad_abi { 2 } else { PLUGIN_ABI_VERSION_MAJOR },
                minor: PLUGIN_ABI_VERSION_MINOR,
                patch: 0,
            },
            capability_id: capability,
            flags: 0,
        }),
        table: (!missing_table).then_some(PluginFunctionTableV1 {
            context: core::ptr::null_mut(),
            invoke: noop_invoke,
        }),
    }
}

extern "C" fn noop_invoke(
    _context: *mut core::ffi::c_void,
    _request_kind: u32,
    _payload_ptr: *const u8,
    _payload_len: usize,
    _out_ptr: *mut u8,
    _out_len: usize,
) -> i32 {
    STATUS_OK
}
