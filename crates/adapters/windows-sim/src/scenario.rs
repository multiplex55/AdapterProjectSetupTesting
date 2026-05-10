use messages::{Target10Command, Target5Status};
use serde::Deserialize;

use crate::replay::{ReplayEvent, ReplayParseError, TimedReplayEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayScenario {
    pub events: Vec<TimedReplayEvent>,
}

impl ReplayScenario {
    pub fn parse_json(input: &str) -> Result<Self, ReplayParseError> {
        let doc: ScenarioDocument =
            serde_json::from_str(input).map_err(|source| ReplayParseError::InvalidJson {
                message: source.to_string(),
            })?;

        let mut events = Vec::with_capacity(doc.events.len());
        let mut last_timestamp = None;

        for event in doc.events {
            if let Some(last) = last_timestamp {
                if event.timestamp_ms < last {
                    return Err(ReplayParseError::InvalidEventOrder {
                        previous_timestamp_ms: last,
                        current_timestamp_ms: event.timestamp_ms,
                    });
                }
            }

            let replay_event = match event.kind.as_str() {
                "target5_status" => {
                    let parsed = serde_json::from_value::<Target5Status>(event.payload).map_err(|source| {
                        ReplayParseError::MissingRequiredField {
                            kind: "target5_status".to_string(),
                            message: source.to_string(),
                        }
                    })?;
                    ReplayEvent::Target5Status(parsed)
                }
                "target10_command" => {
                    let parsed = serde_json::from_value::<Target10Command>(event.payload).map_err(|source| {
                        ReplayParseError::MissingRequiredField {
                            kind: "target10_command".to_string(),
                            message: source.to_string(),
                        }
                    })?;
                    ReplayEvent::Target10Command(parsed)
                }
                other => {
                    return Err(ReplayParseError::UnknownEventKind {
                        kind: other.to_string(),
                    })
                }
            };

            last_timestamp = Some(event.timestamp_ms);
            events.push(TimedReplayEvent {
                sequence: event.timestamp_ms,
                event: replay_event,
            });
        }

        Ok(Self { events })
    }
}

#[derive(Debug, Deserialize)]
struct ScenarioDocument {
    events: Vec<ScenarioEvent>,
}

#[derive(Debug, Deserialize)]
struct ScenarioEvent {
    timestamp_ms: u64,
    kind: String,
    payload: serde_json::Value,
}
