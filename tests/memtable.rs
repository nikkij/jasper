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
