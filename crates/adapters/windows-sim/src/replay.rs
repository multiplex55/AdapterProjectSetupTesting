use core::str::FromStr;

use messages::{Target10Command, Target5Status};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayEvent {
    Target5Status(Target5Status),
    Target10Command(Target10Command),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayParseError {
    InvalidFormat,
    InvalidSequence,
    InvalidPayload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimedReplayEvent {
    pub sequence: u64,
    pub event: ReplayEvent,
}

impl FromStr for TimedReplayEvent {
    type Err = ReplayParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // format: "<sequence>|<kind>|<json payload>"
        let mut parts = s.splitn(3, '|');
        let sequence = parts
            .next()
            .ok_or(ReplayParseError::InvalidFormat)?
            .parse::<u64>()
            .map_err(|_| ReplayParseError::InvalidSequence)?;
        let kind = parts.next().ok_or(ReplayParseError::InvalidFormat)?;
        let payload = parts.next().ok_or(ReplayParseError::InvalidFormat)?;

        let event = match kind {
            "target5_status" => ReplayEvent::Target5Status(
                serde_json::from_str(payload).map_err(|_| ReplayParseError::InvalidPayload)?,
            ),
            "target10_command" => ReplayEvent::Target10Command(
                serde_json::from_str(payload).map_err(|_| ReplayParseError::InvalidPayload)?,
            ),
            _ => return Err(ReplayParseError::InvalidFormat),
        };

        Ok(Self { sequence, event })
    }
}

#[derive(Debug, Clone)]
pub struct DeterministicReplay {
    events: Vec<TimedReplayEvent>,
    index: usize,
}

impl DeterministicReplay {
    pub fn from_lines<I>(lines: I) -> Result<Self, ReplayParseError>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut events = Vec::new();
        for line in lines {
            if line.as_ref().trim().is_empty() {
                continue;
            }
            events.push(TimedReplayEvent::from_str(line.as_ref())?);
        }
        events.sort_by_key(|e| e.sequence);
        Ok(Self { events, index: 0 })
    }
}

impl Iterator for DeterministicReplay {
    type Item = ReplayEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.events.get(self.index).map(|e| e.event.clone());
        if event.is_some() {
            self.index += 1;
        }
        event
    }
}
