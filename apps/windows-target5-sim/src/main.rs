use std::{collections::BTreeMap, env, fs};

use adapter_windows_sim::replay::ReplayEvent;
use adapter_windows_sim::scenario::ReplayScenario;
use core_crate::algorithms::target5_to_target10::map_target5_status_to_target10_command;
use runtime::{
    startup::startup, CapabilityKind, ProfileId, ProviderCandidate, ProviderRegistry, StartupConfig,
};

fn parse_args() -> (&'static str, Option<String>) {
    let mut input_mode = "manual";
    let mut replay_path = None;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                input_mode = match args.next().as_deref() {
                    Some("replay") => "replay",
                    Some("manual") => "manual",
                    Some("ethernet") => "ethernet",
                    Some(other) => panic!("unsupported --input value: {other}"),
                    None => panic!("--input requires one of: replay|manual|ethernet"),
                };
            }
            "--replay" => replay_path = args.next(),
            _ => {}
        }
    }
    (input_mode, replay_path)
}

fn main() {
    let (input_mode, replay_path) = parse_args();
    let scenario_source = replay_path.unwrap_or_else(|| {
        "scenarios/integration/target5_to_target10/sample-replay.json".to_string()
    });

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
        "startup profile={} input={} scenario.source={}",
        ProfileId::WindowsTarget5Sim,
        input_mode,
        scenario_source
    );
    for (cap, provider) in &started.diagnostics.selected_providers {
        println!("provider.{:?}={}", cap, provider);
    }

    if input_mode == "replay" {
        let raw = fs::read_to_string(&scenario_source).expect("failed to read replay file");
        let scenario =
            ReplayScenario::parse_json(&raw).expect("failed to parse canonical replay JSON");
        let mut commands_emitted = 0u64;
        for event in scenario.events {
            if let ReplayEvent::Target5Status(status) = event.event {
                let cmd =
                    map_target5_status_to_target10_command(&status).expect("mapping must succeed");
                println!(
                    "command_out command_id={} action={} priority={}",
                    cmd.command_id, cmd.action, cmd.priority
                );
                commands_emitted += 1;
            }
        }
        println!("diagnostics.commands_emitted={commands_emitted}");
    }
}
