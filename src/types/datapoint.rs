use serde_json::Value as JsonValue;
use crate::types::{Timestamp, Label};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataPoint {
    pub label: Label,
    pub timestamp: Timestamp,
    pub value: JsonValue,
}