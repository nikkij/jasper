#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub fn as_nanos(&self) -> u64 {
        self.0
    }
}