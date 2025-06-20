use std::collections::BTreeMap;
use crate::types::{Timestamp, Label, DataPoint};

#[derive(Default)]
pub struct MemTable {
    data: BTreeMap<(Label, Timestamp), DataPoint>,
}

impl MemTable {
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    /// Insert a datapoint into the memtable
    pub fn insert(&mut self, datapoint: &DataPoint) {
        let key = (datapoint.label.clone(), datapoint.timestamp.clone());
        self.data.insert(key, datapoint.clone());
    }

    // Get a single datapoint for a label and timestamp
    pub fn get(&self, label: &Label, timestamp: &Timestamp) -> Option<&DataPoint> {
        self.data.get(&(label.clone(), timestamp.clone()))
    }
    

    pub fn range(&self, label: &Label, start: &Timestamp, end: &Timestamp) -> Vec<&DataPoint> {
       self.data
        .range((label.clone(), *start)..=(label.clone(), *end))
        .filter_map(|((l, _), v)| if l == label { Some(v) } else { None })
        .collect()
    }

    pub fn get_all(&self) -> Vec<DataPoint> {
        self.data.values().cloned().collect()
    }

}