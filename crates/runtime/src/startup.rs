use std::collections::BTreeMap;

use crate::{
    app_profile::{AppProfile, ProfileId},
    diagnostics::{FallbackStatus, StartupDiagnostics},
    provider_registry::{CapabilityKind, ProviderCandidate, ProviderError, ProviderRegistry},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupError {
    RequiredCapabilityMissing { capability: CapabilityKind },
    ProviderLoadFailed { capability: CapabilityKind },
    ProviderAbiMismatch { capability: CapabilityKind },
    StartupValidationFailed { reason: String },
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
                let status = if res.fallback_attempts.iter().any(|a| a.contains("selected")) {
                    FallbackStatus::Succeeded
                } else {
                    FallbackStatus::Attempted
                };
                diagnostics.selected_providers.push((cap, res.selected));
                diagnostics
                    .fallback_status
                    .push((cap, status, res.fallback_attempts));
            }
            Err(ProviderError::AbiMismatch) => {
                diagnostics.fallback_status.push((
                    cap,
                    FallbackStatus::Failed,
                    vec!["abi-mismatch".into()],
                ));
                return Err(StartupError::ProviderAbiMismatch { capability: cap });
            }
            Err(ProviderError::LoadFailed) => {
                diagnostics.fallback_status.push((
                    cap,
                    FallbackStatus::Failed,
                    vec!["load-failed-or-missing".into()],
                ));
                if required {
                    return Err(StartupError::RequiredCapabilityMissing { capability: cap });
                }
                return Err(StartupError::ProviderLoadFailed { capability: cap });
            }
        }
    }

    Ok(StartupResult { diagnostics })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_provider_missing_falls_back_successfully() {
        let registry = ProviderRegistry::with_defaults();
        let cfg = StartupConfig {
            profile: ProfileId::ReplayRunner,
            plugin_search_paths: vec!["./plugins".into()],
            explicit_providers: BTreeMap::new(),
            required_capabilities: vec![],
            required_abi: 1,
        };
        let result = startup(cfg, &registry).expect("startup should succeed");
        assert!(result
            .diagnostics
            .fallback_status
            .iter()
            .all(|(_, status, _)| *status == FallbackStatus::Succeeded));
    }

    #[test]
    fn required_provider_missing_fails_startup() {
        let registry = ProviderRegistry::empty();
        let cfg = StartupConfig {
            profile: ProfileId::Target5Real,
            plugin_search_paths: vec!["./plugins".into()],
            explicit_providers: BTreeMap::new(),
            required_capabilities: vec![CapabilityKind::Compute],
            required_abi: 1,
        };
        let err = startup(cfg, &registry).expect_err("required capability should fail");
        assert_eq!(
            err,
            StartupError::RequiredCapabilityMissing {
                capability: CapabilityKind::Compute
            }
        );
    }

    #[test]
    fn abi_mismatch_path_has_clear_error() {
        let registry = ProviderRegistry::with_defaults();
        let mut explicit = BTreeMap::new();
        explicit.insert(
            CapabilityKind::Transport,
            ProviderCandidate {
                path: "plugin://transport".into(),
                abi: 2,
            },
        );
        let cfg = StartupConfig {
            profile: ProfileId::WindowsTarget10Sim,
            plugin_search_paths: vec!["./plugins".into()],
            explicit_providers: explicit,
            required_capabilities: vec![CapabilityKind::Transport],
            required_abi: 1,
        };
        let err = startup(cfg, &registry).expect_err("abi mismatch should fail");
        assert_eq!(
            err,
            StartupError::ProviderAbiMismatch {
                capability: CapabilityKind::Transport
            }
        );
    }
}
