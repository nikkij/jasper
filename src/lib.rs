pub mod network;
pub mod storage;
pub mod types;

pub use network::TCPServer;
pub use storage::{StorageEngine, Wal};
pub use types::{Timestamp, Label, DataPoint};

// Current version of jasper
pub const VERSION: &str = env!("CARGO_PKG_VERSION");