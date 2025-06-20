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
    pub fn read_all(&self) -> Vec<DataPoint> {
        let datapoints = self.memtable.get_all();

        println!("Read all datapoints successfully, count: {}", datapoints.len());
        return datapoints
    }

    // Read filtered by label and timestamp
    pub fn read_filtered(
        &self,
        label: &Label,
        start: &Timestamp,
        end: &Timestamp,
    ) -> Vec<DataPoint> {
        println!("Enter read_filtered");
        let all = self.read_all();
        println!("Total datapoints: {}", all.len());

        let filtered: Vec<_> = all.into_iter().filter(|p| {
            let label_match = &p.label == label;
            let ts = p.timestamp.0;
            let ts_match = ts >= start.0 && ts <= end.0;
            println!(
                "Checking dp: label='{}' timestamp={} label_match={} ts_match={}",
                p.label.0, ts, label_match, ts_match
            );
            label_match && ts_match
        }).collect();

        println!("Filtered datapoints count: {}", filtered.len());
        return filtered;
    }
}