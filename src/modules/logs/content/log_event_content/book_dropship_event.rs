use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BookDropshipEvent {
    pub cost: u64,
    pub destination_system: String,
    pub destination_location: String,
}
