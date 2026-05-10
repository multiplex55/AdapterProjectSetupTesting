use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Target10Command {
    pub command_id: u32,
    pub action: String,
    pub priority: u8,
}
