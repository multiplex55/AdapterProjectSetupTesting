pub mod manual_input;
pub mod replay;
pub mod sim_data_source;

#[cfg(test)]
mod tests {
    use super::replay::{DeterministicReplay, ReplayEvent};

    #[test]
    fn replay_orders_by_sequence_deterministically() {
        let lines = vec![
            r#"30|target10_command|{"command_id":11,"action":"arm","priority":2}"#,
            r#"10|target5_status|{"device_id":5,"online":true,"sequence":1}"#,
            r#"20|target5_status|{"device_id":10,"online":true,"sequence":2}"#,
        ];

        let replay = DeterministicReplay::from_lines(lines).expect("valid replay input");
        let events: Vec<_> = replay.collect();

        assert!(matches!(events[0], ReplayEvent::Target5Status(_)));
        assert!(matches!(events[1], ReplayEvent::Target5Status(_)));
        assert!(matches!(events[2], ReplayEvent::Target10Command(_)));
    }
}
