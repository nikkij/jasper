pub mod storage;
pub mod types;

pub use storage::StorageEngine;
pub use types::{Timestamp, Label, DataPoint};

// Current version of jasper
pub const VERSION: &str = env!("CARGO_PKG_VERSION");