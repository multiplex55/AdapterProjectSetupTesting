//! Messaging transport ports.

use core::fmt;

/// Errors returned by transport-facing traits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    /// Underlying channel is not connected.
    Disconnected,
    /// Send/receive timed out.
    Timeout,
    /// Payload or framing is invalid.
    InvalidPayload,
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Publishes outbound messages.
///
/// # Ownership example
/// Keep one publisher instance per logical link and inject `&dyn MessagePublisher<Message = M>`
/// into core services that emit commands.
pub trait MessagePublisher {
    /// Transport payload type.
    type Message;

    /// Publishes a message to the transport.
    fn publish(&self, message: Self::Message) -> Result<(), TransportError>;
}

/// Subscribes and receives inbound messages.
///
/// # Lifecycle example
/// Adapters may run an internal receive loop; core polls via `receive` when ready.
pub trait MessageSubscriber {
    /// Transport payload type.
    type Message;

    /// Receives one message if available.
    fn receive(&self) -> Result<Self::Message, TransportError>;
}

/// Capability marker for transports supporting communication type 1.
pub trait CommType1Transport: MessagePublisher + MessageSubscriber {}

/// Capability marker for transports supporting communication type 2.
pub trait CommType2Transport: MessagePublisher + MessageSubscriber {}
