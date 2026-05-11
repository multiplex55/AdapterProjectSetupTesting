use messages::{Target10Command, Target5Status};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Target10State {
    pub last_seen_status: Option<Target5Status>,
    pub last_command: Option<Target10Command>,
    pub applied_events: u64,
}

impl Target10State {
    pub fn apply(&mut self, status: Target5Status, command: Target10Command) {
        self.last_seen_status = Some(status);
        self.last_command = Some(command);
        self.applied_events += 1;
    }
}
