use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJumpCancelled {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
}
