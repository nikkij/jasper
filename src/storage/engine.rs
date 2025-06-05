use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

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

    pub fn write(&self, line: &str) -> io::Result<()> {
        // Set the file path
        //let path = "lines.txt";
        let path = &self.path;

        // Open the file in append mode, create it if it doesn't exist
        let mut output = OpenOptions::new()
            .create(true)      // create if it doesn't exist
            .append(true)      // append to the end of the file
            .open(path)?;      // open the file

        // Write the line to the file
        write!(output, "{}\n", line)?;

        Ok(())
    }

    pub fn read(&self) -> io::Result<String> {
        let file = File::open(&self.path)?;
        let buffered = BufReader::new(file);
        let mut contents = String::new();
        for line in buffered.lines() {
            contents.push_str(&line?);
            contents.push('\n');
        }
        Ok(contents)
    }
}