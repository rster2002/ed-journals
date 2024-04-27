use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CarrierDepositFuelEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub amount: u16,
    pub total: u16,
}
