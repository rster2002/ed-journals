use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DestinationStatus {
    pub system: u64,
    pub body: u8,
    pub name: String,
}
