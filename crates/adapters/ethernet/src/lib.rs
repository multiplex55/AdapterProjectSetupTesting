use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use messages::EthernetEnvelope;
use ports::{MessagePublisher, MessageSubscriber, TransportError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EthernetAdapterError {
    Timeout,
    TransportDisconnected,
    InvalidPayload,
}

#[derive(Debug, Clone, Default)]
struct SharedBus {
    queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
}

#[derive(Debug, Clone)]
pub struct LoopbackEthernetTransport {
    rx: SharedBus,
    tx: SharedBus,
}

impl LoopbackEthernetTransport {
    pub fn pair() -> (Self, Self) {
        let a_to_b = SharedBus::default();
        let b_to_a = SharedBus::default();

        (
            Self {
                rx: b_to_a.clone(),
                tx: a_to_b.clone(),
            },
            Self {
                rx: a_to_b,
                tx: b_to_a,
            },
        )
    }

    pub fn serialize(envelope: &EthernetEnvelope) -> Result<Vec<u8>, EthernetAdapterError> {
        serde_json::to_vec(envelope).map_err(|_| EthernetAdapterError::InvalidPayload)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<EthernetEnvelope, EthernetAdapterError> {
        serde_json::from_slice(bytes).map_err(|_| EthernetAdapterError::InvalidPayload)
    }
}

impl MessagePublisher for LoopbackEthernetTransport {
    type Message = EthernetEnvelope;

    fn publish(&self, message: Self::Message) -> Result<(), TransportError> {
        let bytes = Self::serialize(&message).map_err(|e| match e {
            EthernetAdapterError::InvalidPayload => TransportError::InvalidPayload,
            EthernetAdapterError::Timeout => TransportError::Timeout,
            EthernetAdapterError::TransportDisconnected => TransportError::Disconnected,
        })?;

        let mut queue = self
            .tx
            .queue
            .lock()
            .map_err(|_| TransportError::Disconnected)?;
        queue.push_back(bytes);
        Ok(())
    }
}

impl MessageSubscriber for LoopbackEthernetTransport {
    type Message = EthernetEnvelope;

    fn receive(&self) -> Result<Self::Message, TransportError> {
        let mut queue = self
            .rx
            .queue
            .lock()
            .map_err(|_| TransportError::Disconnected)?;
        let bytes = queue.pop_front().ok_or(TransportError::Timeout)?;
        Self::deserialize(&bytes).map_err(|_| TransportError::InvalidPayload)
    }
}

#[cfg(test)]
mod tests {
    use messages::{ethernet::EthernetPayload, EthernetEnvelope, Target10Command};
    use ports::{MessagePublisher, MessageSubscriber};

    use super::LoopbackEthernetTransport;

    #[test]
    fn loopback_exchanges_messages_between_two_endpoints() {
        let (target5_side, target10_side) = LoopbackEthernetTransport::pair();
        let outbound = EthernetEnvelope {
            protocol_version: 1,
            payload: EthernetPayload::Target10Command(Target10Command {
                command_id: 42,
                action: "sync".to_string(),
                priority: 7,
            }),
        };

        target10_side
            .publish(outbound.clone())
            .expect("publish succeeds");
        let inbound = target5_side.receive().expect("receive succeeds");

        assert_eq!(inbound, outbound);
    }
}
