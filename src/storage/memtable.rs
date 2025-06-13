use std::collections::BTreeMap;
use crate::data::{DataPoint, Label, Timestamp};

use std::collections::BTreeMap;
use crate::data::{DataPoint, Label, Timestamp};

#[derive(Default)]
pub struct MemTable {
    data: BTreeMap<(Label, Timestamp), DataPoint>,
}

impl MemTable {
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    pub fn insert(&mut self, point: DataPoint) {
        let key = (point.label.clone(), point.timestamp);
        self.data.insert(key, point);
    }

    pub fn get(&self, label: &Label, timestamp: &Timestamp) -> Option<&DataPoint> {
        self.data.get(&(label.clone(), *timestamp))
    }

    pub fn range(&self, label: &Label, start: &Timestamp, end: &Timestamp) -> Vec<&DataPoint> {
        self.data
            .range((label.clone(), *start)..=(label.clone(), *end))
            .map(|(_, v)| v)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
