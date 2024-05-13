use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierShipPackEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub operation: CarrierShipPackEventOperation,
    pub pack_theme: String,
    pub pack_tier: u8,
    pub cost: Option<u64>,
    pub refund: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierShipPackEventOperation {
    BuyPack,
    SellPack,
    RestockPack,
}
