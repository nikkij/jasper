use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufRead, Write};

use crate::types::{DataPoint, Label, Timestamp};
use crate::storage::wal::Wal;
use crate::storage::memtable::MemTable;

use serde_json;
pub struct StorageEngine {
    pub path: String,
    pub wal: Wal,
    pub memtable: MemTable,
}

impl StorageEngine {
    pub fn new(path: &str) -> std::io::Result<Self> {
        // Init data structures
        let wal = Wal::new(path)?;       
        let memtable = MemTable::new();  

        // Return instance
        Ok(Self {
            path: path.to_string(),  // field name: value
            wal,                     // reusing the local variable
            memtable,
        })
    }

    pub fn write(&mut self, datapoint: &DataPoint) -> io::Result<()> {
        // Get desired WAL file path
        let path = &self.path;

        // Write datapoint to WAL
        self.wal.append(&datapoint)?;

        // Write datapoint to memtable
        self.memtable.insert(datapoint);

        Ok(())
    }

    /// Read all datapoints (you can later filter here)
    pub fn read_all(&self) -> io::Result<Vec<DataPoint>> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut datapoints = vec![];

        for line in reader.lines() {
            let json = line?;
            let datapoint: DataPoint = serde_json::from_str(&json)?;
            datapoints.push(datapoint);
        }

        Ok(datapoints)
    }

    // Read filtered by label and timestamp
    pub fn read_filtered(
        &self,
        label: &Label,
        start: &Timestamp,
        end: &Timestamp,
    ) -> io::Result<Vec<DataPoint>> {
        let all = self.read_all()?;
        let filtered = all
            .into_iter()
            .filter(|p| &p.label == label && p.timestamp.0 >= start.0 && p.timestamp.0 <= end.0)
            .collect();
        Ok(filtered)
    }
}