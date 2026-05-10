use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CapabilityKind {
    Compute,
    Transport,
    Clock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCandidate {
    pub path: String,
    pub abi: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderError {
    LoadFailed,
    AbiMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderResolution {
    pub capability: CapabilityKind,
    pub selected: String,
    pub fallback_attempts: Vec<String>,
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
                },
            );
            reg.built_in_reference.insert(
                cap,
                ProviderCandidate {
                    path: format!("builtin://reference/{cap:?}"),
                    abi: 1,
                },
            );
        }
        reg
    }

    pub fn discovered_mut(&mut self) -> &mut BTreeMap<CapabilityKind, ProviderCandidate> {
        &mut self.discovered
    }

    pub fn resolve(
        &self,
        capability: CapabilityKind,
        explicit: Option<ProviderCandidate>,
        required_abi: u32,
        required: bool,
    ) -> Result<ProviderResolution, ProviderError> {
        let mut attempts = Vec::new();

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
                attempts.push(format!("{label}:missing"));
                continue;
            };
            if candidate.abi != required_abi {
                attempts.push(format!(
                    "{label}:abi-mismatch:{}!= {}",
                    candidate.abi, required_abi
                ));
                if required && label == "explicit" {
                    return Err(ProviderError::AbiMismatch);
                }
                continue;
            }
            if candidate.path.contains("load-fail") {
                attempts.push(format!("{label}:load-failed"));
                continue;
            }

            attempts.push(format!("{label}:selected"));
            return Ok(ProviderResolution {
                capability,
                selected: candidate.path.clone(),
                fallback_attempts: attempts,
            });
        }

        if required {
            if attempts.iter().any(|a| a.contains("abi-mismatch")) {
                return Err(ProviderError::AbiMismatch);
            }
            return Err(ProviderError::LoadFailed);
        }

        Ok(ProviderResolution {
            capability,
            selected: "none-optional".to_string(),
            fallback_attempts: attempts,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_provider_wins_selection_order() {
        let registry = ProviderRegistry::with_defaults();
        let explicit = ProviderCandidate { path: "plugin://explicit-compute".into(), abi: 1 };
        let res = registry
            .resolve(CapabilityKind::Compute, Some(explicit.clone()), 1, true)
            .expect("explicit provider should resolve");
        assert_eq!(res.selected, explicit.path);
        assert_eq!(res.fallback_attempts, vec!["explicit:selected"]);
    }

    #[test]
    fn discovered_provider_used_when_no_explicit() {
        let mut registry = ProviderRegistry::with_defaults();
        registry.discovered_mut().insert(
            CapabilityKind::Transport,
            ProviderCandidate { path: "plugin://discovered-transport".into(), abi: 1 },
        );
        let res = registry
            .resolve(CapabilityKind::Transport, None, 1, true)
            .expect("discovered provider should resolve");
        assert_eq!(res.selected, "plugin://discovered-transport");
        assert_eq!(res.fallback_attempts, vec!["explicit:missing", "discovered:selected"]);
    }
}
