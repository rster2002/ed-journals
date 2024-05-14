use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NavBeaconScanEvent {
    pub system_address: u64,

    #[serde(rename = "NumBodies")]
    pub number_of_bodies: u8,
}
