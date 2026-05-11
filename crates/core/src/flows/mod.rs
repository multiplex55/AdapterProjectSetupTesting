//! Domain flow orchestration primitives.
//!
//! - **Algorithms** are pure transformations and calculations with no state mutation.
//! - **Flows** are multi-step domain orchestrations over mutable state.
//! - **Flows** return effect values that can be interpreted by outer layers and must avoid direct I/O.

pub mod target10;
pub mod target5;
pub mod target5_to_target10;
