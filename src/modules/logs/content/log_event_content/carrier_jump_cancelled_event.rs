use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJumpCancelled {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
}
