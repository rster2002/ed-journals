use serde::{Serialize, Deserialize};
use crate::models::station::carrier_docking_access::CarrierDockingAccess;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierDockingPermissionEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub docking_access: CarrierDockingAccess,
    pub allow_notorious: bool,
}
