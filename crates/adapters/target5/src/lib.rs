use messages::Target5Status;
use ports::{DataSource, DataSourceError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target5InfraError {
    HardwarePathUnimplemented,
}

pub struct Target5Adapter;

impl Target5Adapter {
    pub fn new() -> Self {
        Self
    }

    pub fn poll_hardware(&self) -> Result<Target5Status, Target5InfraError> {
        Err(Target5InfraError::HardwarePathUnimplemented)
    }
}

impl Default for Target5Adapter {
    fn default() -> Self {
        Self::new()
    }
}

impl DataSource for Target5Adapter {
    type Item = Target5Status;
    fn read(&self) -> Result<Self::Item, DataSourceError> {
        self.poll_hardware()
            .map_err(|_| DataSourceError::Unavailable)
    }
}
