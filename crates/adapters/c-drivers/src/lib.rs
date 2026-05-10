//! Safe wrappers over raw target C drivers.

use ffi_target_bindings::{
    c_int, c_uint, target_close, target_open, target_packet_t, target_recv, target_send,
};
use messages::{Target10Command, Target5Status};
use ports::{DataSource, DataSourceError, MessagePublisher, TransportError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CDriverInfraError {
    HardwareUnavailable,
    IoFailure,
    InvalidResponse,
}

pub trait CDriverApi: Clone {
    fn open(&self, channel: c_uint) -> c_int;
    fn close(&self, handle: c_int) -> c_int;
    fn send(&self, handle: c_int, packet: &target_packet_t) -> c_int;
    fn recv(&self, handle: c_int, packet: &mut target_packet_t) -> c_int;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FfiCDriverApi;

impl CDriverApi for FfiCDriverApi {
    fn open(&self, channel: c_uint) -> c_int {
        // SAFETY: FFI call with primitive argument only; no aliasing or borrowed data crosses boundary.
        unsafe { target_open(channel) }
    }

    fn close(&self, handle: c_int) -> c_int {
        // SAFETY: FFI call uses validated handle returned by target_open.
        unsafe { target_close(handle) }
    }

    fn send(&self, handle: c_int, packet: &target_packet_t) -> c_int {
        // SAFETY: packet points to initialized memory valid for call duration.
        unsafe { target_send(handle, packet as *const target_packet_t) }
    }

    fn recv(&self, handle: c_int, packet: &mut target_packet_t) -> c_int {
        // SAFETY: packet is a valid mutable output buffer for call duration.
        unsafe { target_recv(handle, packet as *mut target_packet_t) }
    }
}

pub struct TargetDriver<A: CDriverApi = FfiCDriverApi> {
    handle: c_int,
    api: A,
}

impl TargetDriver<FfiCDriverApi> {
    pub fn connect(channel: c_uint) -> Result<Self, CDriverInfraError> {
        Self::connect_with_api(channel, FfiCDriverApi)
    }
}

impl<A: CDriverApi> TargetDriver<A> {
    pub fn connect_with_api(channel: c_uint, api: A) -> Result<Self, CDriverInfraError> {
        let handle = api.open(channel);
        if handle < 0 {
            return Err(CDriverInfraError::HardwareUnavailable);
        }
        Ok(Self { handle, api })
    }

    pub fn disconnect(&self) -> Result<(), CDriverInfraError> {
        let rc = self.api.close(self.handle);
        if rc != 0 {
            return Err(CDriverInfraError::IoFailure);
        }
        Ok(())
    }
}

impl<A: CDriverApi> MessagePublisher for TargetDriver<A> {
    type Message = Target10Command;

    fn publish(&self, message: Self::Message) -> Result<(), TransportError> {
        let mut packet = target_packet_t {
            id: message.command_id,
            len: 0,
            data: [0; 256],
        };
        let bytes = message.action.as_bytes();
        if bytes.len() > packet.data.len() {
            return Err(TransportError::InvalidPayload);
        }
        packet.len = bytes.len() as c_uint;
        packet.data[..bytes.len()].copy_from_slice(bytes);

        let rc = self.api.send(self.handle, &packet);
        if rc != 0 {
            return Err(TransportError::Disconnected);
        }
        Ok(())
    }
}

impl<A: CDriverApi> DataSource for TargetDriver<A> {
    type Item = Target5Status;

    fn read(&self) -> Result<Self::Item, DataSourceError> {
        let mut packet = target_packet_t {
            id: 0,
            len: 0,
            data: [0; 256],
        };
        let rc = self.api.recv(self.handle, &mut packet);
        if rc != 0 {
            return Err(DataSourceError::Unavailable);
        }

        if (packet.len as usize) > packet.data.len() {
            return Err(DataSourceError::InvalidData);
        }

        Ok(Target5Status {
            device_id: packet.id,
            online: true,
            sequence: u64::from(packet.len),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct MockCDriverApi {
        state: Rc<RefCell<MockState>>,
    }

    #[derive(Default)]
    struct MockState {
        open_rc: c_int,
        close_rc: c_int,
        send_rc: c_int,
        recv_rc: c_int,
        recv_len: c_uint,
        recv_id: c_uint,
        sent_packets: Vec<target_packet_t>,
    }

    impl MockCDriverApi {
        fn with_state(state: MockState) -> Self {
            Self {
                state: Rc::new(RefCell::new(state)),
            }
        }
    }

    impl CDriverApi for MockCDriverApi {
        fn open(&self, _channel: c_uint) -> c_int {
            self.state.borrow().open_rc
        }

        fn close(&self, _handle: c_int) -> c_int {
            self.state.borrow().close_rc
        }

        fn send(&self, _handle: c_int, packet: &target_packet_t) -> c_int {
            let mut state = self.state.borrow_mut();
            state.sent_packets.push(*packet);
            state.send_rc
        }

        fn recv(&self, _handle: c_int, packet: &mut target_packet_t) -> c_int {
            let state = self.state.borrow();
            packet.id = state.recv_id;
            packet.len = state.recv_len;
            state.recv_rc
        }
    }

    #[test]
    fn connect_translates_open_error() {
        let api = MockCDriverApi::with_state(MockState {
            open_rc: -1,
            ..Default::default()
        });

        let result = TargetDriver::connect_with_api(1, api);
        assert_eq!(result.err(), Some(CDriverInfraError::HardwareUnavailable));
    }

    #[test]
    fn publish_rejects_oversized_payload() {
        let api = MockCDriverApi::with_state(MockState {
            open_rc: 10,
            ..Default::default()
        });
        let driver = TargetDriver::connect_with_api(1, api).expect("driver should connect");
        let oversized = "x".repeat(257);

        let err = driver
            .publish(Target10Command {
                command_id: 7,
                action: oversized,
                priority: 1,
            })
            .unwrap_err();

        assert_eq!(err, TransportError::InvalidPayload);
    }

    #[test]
    fn publish_translates_send_error() {
        let api = MockCDriverApi::with_state(MockState {
            open_rc: 10,
            send_rc: -5,
            ..Default::default()
        });
        let driver = TargetDriver::connect_with_api(1, api).expect("driver should connect");

        let err = driver
            .publish(Target10Command {
                command_id: 9,
                action: "arm".to_string(),
                priority: 1,
            })
            .unwrap_err();

        assert_eq!(err, TransportError::Disconnected);
    }

    #[test]
    fn read_rejects_invalid_boundary_length() {
        let api = MockCDriverApi::with_state(MockState {
            open_rc: 10,
            recv_len: 999,
            ..Default::default()
        });
        let driver = TargetDriver::connect_with_api(1, api).expect("driver should connect");

        let err = driver.read().unwrap_err();
        assert_eq!(err, DataSourceError::InvalidData);
    }
}
