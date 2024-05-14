use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierDecommissionEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub scrap_refund: u64,
    pub scrap_time: DateTime<Utc>,
}
