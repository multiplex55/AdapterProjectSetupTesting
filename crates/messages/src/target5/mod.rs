use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Target5Status {
    pub device_id: u32,
    pub online: bool,
    pub sequence: u64,
}
