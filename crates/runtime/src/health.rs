#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthState {
    Ready,
    Degraded,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthReport {
    pub state: HealthState,
    pub reason: String,
}
