//! Raw target C ABI declarations and FFI-compatible types only.

#![allow(non_camel_case_types)]

pub type c_int = i32;
pub type c_uint = u32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct target_packet_t {
    pub id: c_uint,
    pub len: c_uint,
    pub data: [u8; 256],
}

unsafe extern "C" {
    pub fn target_open(channel: c_uint) -> c_int;
    pub fn target_close(handle: c_int) -> c_int;
    pub fn target_send(handle: c_int, packet: *const target_packet_t) -> c_int;
    pub fn target_recv(handle: c_int, packet: *mut target_packet_t) -> c_int;
}
