use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct BuyDronesEvent {
    #[serde(rename = "Type")]
    pub kind: BuyDronesEventType,
    pub count: u16,
    pub buy_price: u64,
    pub total_cost: u64,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum BuyDronesEventType {
    Drones,
}
