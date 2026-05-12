use std::collections::BTreeMap;

use runtime::{
    startup::startup, CapabilityKind, ProfileId, ProviderCandidate, ProviderRegistry,
    ProviderSourceSpec, StartupConfig,
};

fn main() {
    // Composition-only: select profile, parse minimal startup config, construct adapters/providers,
    // initialize runtime state, and call the runtime startup host function.
    let mut explicit_providers = BTreeMap::new();
    explicit_providers.insert(
        CapabilityKind::Compute,
        ProviderCandidate {
            path: "adapter://target5".to_string(),
            abi: 1,
            source: ProviderSourceSpec::Adapter,
        },
    );
    explicit_providers.insert(
        CapabilityKind::Transport,
        ProviderCandidate {
            path: "adapter://ethernet".to_string(),
            abi: 1,
            source: ProviderSourceSpec::Adapter,
        },
    );

    let config = StartupConfig {
        profile: ProfileId::Target5Real,
        plugin_search_paths: vec!["./plugins".to_string()],
        explicit_providers,
        required_capabilities: vec![CapabilityKind::Compute, CapabilityKind::Transport],
        required_abi: 1,
    };

    let registry = ProviderRegistry::with_defaults();
    let started = startup(config, &registry).expect("target5-real startup failed");

    println!(
        "profile={} startup_ok selected={:?}",
        ProfileId::Target5Real,
        started.diagnostics.selected_providers
    );
}
