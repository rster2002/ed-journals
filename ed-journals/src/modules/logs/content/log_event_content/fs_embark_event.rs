use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FCMaterialsEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub carrier_name: String,

    #[serde(rename = "CarrierID")]
    pub carrier_id: String,
}
