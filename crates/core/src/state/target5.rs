use messages::Target5Status;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Target5State {
    pub last_status: Option<Target5Status>,
    pub received_events: u64,
}

impl Target5State {
    pub fn record_status(&mut self, status: Target5Status) {
        self.last_status = Some(status);
        self.received_events += 1;
    }
}
