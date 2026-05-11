use messages::Target5Status;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target5Snapshot {
    pub last_status: Option<Target5Status>,
}

impl Target5Snapshot {
    pub fn record(&mut self, status: &Target5Status) {
        self.last_status = Some(status.clone());
    }
}
