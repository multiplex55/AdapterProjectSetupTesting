//! Data acquisition ports.

use core::fmt;

/// Errors returned by [`DataSource`] implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSourceError {
    /// The source is temporarily unavailable and may recover.
    Unavailable,
    /// The requested input is not present.
    NotFound,
    /// Input was malformed and could not be decoded.
    InvalidData,
}

impl fmt::Display for DataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Supplies domain input data to core workflows.
///
/// # Lifecycle example
/// - Construct an implementation once during application bootstrap.
/// - Share it with core via `&dyn DataSource<Item = T>` for request/response reads.
/// - Adapter crates own connection pooling and retry policy.
pub trait DataSource {
    /// Domain item returned by this source.
    type Item;

    /// Reads the next available item.
    fn read(&self) -> Result<Self::Item, DataSourceError>;
}
