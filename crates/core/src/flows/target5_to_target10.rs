use crate::algorithms::target5_to_target10::{
    map_target5_status_to_target10_command, Target5ToTarget10Error,
};
use crate::state::Target10State;
use messages::{Target10Command, Target5Status};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target5ToTarget10Effect {
    EmitTarget10Command(Target10Command),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target5ToTarget10FlowError {
    Mapping(Target5ToTarget10Error),
    StateInvariantViolation(&'static str),
}

pub fn run_target5_to_target10_flow(
    state: &mut Target10State,
    status: Target5Status,
) -> Result<Vec<Target5ToTarget10Effect>, Target5ToTarget10FlowError> {
    if status.sequence == 0 {
        return Err(Target5ToTarget10FlowError::StateInvariantViolation(
            "sequence must be non-zero",
        ));
    }

    let command = map_target5_status_to_target10_command(&status)
        .map_err(Target5ToTarget10FlowError::Mapping)?;

    state.apply(status, command.clone());

    Ok(vec![Target5ToTarget10Effect::EmitTarget10Command(command)])
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
    fn flow_updates_state_and_returns_effect() {
        let mut state = Target10State::default();
        let status = Target5Status {
            device_id: 7,
            online: true,
            sequence: 3,
        };

        let effects =
            run_target5_to_target10_flow(&mut state, status.clone()).expect("flow succeeds");

        assert_eq!(effects.len(), 1);
        assert_eq!(state.last_seen_status, Some(status));
        assert_eq!(state.applied_events, 1);
        assert!(matches!(
            &effects[0],
            Target5ToTarget10Effect::EmitTarget10Command(command)
            if command.command_id == 1007
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

    #[test]
    fn flow_surfaces_state_invariant_failure() {
        let mut state = Target10State::default();
        let status = Target5Status {
            device_id: 1,
            online: true,
            sequence: 0,
        };

        let err =
            run_target5_to_target10_flow(&mut state, status).expect_err("zero sequence rejected");

        assert_eq!(
            err,
            Target5ToTarget10FlowError::StateInvariantViolation("sequence must be non-zero")
        );
    }
}
