#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Timestamp(pub u128);

impl Timestamp {
    pub fn as_nanos(&self) -> u128 {
        self.0
    }
}