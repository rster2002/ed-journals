use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierNameChangeEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub callsign: String,
    pub name: String,
}
