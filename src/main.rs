use storage::engine::StorageEngine;
use std::io;

mod storage;

fn main() -> io::Result<()> {
    let db = StorageEngine::new("test.db");
    db.write("hello")?;
    let contents = db.read()?;
    println!("{}", contents);
    Ok(())
}
