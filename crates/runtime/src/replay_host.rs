use core::{
    algorithms::target5_to_target10::{
        map_target5_status_to_target10_command, Target5ToTarget10Error,
    },
    flows::target5_to_target10::{run_target5_to_target10_flow, Target5ToTarget10FlowError},
    state::Target10State,
};
use messages::{EthernetEnvelope, Target5Status};
use ports::{MessagePublisher, TransportError};

use crate::{dispatch_target5_to_target10_effects, EffectDispatchState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayFlowSummary {
    pub commands_emitted: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayFlowError {
    Flow(Target5ToTarget10FlowError),
    Transport(TransportError),
}

pub fn run_target5_to_target10_replay_flow<T: MessagePublisher<Message = EthernetEnvelope>>(
    statuses: impl IntoIterator<Item = Target5Status>,
    transport: &T,
) -> Result<ReplayFlowSummary, ReplayFlowError> {
    let mut commands_emitted = 0u64;
    let mut flow_state = Target10State::default();
    let mut dispatch_state = EffectDispatchState::default();

    for status in statuses {
        let effects =
            run_target5_to_target10_flow(&mut flow_state, status).map_err(ReplayFlowError::Flow)?;
        dispatch_target5_to_target10_effects(effects, transport, &mut dispatch_state).map_err(
            |err| match err {
                crate::EffectDispatchError::Transport(transport) => {
                    ReplayFlowError::Transport(transport)
                }
                crate::EffectDispatchError::Flow(flow) => ReplayFlowError::Flow(flow),
            },
        )?;
        commands_emitted += 1;
    }

    Ok(ReplayFlowSummary { commands_emitted })
}

pub fn map_target5_statuses_to_target10_commands(
    statuses: impl IntoIterator<Item = Target5Status>,
) -> Result<Vec<messages::Target10Command>, Target5ToTarget10Error> {
    statuses
        .into_iter()
        .map(|status| map_target5_status_to_target10_command(&status))
        .collect()
}
