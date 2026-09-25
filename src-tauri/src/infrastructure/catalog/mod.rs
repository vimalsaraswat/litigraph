pub mod file_store;

pub use file_store::*;

// Re-export domain catalog abstractions for backward compatibility and convenience
pub use crate::domain::catalog::*;
