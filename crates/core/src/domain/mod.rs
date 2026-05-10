use messages::common::MessageType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputFrame {
    pub protocol_version: u16,
    pub sequence: u64,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputFrame {
    pub protocol_version: u16,
    pub sequence: u64,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    InvalidInput(&'static str),
    InvalidStateTransition(&'static str),
    ValidationFailed(&'static str),
    AlgorithmFailed(&'static str),
    UnsupportedMessageVersion { found: u16, min: u16, max: u16 },
}
