use std::collections::BTreeMap;

use runtime::{
    startup::startup, CapabilityKind, ProfileId, ProviderCandidate, ProviderRegistry, StartupConfig,
};

fn main() {
    let mut explicit_providers = BTreeMap::new();
    explicit_providers.insert(
        CapabilityKind::Compute,
        ProviderCandidate {
            path: "adapter://target10".to_string(),
            abi: 1,
        },
    );
    explicit_providers.insert(
        CapabilityKind::Transport,
        ProviderCandidate {
            path: "adapter://ethernet+commtype1+commtype2".to_string(),
            abi: 1,
        },
    );

    let config = StartupConfig {
        profile: ProfileId::Target10Real,
        plugin_search_paths: vec!["./plugins".to_string()],
        explicit_providers,
        required_capabilities: vec![CapabilityKind::Compute, CapabilityKind::Transport],
        required_abi: 1,
    };

    let registry = ProviderRegistry::with_defaults();
    let started = startup(config, &registry).expect("target10-real startup failed");

    println!(
        "profile={} startup_ok selected={:?}",
        ProfileId::Target10Real,
        started.diagnostics.selected_providers
    );
}
