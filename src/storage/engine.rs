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
    println!("Opening WAL file at path: {}", &self.path);
    let file = File::open(&self.path)?;
    println!("File opened successfully");
    
    let reader = BufReader::new(file);
    let mut datapoints = vec![];

    for (idx, line) in reader.lines().enumerate() {
        let json = line?;
        println!("Read line {}: {}", idx, json);

        let datapoint: DataPoint = match serde_json::from_str(&json) {
            Ok(dp) => dp,
            Err(e) => {
                println!("Failed to parse DataPoint on line {}: {:?}", idx, e);
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
            }
        };
        datapoints.push(datapoint);
    }

    println!("Read all datapoints successfully, count: {}", datapoints.len());
    Ok(datapoints)
}

    // Read filtered by label and timestamp
pub fn read_filtered(
    &self,
    label: &Label,
    start: &Timestamp,
    end: &Timestamp,
) -> io::Result<Vec<DataPoint>> {
    println!("Enter read_filtered");
    let all = self.read_all()?;
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
    Ok(filtered)
}
}