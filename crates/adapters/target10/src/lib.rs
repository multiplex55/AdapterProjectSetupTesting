use messages::Target10Command;
use ports::{MessagePublisher, TransportError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target10InfraError {
    HardwarePathUnimplemented,
}

pub struct Target10Adapter;
impl Target10Adapter {
    pub fn new() -> Self {
        Self
    }
    pub fn send_command(&self, _command: &Target10Command) -> Result<(), Target10InfraError> {
        Err(Target10InfraError::HardwarePathUnimplemented)
    }
}
impl Default for Target10Adapter {
    fn default() -> Self {
        Self::new()
    }
}

impl MessagePublisher for Target10Adapter {
    type Message = Target10Command;
    fn publish(&self, message: Self::Message) -> Result<(), TransportError> {
        self.send_command(&message)
            .map_err(|_| TransportError::Disconnected)
    }
}
