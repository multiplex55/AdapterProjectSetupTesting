//! Time abstraction ports.

use core::fmt;

/// Errors returned by [`Clock`] implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockError {
    /// Clock backend is unavailable.
    Unavailable,
}

impl fmt::Display for ClockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Provides current monotonic/logical time to core workflows.
///
/// # Ownership example
/// Inject as `&dyn Clock<Instant = I>` so tests can provide deterministic clocks while runtime
/// uses a system implementation.
pub trait Clock {
    /// Time representation used by the application.
    type Instant;

    /// Returns the current instant.
    fn now(&self) -> Result<Self::Instant, ClockError>;
}
