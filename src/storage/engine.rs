use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufRead, Write};

use crate::types::{DataPoint, Label, Timestamp};
use crate::storage::wal::Wal;

use serde_json;
pub struct StorageEngine {
    pub path: String,
    pub file: io::Result<File>,
}

impl StorageEngine {
    pub fn new(path: &str) -> Self {
        StorageEngine {
            path: path.to_string(),
            file: File::open(path),
        }
    }

    pub fn write(&self, datapoint: &DataPoint) -> io::Result<()> {
        // Set the file path
        let path = &self.path;

        // Open the file in append mode, create it if it doesn't exist
        let mut file = OpenOptions::new()
            .create(true)      // create if it doesn't exist
            .append(true)      // append to the end of the file
            .open(path)?;      // open the file

        // Write the line to the file
        let line = datapoint;
        let mut writer = Wal::new(&path)?;
        writer.append(&datapoint)?;

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

    // pub fn read(&self) -> io::Result<String> {
    //     let file = File::open(&self.path)?;
    //     let buffered = BufReader::new(file);
    //     let mut contents = String::new();
    //     for line in buffered.lines() {
    //         contents.push_str(&line?);
    //         contents.push('\n');
    //     }
    //     Ok(contents)
    // }
}