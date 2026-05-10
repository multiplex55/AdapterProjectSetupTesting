//! Driver lifecycle/error ports.

use core::fmt;

/// Driver-level lifecycle errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    /// Driver start operation failed.
    StartFailed,
    /// Driver stop operation failed.
    StopFailed,
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
