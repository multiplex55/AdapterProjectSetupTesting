use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileId {
    Target5Real,
    Target10Real,
    WindowsTarget5Sim,
    WindowsTarget10Sim,
    ReplayRunner,
}

impl ProfileId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Target5Real => "target5-real",
            Self::Target10Real => "target10-real",
            Self::WindowsTarget5Sim => "windows-target5-sim",
            Self::WindowsTarget10Sim => "windows-target10-sim",
            Self::ReplayRunner => "replay-runner",
        }
    }
}

impl fmt::Display for ProfileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Live,
    Simulated,
    Replay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommType {
    Ethernet,
    Serial,
    Loopback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppProfile {
    pub id: ProfileId,
    pub enabled_comms: Vec<CommType>,
    pub disabled_comms: Vec<CommType>,
    pub input_mode: InputMode,
}

impl AppProfile {
    pub fn new(id: ProfileId) -> Self {
        match id {
            ProfileId::Target5Real | ProfileId::Target10Real => Self {
                id,
                enabled_comms: vec![CommType::Ethernet, CommType::Serial],
                disabled_comms: vec![CommType::Loopback],
                input_mode: InputMode::Live,
            },
            ProfileId::WindowsTarget5Sim | ProfileId::WindowsTarget10Sim => Self {
                id,
                enabled_comms: vec![CommType::Loopback],
                disabled_comms: vec![CommType::Ethernet, CommType::Serial],
                input_mode: InputMode::Simulated,
            },
            ProfileId::ReplayRunner => Self {
                id,
                enabled_comms: vec![CommType::Loopback],
                disabled_comms: vec![CommType::Ethernet, CommType::Serial],
                input_mode: InputMode::Replay,
            },
        }
    }
}
