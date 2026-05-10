use serde::{Deserialize, Serialize};

/// Wire protocol version currently emitted by this crate.
pub const CURRENT_PROTOCOL_VERSION: u16 = 1;

/// Inclusive minimum accepted protocol version.
pub const MIN_SUPPORTED_PROTOCOL_VERSION: u16 = 1;

/// Inclusive maximum accepted protocol version.
pub const MAX_SUPPORTED_PROTOCOL_VERSION: u16 = 1;

/// Message type discriminator used by the Ethernet envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum MessageType {
    Target5Status = 0x0005,
    Target10Command = 0x000A,
}

impl MessageType {
    pub fn from_code(value: u16) -> Option<Self> {
        match value {
            0x0005 => Some(Self::Target5Status),
            0x000A => Some(Self::Target10Command),
            _ => None,
        }
    }

    pub const fn code(self) -> u16 {
        self as u16
    }
}
