use core::flows::target5_to_target10::{Target5ToTarget10Effect, Target5ToTarget10FlowError};
use messages::EthernetEnvelope;
use ports::{MessagePublisher, TransportError};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EffectDispatchState {
    pub queued_commands: Vec<messages::Target10Command>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectDispatchError {
    Transport(TransportError),
    Flow(Target5ToTarget10FlowError),
}

pub fn dispatch_target5_to_target10_effects<T: MessagePublisher<Message = EthernetEnvelope>>(
    effects: Vec<Target5ToTarget10Effect>,
    transport: &T,
    dispatch_state: &mut EffectDispatchState,
) -> Result<(), EffectDispatchError> {
    for effect in effects {
        match effect {
            Target5ToTarget10Effect::QueueCommand(command) => {
                dispatch_state.queued_commands.push(command);
            }
            Target5ToTarget10Effect::SendEthernet(envelope) => {
                transport
                    .publish(envelope)
                    .map_err(EffectDispatchError::Transport)?;
            }
        }
    }

    Ok(())
}
