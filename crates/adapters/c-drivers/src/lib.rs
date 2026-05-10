//! Safe wrappers over raw target C drivers.

use ffi_target_bindings::{c_int, c_uint, target_close, target_open, target_recv, target_send, target_packet_t};
use messages::{Target10Command, Target5Status};
use ports::{DataSource, DataSourceError, MessagePublisher, TransportError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CDriverInfraError {
    HardwareUnavailable,
    IoFailure,
    InvalidResponse,
}

pub struct TargetDriver {
    handle: c_int,
}

impl TargetDriver {
    pub fn connect(channel: c_uint) -> Result<Self, CDriverInfraError> {
        // SAFETY: FFI call with primitive argument only; no aliasing or borrowed data crosses boundary.
        let handle = unsafe { target_open(channel) };
        if handle < 0 {
            return Err(CDriverInfraError::HardwareUnavailable);
        }
        Ok(Self { handle })
    }

    pub fn disconnect(&self) -> Result<(), CDriverInfraError> {
        // SAFETY: FFI call uses validated handle returned by target_open.
        let rc = unsafe { target_close(self.handle) };
        if rc != 0 {
            return Err(CDriverInfraError::IoFailure);
        }
        Ok(())
    }
}

impl MessagePublisher for TargetDriver {
    type Message = Target10Command;

    fn publish(&self, message: Self::Message) -> Result<(), TransportError> {
        let mut packet = target_packet_t { id: message.command_id, len: 0, data: [0; 256] };
        let bytes = message.action.as_bytes();
        if bytes.len() > packet.data.len() {
            return Err(TransportError::InvalidPayload);
        }
        packet.len = bytes.len() as c_uint;
        packet.data[..bytes.len()].copy_from_slice(bytes);

        // SAFETY: packet points to initialized stack memory valid for call duration.
        let rc = unsafe { target_send(self.handle, &packet as *const target_packet_t) };
        if rc != 0 {
            return Err(TransportError::Disconnected);
        }
        Ok(())
    }
}

impl DataSource for TargetDriver {
    type Item = Target5Status;

    fn read(&self) -> Result<Self::Item, DataSourceError> {
        let mut packet = target_packet_t { id: 0, len: 0, data: [0; 256] };
        // SAFETY: packet is valid mutable output buffer for call duration.
        let rc = unsafe { target_recv(self.handle, &mut packet as *mut target_packet_t) };
        if rc != 0 {
            return Err(DataSourceError::Unavailable);
        }

        Ok(Target5Status {
            device_id: packet.id,
            online: true,
            sequence: u64::from(packet.len),
        })
    }
}
