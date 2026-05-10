use std::collections::BTreeMap;

use crate::{
    app_profile::{AppProfile, ProfileId},
    diagnostics::{FallbackStatus, StartupDiagnostics},
    provider_registry::{CapabilityKind, ProviderCandidate, ProviderError, ProviderRegistry},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupError {
    RequiredCapabilityMissing {
        capability: CapabilityKind,
        decision_path: Vec<String>,
    },
    ProviderLoadFailed {
        capability: CapabilityKind,
        reason: String,
        decision_path: Vec<String>,
    },
    ProviderAbiMismatch {
        capability: CapabilityKind,
        reason: String,
        decision_path: Vec<String>,
    },
    ProviderSpecInvalid {
        capability: CapabilityKind,
        reason: String,
        decision_path: Vec<String>,
    },
    StartupValidationFailed {
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub struct StartupConfig {
    pub profile: ProfileId,
    pub plugin_search_paths: Vec<String>,
    pub explicit_providers: BTreeMap<CapabilityKind, ProviderCandidate>,
    pub required_capabilities: Vec<CapabilityKind>,
    pub required_abi: u32,
}

#[derive(Debug, Clone)]
pub struct StartupResult {
    pub diagnostics: StartupDiagnostics,
}

pub fn startup(
    config: StartupConfig,
    registry: &ProviderRegistry,
) -> Result<StartupResult, StartupError> {
    if config.plugin_search_paths.is_empty() {
        return Err(StartupError::StartupValidationFailed {
            reason: "plugin_search_paths cannot be empty".into(),
        });
    }

    let profile = AppProfile::new(config.profile);
    let mut diagnostics = StartupDiagnostics::from_profile(&profile);
    diagnostics.plugin_search_paths = config.plugin_search_paths;

    for cap in [
        CapabilityKind::Compute,
        CapabilityKind::Transport,
        CapabilityKind::Clock,
    ] {
        let required = config.required_capabilities.contains(&cap);
        let explicit = config.explicit_providers.get(&cap).cloned();
        match registry.resolve(cap, explicit, config.required_abi, required) {
            Ok(res) => {
                let status = if res.decision_path.iter().any(|a| a.ends_with("selected")) {
                    FallbackStatus::Succeeded
                } else {
                    FallbackStatus::Attempted
                };
                diagnostics
                    .selected_providers
                    .push((cap, format!("{} ({:?})", res.selected, res.selected_source)));
                diagnostics
                    .fallback_status
                    .push((cap, status, res.decision_path));
            }
            Err(ProviderError::AbiMismatch { reason }) => {
                diagnostics.fallback_status.push((
                    cap,
                    FallbackStatus::Failed,
                    vec![format!("abi-mismatch:{reason}")],
                ));
                return Err(StartupError::ProviderAbiMismatch {
                    capability: cap,
                    reason,
                    decision_path: vec!["explicit fallback blocked".into()],
                });
            }
            Err(ProviderError::InvalidSpec { reason }) => {
                diagnostics.fallback_status.push((
                    cap,
                    FallbackStatus::Failed,
                    vec![format!("invalid-spec:{reason}")],
                ));
                return Err(StartupError::ProviderSpecInvalid {
                    capability: cap,
                    reason,
                    decision_path: vec!["explicit fallback blocked".into()],
                });
            }
            Err(ProviderError::LoadFailed { reason }) => {
                diagnostics.fallback_status.push((
                    cap,
                    FallbackStatus::Failed,
                    vec![format!("load-failed:{reason}")],
                ));
                if required {
                    return Err(StartupError::RequiredCapabilityMissing {
                        capability: cap,
                        decision_path: vec![reason],
                    });
                }
                return Err(StartupError::ProviderLoadFailed {
                    capability: cap,
                    reason,
                    decision_path: vec!["fallback exhausted".into()],
                });
            }
        }
    }

    Ok(StartupResult { diagnostics })
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, fs, path::PathBuf};

    use super::*;
    use crate::provider_registry::ProviderSourceSpec;
    use plugins_loader::platform_library_extension;

    fn touch_plugin(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join("runtime-provider-tests");
        fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join(name);
        fs::write(&path, b"stub").expect("write");
        path
    }

    #[test]
    fn resolves_each_source_variant() {
        let ext = platform_library_extension();
        let plugin = touch_plugin(&format!("transport-valid.{ext}"));
        let registry = ProviderRegistry::with_defaults();
        let mut explicit = BTreeMap::new();
        explicit.insert(
            CapabilityKind::Compute,
            ProviderCandidate {
                path: "adapter://compute".into(),
                abi: 1,
                source: ProviderSourceSpec::Adapter,
            },
        );
        explicit.insert(
            CapabilityKind::Transport,
            ProviderCandidate {
                path: plugin.display().to_string(),
                abi: 1,
                source: ProviderSourceSpec::PluginPath(plugin),
            },
        );
        explicit.insert(
            CapabilityKind::Clock,
            ProviderCandidate {
                path: "builtin://optimized/Clock".into(),
                abi: 1,
                source: ProviderSourceSpec::BuiltInOptimized,
            },
        );
        let cfg = StartupConfig {
            profile: ProfileId::ReplayRunner,
            plugin_search_paths: vec!["./plugins".into()],
            explicit_providers: explicit,
            required_capabilities: vec![
                CapabilityKind::Compute,
                CapabilityKind::Transport,
                CapabilityKind::Clock,
            ],
            required_abi: 1,
        };
        let result = startup(cfg, &registry).expect("startup should resolve all variants");
        assert_eq!(result.diagnostics.selected_providers.len(), 3);
    }

    #[test]
    fn invalid_plugin_path_surfaces_typed_error() {
        let registry = ProviderRegistry::with_defaults();
        let mut explicit = BTreeMap::new();
        explicit.insert(
            CapabilityKind::Compute,
            ProviderCandidate {
                path: "bad".into(),
                abi: 1,
                source: ProviderSourceSpec::PluginPath(PathBuf::from("/tmp/noext")),
            },
        );
        let cfg = StartupConfig {
            profile: ProfileId::ReplayRunner,
            plugin_search_paths: vec!["./plugins".into()],
            explicit_providers: explicit,
            required_capabilities: vec![CapabilityKind::Compute],
            required_abi: 1,
        };
        let err = startup(cfg, &registry).expect_err("invalid plugin path should fail");
        assert!(matches!(err, StartupError::ProviderSpecInvalid { .. }));
    }
}
