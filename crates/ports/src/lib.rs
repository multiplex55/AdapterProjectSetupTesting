//! Capability-focused port contracts that core logic depends on.
//!
//! This crate intentionally defines traits and errors only; implementations
//! belong in adapter crates.

pub mod clock;
pub mod data_source;
pub mod driver;
pub mod provider;
pub mod telemetry;
pub mod transport;

pub use clock::{Clock, ClockError};
pub use data_source::{DataSource, DataSourceError};
pub use driver::DriverError;
pub use provider::{AlgorithmProvider, ProviderError};
pub use telemetry::{TelemetryError, TelemetrySink};
pub use transport::{
    CommType1Transport, CommType2Transport, MessagePublisher, MessageSubscriber, TransportError,
};

pub fn crate_name() -> &'static str {
    "ports"
}
