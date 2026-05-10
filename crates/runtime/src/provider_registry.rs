use std::{collections::BTreeMap, path::PathBuf};

use plugins_loader::{
    load_plugin, Capability as PluginCapability, LoadOutcome, LoadStrategy as PluginLoadStrategy,
    PluginLoadRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CapabilityKind {
    Compute,
    Transport,
    Clock,
}

impl CapabilityKind {
    fn as_plugin_capability(self) -> PluginCapability {
        match self {
            Self::Compute => PluginCapability::Compute,
            Self::Transport => PluginCapability::Transport,
            Self::Clock => PluginCapability::Clock,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderSourceSpec {
    Adapter,
    PluginPath(PathBuf),
    BuiltInOptimized,
    BuiltInReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCandidate {
    pub path: String,
    pub abi: u32,
    pub source: ProviderSourceSpec,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderError {
    LoadFailed { reason: String },
    AbiMismatch { reason: String },
    InvalidSpec { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderResolution {
    pub capability: CapabilityKind,
    pub selected: String,
    pub selected_source: ProviderSourceSpec,
    pub decision_path: Vec<String>,
}

#[derive(Debug, Default)]
pub struct ProviderRegistry {
    discovered: BTreeMap<CapabilityKind, ProviderCandidate>,
    built_in_optimized: BTreeMap<CapabilityKind, ProviderCandidate>,
    built_in_reference: BTreeMap<CapabilityKind, ProviderCandidate>,
}

impl ProviderRegistry {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_defaults() -> Self {
        let mut reg = Self::default();
        for cap in [
            CapabilityKind::Compute,
            CapabilityKind::Transport,
            CapabilityKind::Clock,
        ] {
            reg.built_in_optimized.insert(
                cap,
                ProviderCandidate {
                    path: format!("builtin://optimized/{cap:?}"),
                    abi: 1,
                    source: ProviderSourceSpec::BuiltInOptimized,
                },
            );
            reg.built_in_reference.insert(
                cap,
                ProviderCandidate {
                    path: format!("builtin://reference/{cap:?}"),
                    abi: 1,
                    source: ProviderSourceSpec::BuiltInReference,
                },
            );
        }
        reg
    }

    pub fn discovered_mut(&mut self) -> &mut BTreeMap<CapabilityKind, ProviderCandidate> {
        &mut self.discovered
    }

    fn validate_and_load_candidate(
        capability: CapabilityKind,
        candidate: &ProviderCandidate,
        required_abi: u32,
        required: bool,
    ) -> Result<String, ProviderError> {
        if candidate.abi != required_abi {
            return Err(ProviderError::AbiMismatch {
                reason: format!(
                    "candidate abi {} != required {}",
                    candidate.abi, required_abi
                ),
            });
        }
        match &candidate.source {
            ProviderSourceSpec::PluginPath(path) => {
                if path.extension().is_none() {
                    return Err(ProviderError::InvalidSpec {
                        reason: format!("plugin path {} has no extension", path.display()),
                    });
                }
                let req = PluginLoadRequest {
                    path: path.clone(),
                    expected_capability: capability.as_plugin_capability(),
                    required,
                    strategy: PluginLoadStrategy::Dynamic,
                };
                match load_plugin(&req) {
                    Ok(LoadOutcome::Loaded(loaded)) => Ok(loaded.path.display().to_string()),
                    Ok(LoadOutcome::OptionalMissing { attempted_path }) => {
                        Ok(format!("optional-missing:{}", attempted_path.display()))
                    }
                    Err(err) => Err(ProviderError::LoadFailed {
                        reason: format!(
                            "plugin load failed at {}: {:?}",
                            err.plugin_path_attempted.display(),
                            err.reason
                        ),
                    }),
                }
            }
            _ => {
                if candidate.path.contains("load-fail") {
                    return Err(ProviderError::LoadFailed {
                        reason: format!("provider {} failed during simulated load", candidate.path),
                    });
                }
                Ok(candidate.path.clone())
            }
        }
    }

    pub fn resolve(
        &self,
        capability: CapabilityKind,
        explicit: Option<ProviderCandidate>,
        required_abi: u32,
        required: bool,
    ) -> Result<ProviderResolution, ProviderError> {
        let mut decisions = Vec::new();
        for (label, candidate_opt) in [
            ("explicit", explicit.as_ref()),
            ("discovered", self.discovered.get(&capability)),
            (
                "builtin-optimized",
                self.built_in_optimized.get(&capability),
            ),
            (
                "builtin-reference",
                self.built_in_reference.get(&capability),
            ),
        ] {
            let Some(candidate) = candidate_opt else {
                decisions.push(format!("{label}:missing"));
                continue;
            };
            decisions.push(format!("{label}:source={:?}", candidate.source));
            match Self::validate_and_load_candidate(
                capability,
                candidate,
                required_abi,
                required && label == "explicit",
            ) {
                Ok(selected) => {
                    decisions.push(format!("{label}:selected"));
                    return Ok(ProviderResolution {
                        capability,
                        selected,
                        selected_source: candidate.source.clone(),
                        decision_path: decisions,
                    });
                }
                Err(err) => {
                    decisions.push(format!("{label}:rejected:{err:?}"));
                    if required && label == "explicit" {
                        return Err(err);
                    }
                }
            }
        }
        if required {
            return Err(ProviderError::LoadFailed {
                reason: format!("required capability {:?} unresolved", capability),
            });
        }
        Ok(ProviderResolution {
            capability,
            selected: "none-optional".into(),
            selected_source: ProviderSourceSpec::BuiltInReference,
            decision_path: decisions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_provider_wins_selection_order() {
        let registry = ProviderRegistry::with_defaults();
        let explicit = ProviderCandidate {
            path: "plugin://explicit-compute".into(),
            abi: 1,
            source: ProviderSourceSpec::Adapter,
        };
        let res = registry
            .resolve(CapabilityKind::Compute, Some(explicit.clone()), 1, true)
            .expect("explicit provider should resolve");
        assert_eq!(res.selected, explicit.path);
        assert_eq!(
            res.decision_path.last(),
            Some(&"explicit:selected".to_string())
        );
    }

    #[test]
    fn plugin_path_without_extension_is_invalid_spec() {
        let registry = ProviderRegistry::with_defaults();
        let explicit = ProviderCandidate {
            path: "plugin://bad".into(),
            abi: 1,
            source: ProviderSourceSpec::PluginPath(PathBuf::from("/tmp/plugin-no-ext")),
        };
        let err = registry
            .resolve(CapabilityKind::Compute, Some(explicit), 1, true)
            .expect_err("invalid spec expected");
        assert!(matches!(err, ProviderError::InvalidSpec { .. }));
    }
}
