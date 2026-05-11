use crate::algorithms::target5_to_target10::{
    map_target5_status_to_target10_command, Target5ToTarget10Error,
};
use messages::{Target10Command, Target5Status};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target5ToTarget10FlowError {
    Mapping(Target5ToTarget10Error),
}

pub fn orchestrate_target5_to_target10_command(
    status: &Target5Status,
) -> Result<Target10Command, Target5ToTarget10FlowError> {
    map_target5_status_to_target10_command(status).map_err(Target5ToTarget10FlowError::Mapping)
}

#[cfg(test)]
mod tests {
    use super::{orchestrate_target5_to_target10_command, Target5ToTarget10FlowError};
    use crate::algorithms::target5_to_target10::Target5ToTarget10Error;
    use messages::Target5Status;

    #[test]
    fn flow_orchestrates_algorithm_mapping() {
        let status = Target5Status {
            device_id: 7,
            online: true,
            sequence: 3,
        };

        let command = orchestrate_target5_to_target10_command(&status).expect("flow succeeds");

        assert_eq!(command.command_id, 1007);
        assert_eq!(command.action, "arm");
        assert_eq!(command.priority, 1);
    }

    #[test]
    fn flow_surfaces_typed_mapping_error() {
        let status = Target5Status {
            device_id: u32::MAX,
            online: true,
            sequence: 42,
        };

        let err = orchestrate_target5_to_target10_command(&status).expect_err("overflow expected");

        assert_eq!(
            err,
            Target5ToTarget10FlowError::Mapping(Target5ToTarget10Error::CommandIdOverflow {
                device_id: u32::MAX,
                online: true,
            })
        );
    }
}
