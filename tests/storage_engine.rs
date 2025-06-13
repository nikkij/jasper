use jasper::storage::engine::StorageEngine;
use jasper::types::{DataPoint, Label, Timestamp};
use serde_json::json;
use std::fs;
use std::path::Path;
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};


fn cleanup_test_file(path: &str) {
    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }
}

fn now() -> Timestamp {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    Timestamp(duration.as_nanos())
}

#[test]
fn test_write() -> Result<()> {
    let test_path = "test_storage_engine.wal";

    cleanup_test_file(test_path);

    let storage = StorageEngine::new(test_path);

    let dp1 = DataPoint {
        timestamp: now(),
        label: Label("npc.interaction".to_string()),
        value: json!({"text": "hello", "from": "Bob"}),
    };

    let dp2 = DataPoint {
        timestamp: now(),
        label: Label("npc.interaction".to_string()),
        value: json!({"text": "goodbye", "from": "Alice"}),
    };

    storage.write(&dp1)?;
    storage.write(&dp2)?;

    let contents = storage.read_all().unwrap();

    assert!(contents.iter().any(|dp| dp.value["text"] == "goodbye"));

    cleanup_test_file(test_path);

    Ok(())
}