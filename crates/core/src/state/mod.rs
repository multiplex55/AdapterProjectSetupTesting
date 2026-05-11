//! Core domain state ownership boundaries.
//!
//! Ownership rules:
//! - Domain decision state lives in `core/state`.
//! - Lifecycle/process state belongs in runtime crates.
//! - Handles/sockets/drivers belong in adapter crates.

mod link;
mod processing;
mod target10;
mod target5;

pub use link::LinkState;
pub use processing::CoreProcessingState;
pub use target10::Target10State;
pub use target5::Target5State;
