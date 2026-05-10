use adapter_windows_sim::scenario::ReplayScenario;
use core_crate::algorithms::target5_to_target10::map_target5_status_to_target10_command;
use messages::{Target10Command, Target5Status};
use std::process::Command;

const SAMPLE_REPLAY: &str =
    include_str!("../../../scenarios/integration/target5_to_target10/sample-replay.json");

fn commands_from_scenario(input: &str) -> Vec<Target10Command> {
    let scenario = ReplayScenario::parse_json(input).expect("valid canonical replay");
    scenario
        .events
        .into_iter()
        .filter_map(|timed| match timed.event {
            adapter_windows_sim::replay::ReplayEvent::Target5Status(status) => Some(status),
            adapter_windows_sim::replay::ReplayEvent::Target10Command(_) => None,
        })
        .map(|status| {
            map_target5_status_to_target10_command(&status).expect("valid mapped command")
        })
        .collect()
}

#[test]
fn maps_target5_statuses_via_core_production_mapping() {
    let statuses = [
        Target5Status {
            device_id: 5,
            online: true,
            sequence: 1,
        },
        Target5Status {
            device_id: 10,
            online: false,
            sequence: 2,
        },
    ];

    let commands: Vec<_> = statuses
        .iter()
        .map(|status| map_target5_status_to_target10_command(status).expect("valid mapped command"))
        .collect();

    assert_eq!(commands[0].action, "arm");
    assert_eq!(commands[0].command_id, 1005);
    assert_eq!(commands[1].action, "standby");
    assert_eq!(commands[1].command_id, 2010);
}

#[test]
fn replay_driven_output_uses_canonical_json_loader() {
    let output = commands_from_scenario(SAMPLE_REPLAY);

    assert_eq!(
        output,
        vec![
            Target10Command {
                command_id: 1005,
                action: "arm".to_string(),
                priority: 1
            },
            Target10Command {
                command_id: 2010,
                action: "standby".to_string(),
                priority: 5
            }
        ]
    );
}

#[test]
fn sim_apps_execute_replay_fixture_with_traceable_startup_logs() {
    let replay_path = "../../scenarios/integration/target5_to_target10/sample-replay.json";

    for app in ["windows-target5-sim", "windows-target10-sim"] {
        let output = Command::new("cargo")
            .args([
                "run",
                "-p",
                app,
                "--",
                "--input",
                "replay",
                "--replay",
                replay_path,
            ])
            .output()
            .expect("must start sim app");

        assert!(
            output.status.success(),
            "{} failed: {}",
            app,
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("startup profile="),
            "missing startup log for {app}: {stdout}"
        );
        assert!(
            stdout.contains("provider.Compute="),
            "missing compute provider for {app}: {stdout}"
        );
        assert!(
            stdout.contains("scenario.source="),
            "missing scenario source for {app}: {stdout}"
        );
        assert!(
            stdout.contains("command_out command_id=1005 action=arm priority=1"),
            "missing first command for {app}: {stdout}"
        );
        assert!(
            stdout.contains("command_out command_id=2010 action=standby priority=5"),
            "missing second command for {app}: {stdout}"
        );
        assert!(
            stdout.contains("diagnostics.commands_emitted=2"),
            "missing command diagnostics for {app}: {stdout}"
        );
    }
}
