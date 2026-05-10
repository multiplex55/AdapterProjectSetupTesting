pub mod manual_input;
pub mod replay;
pub mod scenario;
pub mod sim_data_source;

#[cfg(test)]
mod tests {
    use super::replay::{DeterministicReplay, ReplayEvent, ReplayParseError};
    use super::scenario::ReplayScenario;

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

    #[test]
    fn parses_sample_replay_fixture() {
        let raw = include_str!("../../../../scenarios/integration/target5_to_target10/sample-replay.json");
        let scenario = ReplayScenario::parse_json(raw).expect("sample fixture should parse");

        assert_eq!(scenario.events.len(), 2);
        assert!(matches!(scenario.events[0].event, ReplayEvent::Target5Status(_)));
        assert!(matches!(scenario.events[1].event, ReplayEvent::Target5Status(_)));
    }

    #[test]
    fn rejects_unknown_event_kind() {
        let raw = r#"{"events":[{"timestamp_ms":1000,"kind":"unknown_kind","payload":{}}]}"#;
        let err = ReplayScenario::parse_json(raw).expect_err("must fail for unknown kind");
        assert!(matches!(err, ReplayParseError::UnknownEventKind { .. }));
    }

    #[test]
    fn rejects_missing_required_fields() {
        let raw = r#"{"events":[{"timestamp_ms":1000,"kind":"target5_status","payload":{"device_id":5,"online":true}}]}"#;
        let err = ReplayScenario::parse_json(raw).expect_err("must fail when required fields are missing");
        assert!(matches!(err, ReplayParseError::MissingRequiredField { .. }));
    }

    #[test]
    fn rejects_invalid_event_order_and_malformed_fixture() {
        let raw = include_str!("../../../../scenarios/integration/target5_to_target10/malformed-replay.json");
        let err = ReplayScenario::parse_json(raw).expect_err("malformed fixture should fail");
        assert!(matches!(err, ReplayParseError::UnknownEventKind { .. }));

        let ordered_bad = r#"{"events":[{"timestamp_ms":2000,"kind":"target5_status","payload":{"device_id":5,"online":true,"sequence":1}},{"timestamp_ms":1000,"kind":"target5_status","payload":{"device_id":5,"online":false,"sequence":2}}]}"#;
        let err2 = ReplayScenario::parse_json(ordered_bad).expect_err("must fail on decreasing timestamp");
        assert!(matches!(err2, ReplayParseError::InvalidEventOrder { .. }));
    }
}
