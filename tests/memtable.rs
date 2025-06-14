use jasper::types::{DataPoint, Label, Timestamp};
use jasper::storage::memtable::MemTable;
use serde_json::json;

#[test]
fn test_insert_datapoint() {
    let mut memtable = MemTable::new();

    let datapoint = DataPoint {
        label: Label("cpu_usage".into()),
        timestamp: Timestamp(1234567890),
        value: json!(0.87),
    };

    memtable.insert(&datapoint);

    let retrieved = memtable.get(&datapoint.label, &datapoint.timestamp);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), &datapoint);
}

#[test]
fn test_range() {
    let mut memtable = MemTable::new();

    let datapoint1 = DataPoint {
        label: Label("cpu_usage".into()),
        timestamp: Timestamp(1234567890),
        value: json!(0.87),
    };

    let datapoint2 = DataPoint {
        label: Label("cpu_usage".into()),
        timestamp: Timestamp(1234567891),
        value: json!(0.87),
    };

    memtable.insert(&datapoint1);
    memtable.insert(&datapoint2);

    let range = memtable.range(&datapoint1.label, &datapoint1.timestamp, &datapoint2.timestamp);
    assert_eq!(range.len(), 2);
    assert_eq!(range[0], &datapoint1);
    assert_eq!(range[1], &datapoint2);
}


