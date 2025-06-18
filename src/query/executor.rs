use crate::storage::StorageEngine;
use crate::query::parser::QueryRequest;
use crate::types::{DataPoint, Label, Timestamp};

use serde_json::{json, Value};

pub fn execute_query(engine: &mut StorageEngine, query: QueryRequest) -> Value {
    match query {
        QueryRequest::Write { label, timestamp, value } => {
            let datapoint = DataPoint {
                label: Label(label),
                timestamp: Timestamp(timestamp),
                value,
            };

            if let Err(e) = engine.write(&datapoint) {
                return json!({
                    "status": "error",
                    "message": format!("Write failed: {}", e),
                });
            }

            json!({ "status": "ok" })
        }

        QueryRequest::Read { label, start, end } => {
            let label = Label(label);
            let start = Timestamp(start);
            let end = Timestamp(end);

            let results = engine.read_filtered(&label, &start, &end)
                .unwrap_or_else(|_| Vec::new());  // handle error gracefully
            println!("Attempting to send {} results", results.len());
            json!({
                "status": "ok",
                "results": results
            })
        }
    }
}
