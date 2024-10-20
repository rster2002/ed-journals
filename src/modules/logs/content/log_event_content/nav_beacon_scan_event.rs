use serde::{Deserialize, Serialize};

/// Fired when the player scans a system's nav beacon.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NavBeaconScanEvent {
    /// The address of the system the nav beacon belongs to.
    pub system_address: u64,

    /// The number of bodies that are in the given system.
    #[serde(rename = "NumBodies")]
    pub number_of_bodies: u8,
}
