pub mod network;
pub mod storage;
pub mod types;
pub mod query;

pub use network::TCPServer;
pub use storage::{StorageEngine, Wal};
pub use types::{Timestamp, Label, DataPoint};
pub use query::executor::execute_query;

// Current version of jasper
pub const VERSION: &str = env!("CARGO_PKG_VERSION");