use crate::algorithms::process_frame_with_provider;
use crate::domain::{CoreError, InputFrame, OutputFrame};
use crate::state::CoreProcessingState;
use messages::versioning::{validate_protocol_version, VersioningError};
use ports::AlgorithmProvider;

pub struct CoreService<P>
where
    P: AlgorithmProvider<Input = InputFrame, Output = Vec<u8>>,
{
    provider: P,
    state: CoreProcessingState,
}

impl<P> CoreService<P>
where
    P: AlgorithmProvider<Input = InputFrame, Output = Vec<u8>>,
{
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            state: CoreProcessingState::default(),
        }
    }

    pub fn state(&self) -> &CoreProcessingState {
        &self.state
    }

    pub fn process(&mut self, input: InputFrame) -> Result<OutputFrame, CoreError> {
        validate_protocol_version(input.protocol_version).map_err(map_versioning_error)?;
        self.state.record_processed_frame(&input)?;
        process_frame_with_provider(&self.provider, input)
    }
}

fn map_versioning_error(err: VersioningError) -> CoreError {
    match err {
        VersioningError::UnsupportedProtocolVersion { found, min, max } => {
            CoreError::UnsupportedMessageVersion { found, min, max }
        }
        VersioningError::UnknownMessageType(_) => {
            CoreError::ValidationFailed("unknown message type")
        }
    }
}
