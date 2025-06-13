use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use crate::types::DataPoint;

pub struct Wal {
    writer: BufWriter<File>,
}

impl Wal {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    pub fn append(&mut self, datapoint: &DataPoint) -> std::io::Result<()> {
        let serialized = serde_json::to_string(datapoint)?;
        writeln!(self.writer, "{}", serialized)?;
        // self.writer.write_all(serialized.as_bytes())?;
        self.writer.flush()?; // Force flush for durability
        Ok(())
    }
}