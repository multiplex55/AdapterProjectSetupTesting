use crate::domain::{CoreError, InputFrame, OutputFrame};

pub mod target5_to_target10;
use ports::AlgorithmProvider;

pub fn process_frame_with_provider<P>(
    provider: &P,
    input: InputFrame,
) -> Result<OutputFrame, CoreError>
where
    P: AlgorithmProvider<Input = InputFrame, Output = Vec<u8>>,
{
    let payload = provider
        .compute(input.clone())
        .map_err(|_| CoreError::AlgorithmFailed("provider compute failed"))?;

    Ok(OutputFrame {
        protocol_version: input.protocol_version,
        sequence: input.sequence,
        message_type: input.message_type,
        payload,
    })
}
