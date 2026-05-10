use adapter_windows_sim::replay::{DeterministicReplay, ReplayEvent};
use core_crate::algorithms::target5_to_target10::map_target5_status_to_target10_command;
use messages::Target10Command;

#[test]
fn windows_target5_to_target10_loopback_message_exchange() {
    let statuses = [
        messages::Target5Status {
            device_id: 5,
            online: true,
            sequence: 1,
        },
        messages::Target5Status {
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
fn replay_driven_output_is_deterministic() {
    let lines = vec![
        r#"30|target5_status|{"device_id":10,"online":false,"sequence":2}"#,
        r#"10|target5_status|{"device_id":5,"online":true,"sequence":1}"#,
    ];

    let replay = DeterministicReplay::from_lines(lines).expect("valid replay");
    let output: Vec<_> = replay
        .filter_map(|event| match event {
            ReplayEvent::Target5Status(status) => {
                Some(map_target5_status_to_target10_command(&status).expect("valid mapped command"))
            }
            ReplayEvent::Target10Command(_) => None,
        })
        .collect();

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
