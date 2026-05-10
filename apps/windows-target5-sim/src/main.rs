use std::{collections::BTreeMap, env};

use runtime::{
    startup::startup, CapabilityKind, ProfileId, ProviderCandidate, ProviderRegistry, StartupConfig,
};

fn parse_input_mode() -> &'static str {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--input" {
            return match args.next().as_deref() {
                Some("replay") => "replay",
                Some("manual") => "manual",
                Some("ethernet") => "ethernet",
                Some(other) => panic!("unsupported --input value: {other}"),
                None => panic!("--input requires one of: replay|manual|ethernet"),
            };
        }
    }
    "manual"
}

fn main() {
    let input_mode = parse_input_mode();

    let mut explicit_providers = BTreeMap::new();
    explicit_providers.insert(
        CapabilityKind::Compute,
        ProviderCandidate {
            path: "adapter://windows-sim-target5".to_string(),
            abi: 1,
        },
    );
    explicit_providers.insert(
        CapabilityKind::Transport,
        ProviderCandidate {
            path: format!("adapter://ethernet?input={input_mode}"),
            abi: 1,
        },
    );

    let config = StartupConfig {
        profile: ProfileId::WindowsTarget5Sim,
        plugin_search_paths: vec!["./plugins".to_string()],
        explicit_providers,
        required_capabilities: vec![CapabilityKind::Compute, CapabilityKind::Transport],
        required_abi: 1,
    };

    let registry = ProviderRegistry::with_defaults();
    let started = startup(config, &registry).expect("windows-target5-sim startup failed");

    println!(
        "profile={} input={} startup_ok selected={:?}",
        ProfileId::WindowsTarget5Sim,
        input_mode,
        started.diagnostics.selected_providers
    );
}
