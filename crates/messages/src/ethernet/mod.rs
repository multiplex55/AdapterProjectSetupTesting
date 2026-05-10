use crate::common::MessageType;
use crate::{Target10Command, Target5Status};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "message_type", content = "payload")]
pub enum EthernetPayload {
    Target5Status(Target5Status),
    Target10Command(Target10Command),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthernetEnvelope {
    pub protocol_version: u16,
    pub payload: EthernetPayload,
}

impl EthernetEnvelope {
    pub fn message_type(&self) -> MessageType {
        match self.payload {
            EthernetPayload::Target5Status(_) => MessageType::Target5Status,
            EthernetPayload::Target10Command(_) => MessageType::Target10Command,
        }
    }
}
