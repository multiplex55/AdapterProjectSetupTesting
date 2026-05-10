use crate::{health::HealthReport, startup::StartupResult};

#[derive(Debug)]
pub struct AppHost {
    pub startup: StartupResult,
    pub health: HealthReport,
}

impl AppHost {
    pub fn new(startup: StartupResult, health: HealthReport) -> Self {
        Self { startup, health }
    }
}
