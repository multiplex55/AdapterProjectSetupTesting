//! Replaceable compute provider ports.

use core::fmt;

/// Errors returned by [`AlgorithmProvider`] implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderError {
    /// Provider setup is incomplete or unavailable.
    Unavailable,
    /// Input is invalid for the selected algorithm.
    InvalidInput,
    /// Computation could not complete successfully.
    ComputeFailed,
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Interface for pluggable algorithm execution.
///
/// # Ownership example
/// Construct provider instances in runtime/adapters and inject trait objects into core to
/// switch algorithms without changing core crate dependencies.
pub trait AlgorithmProvider {
    /// Input type consumed by the provider.
    type Input;
    /// Output type produced by the provider.
    type Output;

    /// Executes provider-specific logic.
    fn compute(&self, input: Self::Input) -> Result<Self::Output, ProviderError>;
}
