//! Telemetry sink ports.

use core::fmt;

/// Errors returned by [`TelemetrySink`] implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetryError {
    /// Sink backend is currently unavailable.
    Unavailable,
    /// Event rejected by sink validation.
    Rejected,
}

impl fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Consumes telemetry events produced by core services.
///
/// # Lifecycle example
/// Runtime owns sink creation and shutdown. Core keeps a borrowed `&dyn TelemetrySink<Event = E>`
/// and emits best-effort events during processing.
pub trait TelemetrySink {
    /// Event payload type.
    type Event;

    /// Records one telemetry event.
    fn emit(&self, event: Self::Event) -> Result<(), TelemetryError>;
}
