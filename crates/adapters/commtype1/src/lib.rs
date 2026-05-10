use messages::EthernetEnvelope;
use ports::{CommType1Transport, MessagePublisher, MessageSubscriber, TransportError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommType1InfraError { HardwarePathUnimplemented }

pub struct CommType1Adapter;
impl CommType1Adapter { pub fn new() -> Self { Self } }
impl Default for CommType1Adapter { fn default() -> Self { Self::new() } }

impl MessagePublisher for CommType1Adapter {
    type Message = EthernetEnvelope;
    fn publish(&self, _message: Self::Message) -> Result<(), TransportError> { Err(TransportError::Disconnected) }
}
impl MessageSubscriber for CommType1Adapter {
    type Message = EthernetEnvelope;
    fn receive(&self) -> Result<Self::Message, TransportError> { Err(TransportError::Timeout) }
}
impl CommType1Transport for CommType1Adapter {}
