use messages::{Target10Command, Target5Status};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target5ToTarget10Error {
    CommandIdOverflow { device_id: u32, online: bool },
}

pub fn map_target5_status_to_target10_command(
    status: &Target5Status,
) -> Result<Target10Command, Target5ToTarget10Error> {
    let (base, action, priority) = if status.online {
        (1000_u32, "arm", 1_u8)
    } else {
        (2000_u32, "standby", 5_u8)
    };

    let command_id = base
        .checked_add(status.device_id)
        .ok_or(Target5ToTarget10Error::CommandIdOverflow {
            device_id: status.device_id,
            online: status.online,
        })?;

    Ok(Target10Command {
        command_id,
        action: action.to_string(),
        priority,
    })
}

#[cfg(test)]
mod tests {
    use super::{map_target5_status_to_target10_command, Target5ToTarget10Error};
    use messages::{Target10Command, Target5Status};

    #[test]
    fn maps_online_status_to_arm_command() {
        let status = Target5Status {
            device_id: 5,
            online: true,
            sequence: 1,
        };

        let command = map_target5_status_to_target10_command(&status).expect("mapping succeeds");

        assert_eq!(
            command,
            Target10Command {
                command_id: 1005,
                action: "arm".to_string(),
                priority: 1,
            }
        );
    }

    #[test]
    fn maps_offline_status_to_standby_command() {
        let status = Target5Status {
            device_id: 10,
            online: false,
            sequence: 2,
        };

        let command = map_target5_status_to_target10_command(&status).expect("mapping succeeds");

        assert_eq!(
            command,
            Target10Command {
                command_id: 2010,
                action: "standby".to_string(),
                priority: 5,
            }
        );
    }

    #[test]
    fn maps_edge_device_ids_without_overflow() {
        let min = Target5Status {
            device_id: 0,
            online: true,
            sequence: 1,
        };
        let max_safe = Target5Status {
            device_id: u32::MAX - 2000,
            online: false,
            sequence: 2,
        };

        let min_command = map_target5_status_to_target10_command(&min).expect("min maps");
        let max_command = map_target5_status_to_target10_command(&max_safe).expect("max maps");

        assert_eq!(min_command.command_id, 1000);
        assert_eq!(max_command.command_id, u32::MAX);
    }

    #[test]
    fn returns_explicit_error_for_command_id_overflow() {
        let status = Target5Status {
            device_id: u32::MAX,
            online: true,
            sequence: 99,
        };

        let err = map_target5_status_to_target10_command(&status).expect_err("overflow expected");

        assert_eq!(
            err,
            Target5ToTarget10Error::CommandIdOverflow {
                device_id: u32::MAX,
                online: true,
            }
        );
    }
}
