//! Shared wire contracts and serialization helpers.

pub mod common;
pub mod ethernet;
pub mod target10;
pub mod target5;
pub mod versioning;

pub use ethernet::EthernetEnvelope;
pub use target10::Target10Command;
pub use target5::Target5Status;
