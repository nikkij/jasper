use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum QueryRequest {
    #[serde(rename = "write")]
    Write {
        label: String,
        timestamp: u64,
        value: serde_json::Value,
    },
    #[serde(rename = "read")]
    Read {
        label: String,
        start: u64,
        end: u64,
    },
}

pub fn parse_query(json: &str) -> Result<QueryRequest, serde_json::Error> {
    serde_json::from_str(json)
}
