use crate::domain::{CoreError, InputFrame};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationState {
    pub initialized: bool,
    pub last_sequence: Option<u64>,
    pub processed_count: u64,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            initialized: true,
            last_sequence: None,
            processed_count: 0,
        }
    }
}

impl SimulationState {
    pub fn advance(&mut self, frame: &InputFrame) -> Result<(), CoreError> {
        if !self.initialized {
            return Err(CoreError::InvalidStateTransition("state not initialized"));
        }

        if frame.payload.is_empty() {
            return Err(CoreError::InvalidInput("payload cannot be empty"));
        }

        if let Some(previous) = self.last_sequence {
            if frame.sequence <= previous {
                return Err(CoreError::InvalidStateTransition(
                    "sequence must strictly increase",
                ));
            }
        }

        self.last_sequence = Some(frame.sequence);
        self.processed_count += 1;
        Ok(())
    }
}
