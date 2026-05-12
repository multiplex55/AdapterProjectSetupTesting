use crate::algorithms::target5_to_target10::{
    map_target5_status_to_target10_command, Target5ToTarget10Error,
};
use crate::state::Target10State;
use messages::{ethernet::EthernetPayload, EthernetEnvelope, Target10Command, Target5Status};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target5ToTarget10Effect {
    SendEthernet(EthernetEnvelope),
    QueueCommand(Target10Command),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target5ToTarget10FlowError {
    InvalidSequence {
        received: u64,
        previous: Option<u64>,
    },
    Mapping(Target5ToTarget10Error),
}

pub fn run_target5_to_target10_flow(
    state: &mut Target10State,
    status: Target5Status,
) -> Result<Vec<Target5ToTarget10Effect>, Target5ToTarget10FlowError> {
    validate_sequence(state, &status)?;

    let command = map_target5_status_to_target10_command(&status)
        .map_err(Target5ToTarget10FlowError::Mapping)?;

    state.apply(status, command.clone());

    let envelope = EthernetEnvelope {
        protocol_version: 1,
        payload: EthernetPayload::Target10Command(command.clone()),
    };

    Ok(vec![
        Target5ToTarget10Effect::QueueCommand(command),
        Target5ToTarget10Effect::SendEthernet(envelope),
    ])
}

fn validate_sequence(
    state: &Target10State,
    status: &Target5Status,
) -> Result<(), Target5ToTarget10FlowError> {
    if status.sequence == 0 {
        return Err(Target5ToTarget10FlowError::InvalidSequence {
            received: status.sequence,
            previous: state.last_seen_status.as_ref().map(|s| s.sequence),
        });
    }

    if let Some(previous) = state.last_seen_status.as_ref().map(|s| s.sequence) {
        if status.sequence <= previous {
            return Err(Target5ToTarget10FlowError::InvalidSequence {
                received: status.sequence,
                previous: Some(previous),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        run_target5_to_target10_flow, Target5ToTarget10Effect, Target5ToTarget10FlowError,
    };
    use crate::algorithms::target5_to_target10::Target5ToTarget10Error;
    use crate::state::Target10State;
    use messages::Target5Status;

    #[test]
    fn flow_updates_state_and_returns_queue_and_ethernet_effects() {
        let mut state = Target10State::default();
        let status = Target5Status {
            device_id: 7,
            online: true,
            sequence: 3,
        };

        let effects =
            run_target5_to_target10_flow(&mut state, status.clone()).expect("flow succeeds");

        assert_eq!(effects.len(), 2);
        assert_eq!(state.last_seen_status, Some(status));
        assert_eq!(state.applied_events, 1);
        assert!(matches!(
            &effects[0],
            Target5ToTarget10Effect::QueueCommand(command)
            if command.command_id == 1007
        ));
        assert!(matches!(
            &effects[1],
            Target5ToTarget10Effect::SendEthernet(_)
        ));
    }

    #[test]
    fn flow_surfaces_typed_mapping_error() {
        let mut state = Target10State::default();
        let status = Target5Status {
            device_id: u32::MAX,
            online: true,
            sequence: 42,
        };

        let err = run_target5_to_target10_flow(&mut state, status).expect_err("overflow expected");

        assert_eq!(
            err,
            Target5ToTarget10FlowError::Mapping(Target5ToTarget10Error::CommandIdOverflow {
                device_id: u32::MAX,
                online: true,
            })
        );
    }
}
