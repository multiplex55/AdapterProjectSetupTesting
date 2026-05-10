use messages::EthernetEnvelope;
use ports::{CommType2Transport, MessagePublisher, MessageSubscriber, TransportError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommType2InfraError {
    HardwarePathUnimplemented,
}

pub struct CommType2Adapter;
impl CommType2Adapter {
    pub fn new() -> Self {
        Self
    }
}
impl Default for CommType2Adapter {
    fn default() -> Self {
        Self::new()
    }
}

impl MessagePublisher for CommType2Adapter {
    type Message = EthernetEnvelope;
    fn publish(&self, _message: Self::Message) -> Result<(), TransportError> {
        Err(TransportError::Disconnected)
    }
}
impl MessageSubscriber for CommType2Adapter {
    type Message = EthernetEnvelope;
    fn receive(&self) -> Result<Self::Message, TransportError> {
        Err(TransportError::Timeout)
    }
}
impl CommType2Transport for CommType2Adapter {}
