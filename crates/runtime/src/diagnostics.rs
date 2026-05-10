use crate::{app_profile::CommType, provider_registry::CapabilityKind, AppProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackStatus {
    Attempted,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupDiagnostics {
    pub selected_profile: String,
    pub selected_adapters: Vec<String>,
    pub selected_providers: Vec<(CapabilityKind, String)>,
    pub plugin_search_paths: Vec<String>,
    pub fallback_status: Vec<(CapabilityKind, FallbackStatus, Vec<String>)>,
    pub enabled_comms: Vec<CommType>,
    pub disabled_comms: Vec<CommType>,
    pub input_mode: String,
}

impl StartupDiagnostics {
    pub fn from_profile(profile: &AppProfile) -> Self {
        Self {
            selected_profile: profile.id.to_string(),
            selected_adapters: Vec::new(),
            selected_providers: Vec::new(),
            plugin_search_paths: Vec::new(),
            fallback_status: Vec::new(),
            enabled_comms: profile.enabled_comms.clone(),
            disabled_comms: profile.disabled_comms.clone(),
            input_mode: format!("{:?}", profile.input_mode),
        }
    }
}
