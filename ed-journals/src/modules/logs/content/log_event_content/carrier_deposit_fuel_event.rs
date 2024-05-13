use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierDepositFuelEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub amount: u16,
    pub total: u16,
}
