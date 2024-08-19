use serde::{Deserialize, Serialize};

use crate::modules::station::CarrierDockingAccess;

/// Fired when the fleet carrier owner changes the docking permissions for their fleet carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierDockingPermissionEvent {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The new docking access for the given carrier.
    pub docking_access: CarrierDockingAccess,

    /// Whether the carrier allows notorious commanders to dock.
    pub allow_notorious: bool,
}
